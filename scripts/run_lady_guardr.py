#!/usr/bin/env python3
"""
🎭 Lady Guardr - Fierce Coding Assistant
=========================================
A drag queen persona AI assistant for the Guardr repository.
Uses Codestral via GitHub Models.

Usage:
    python scripts/run_lady_guardr.py
    python scripts/run_lady_guardr.py "Your question here"
    python scripts/run_lady_guardr.py --interactive

Requirements:
    pip install mistralai>=1.0.0

Environment:
    GITHUB_TOKEN - Your GitHub Personal Access Token
                   Get one at: https://github.com/settings/tokens
"""

import os
import sys
from mistralai import Mistral, UserMessage, SystemMessage

# ═══════════════════════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════

SYSTEM_PROMPT = """
You are Lady Guardr: a fierce, witty drag queen persona and a meticulous coding assistant for the Guardr repository (AI-powered dating safety for LGBTQ+ users). You prioritize safety, privacy, inclusivity, and technical accuracy. Speak with occasional playful drag flair, but keep responses professional and concise.

Project context:
- Rust backend (Axum/SQLx/Tokio), SQLite/Redis
- Next.js 14 frontend (TypeScript/Tailwind)
- Focus on dating safety, OSINT, breach monitoring, AI risk analysis
- Commands: `cargo build`, `cargo test`, `cargo clippy`, `npm run dev` in /website

Behavioral rules:
- Never expose secrets or request credentials.
- Prefer minimal, reversible changes.
- If you change code (not docs/comments), run tests and linters.
- Explain reasoning and list files touched.
""".strip()

MODEL = "mistral-ai/Codestral-2501"
SERVER_URL = "https://models.github.ai/inference"

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

def get_client():
    """Initialize the Mistral client with GitHub Models."""
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        print("❌ GITHUB_TOKEN environment variable not set!")
        print()
        print("To fix this:")
        print("  1. Get a token at: https://github.com/settings/tokens")
        print("  2. Set it with: $env:GITHUB_TOKEN = 'ghp_your_token_here'")
        sys.exit(1)
    
    return Mistral(api_key=token, server_url=SERVER_URL)


def chat(client, user_message: str, conversation: list = None) -> str:
    """Send a message to Lady Guardr and get a response."""
    if conversation is None:
        conversation = []
    
    messages = [SystemMessage(SYSTEM_PROMPT)] + conversation + [UserMessage(user_message)]
    
    response = client.chat(
        model=MODEL,
        messages=messages,
        temperature=0.7,
        max_tokens=1500,
        top_p=1.0,
    )
    
    return response.choices[0].message.content


def interactive_mode(client):
    """Run Lady Guardr in interactive chat mode."""
    print()
    print("💄 ═══════════════════════════════════════════════════════════════════")
    print("   🎭 LADY GUARDR - Interactive Mode")
    print("   Type your questions, darling. Type 'exit' or 'quit' to leave.")
    print("═══════════════════════════════════════════════════════════════════ 💄")
    print()
    
    conversation = []
    
    while True:
        try:
            user_input = input("You: ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\n\n✨ Sashay away, darling! Stay safe out there! ✨")
            break
        
        if not user_input:
            continue
        
        if user_input.lower() in ("exit", "quit", "bye", "q"):
            print("\n✨ Sashay away, darling! Stay safe out there! ✨")
            break
        
        try:
            response = chat(client, user_input, conversation)
            print(f"\n🎭 Lady Guardr: {response}\n")
            
            # Keep conversation history (last 10 exchanges)
            conversation.append(UserMessage(user_input))
            conversation.append(SystemMessage(response))
            if len(conversation) > 20:
                conversation = conversation[-20:]
                
        except Exception as e:
            print(f"\n❌ Error: {e}\n")


def single_query(client, question: str):
    """Ask Lady Guardr a single question."""
    print()
    print("💄 Lady Guardr says:")
    print("─" * 60)
    response = chat(client, question)
    print(response)
    print("─" * 60)
    print()


def main():
    client = get_client()
    
    if len(sys.argv) > 1:
        arg = sys.argv[1]
        
        if arg in ("--interactive", "-i"):
            interactive_mode(client)
        elif arg in ("--help", "-h"):
            print(__doc__)
        else:
            # Treat all args as the question
            question = " ".join(sys.argv[1:])
            single_query(client, question)
    else:
        # No args - show intro and go interactive
        intro = chat(client, "Introduce yourself briefly and ask how you can help.")
        print()
        print(f"🎭 {intro}")
        print()
        interactive_mode(client)


if __name__ == "__main__":
    main()
