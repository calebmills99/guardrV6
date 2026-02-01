#!/usr/bin/env python3
"""
LeakOSINT Telegram Bot
Searches databases via LeakOSINT API
"""

import sys
import time
import logging
from random import randint

try:
    import requests
except ModuleNotFoundError:
    sys.exit("Missing requests. Run: pip install requests")

try:
    import telebot
    from telebot.types import InlineKeyboardMarkup, InlineKeyboardButton, CallbackQuery
except ModuleNotFoundError:
    sys.exit("Missing telebot. Run: pip install pyTelegramBotAPI")

# Configuration
CONFIG = {
    "api_url": "https://leakosintapi.com/",
    "bot_token": "8522376580:AAFJhi1vH4NtAIJ3lySodsVi526LfZ3eimE",
    "api_token": "7779445189:TMeEIJEh",
    "lang": "en",
    "limit": 300,
    "max_message_length": 3500,
}

# Setup logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[logging.StreamHandler()]
)
log = logging.getLogger(__name__)

# Cache for paginated reports
report_cache: dict[str, list[str]] = {}


def check_user_access(user_id: int) -> bool:
    """Check if user has access to the bot."""
    return True


def query_leakosint(query: str) -> dict | None:
    """Query the LeakOSINT API."""
    try:
        response = requests.post(
            CONFIG["api_url"],
            json={
                "token": CONFIG["api_token"],
                "request": query.split("\n")[0],
                "limit": CONFIG["limit"],
                "lang": CONFIG["lang"],
            },
            timeout=30
        )
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        log.error(f"API request failed: {e}")
        return None


def format_report(data: dict, query_id: int) -> list[str] | None:
    """Format API response into paginated messages."""
    if "Error code" in data:
        log.error(f"API error: {data['Error code']}")
        return None

    if "List" not in data:
        log.error("Invalid API response: missing 'List' key")
        return None

    pages = []
    for db_name, db_data in data["List"].items():
        lines = [f"<b>{db_name}</b>", ""]

        info_leak = db_data.get("InfoLeak", "")
        if info_leak:
            lines.append(f"{info_leak}\n")

        if db_name != "No results found":
            for record in db_data.get("Data", []):
                for key, value in record.items():
                    lines.append(f"<b>{key}</b>: {value}")
                lines.append("")

        text = "\n".join(lines)

        # Truncate if too long
        max_len = CONFIG["max_message_length"]
        if len(text) > max_len:
            text = text[:max_len]
            if "\n" in text[max_len-100:]:
                text = text.rsplit("\n", 1)[0]
            text += "\n\n<i>Message truncated...</i>"

        pages.append(text)

    report_cache[str(query_id)] = pages
    return pages


def generate_report(query: str, query_id: int) -> list[str] | None:
    """Generate a report for the given query."""
    data = query_leakosint(query)
    if data is None:
        return None
    return format_report(data, query_id)


def create_pagination_keyboard(query_id: int, page: int, total_pages: int) -> InlineKeyboardMarkup:
    """Create inline keyboard for pagination."""
    markup = InlineKeyboardMarkup()

    if total_pages <= 1:
        return markup

    page = page % total_pages

    markup.row_width = 3
    markup.add(
        InlineKeyboardButton("<<", callback_data=f"/page {query_id} {page - 1}"),
        InlineKeyboardButton(f"{page + 1}/{total_pages}", callback_data="noop"),
        InlineKeyboardButton(">>", callback_data=f"/page {query_id} {page + 1}")
    )
    return markup


def send_report_page(bot: telebot.TeleBot, chat_id: int, text: str, markup: InlineKeyboardMarkup) -> None:
    """Send a report page, falling back to plain text if HTML fails."""
    try:
        bot.send_message(chat_id, text, parse_mode="HTML", reply_markup=markup)
    except telebot.apihelper.ApiTelegramException:
        plain = text.replace("<b>", "").replace("</b>", "").replace("<i>", "").replace("</i>", "")
        bot.send_message(chat_id, plain, reply_markup=markup)


def edit_report_page(bot: telebot.TeleBot, chat_id: int, msg_id: int, text: str, markup: InlineKeyboardMarkup) -> None:
    """Edit a report page, falling back to plain text if HTML fails."""
    try:
        bot.edit_message_text(chat_id=chat_id, message_id=msg_id, text=text, parse_mode="HTML", reply_markup=markup)
    except telebot.apihelper.ApiTelegramException:
        plain = text.replace("<b>", "").replace("</b>", "").replace("<i>", "").replace("</i>", "")
        bot.edit_message_text(chat_id=chat_id, message_id=msg_id, text=plain, reply_markup=markup)


def main():
    """Main entry point."""
    log.info("Starting LeakOSINT Telegram Bot...")

    bot = telebot.TeleBot(CONFIG["bot_token"])

    @bot.message_handler(commands=["start", "help"])
    def handle_start(message):
        log.info(f"Received /start from {message.from_user.id}")
        bot.reply_to(
            message,
            "Welcome! Send me an email, phone, username, or other identifier to search leaked databases."
        )

    @bot.message_handler(func=lambda m: m.content_type == "text")
    def handle_query(message):
        user_id = message.from_user.id

        if not check_user_access(user_id):
            bot.send_message(message.chat.id, "Access denied.")
            return

        query = message.text.strip()
        if not query:
            return

        log.info(f"Query from {user_id}: {query[:50]}...")

        searching_msg = bot.reply_to(message, "Searching...")

        query_id = randint(0, 9999999)
        report = generate_report(query, query_id)

        try:
            bot.delete_message(message.chat.id, searching_msg.message_id)
        except Exception:
            pass

        if report is None or len(report) == 0:
            bot.reply_to(message, "No results found or service unavailable.")
            return

        markup = create_pagination_keyboard(query_id, 0, len(report))
        send_report_page(bot, message.chat.id, report[0], markup)

    @bot.callback_query_handler(func=lambda call: call.data.startswith("/page "))
    def handle_pagination(call: CallbackQuery):
        parts = call.data.split()
        if len(parts) != 3:
            return

        query_id = parts[1]
        try:
            page = int(parts[2])
        except ValueError:
            return

        if query_id not in report_cache:
            bot.answer_callback_query(call.id, "Results expired. Search again.")
            return

        report = report_cache[query_id]
        total_pages = len(report)
        page = page % total_pages

        markup = create_pagination_keyboard(int(query_id), page, total_pages)
        edit_report_page(bot, call.message.chat.id, call.message.message_id, report[page], markup)
        bot.answer_callback_query(call.id)

    @bot.callback_query_handler(func=lambda call: call.data == "noop")
    def handle_noop(call: CallbackQuery):
        bot.answer_callback_query(call.id)

    # Test bot connection
    try:
        bot_info = bot.get_me()
        log.info(f"Connected as @{bot_info.username} (ID: {bot_info.id})")
    except Exception as e:
        log.error(f"Failed to connect to Telegram: {e}")
        return

    log.info("Bot is running. Press Ctrl+C to stop.")
    log.info("Send a message to your bot on Telegram to test.")

    while True:
        try:
            bot.polling(none_stop=True, interval=0, timeout=30)
        except KeyboardInterrupt:
            log.info("Shutting down...")
            break
        except Exception as e:
            log.error(f"Polling error: {e}")
            time.sleep(5)


if __name__ == "__main__":
    main()
