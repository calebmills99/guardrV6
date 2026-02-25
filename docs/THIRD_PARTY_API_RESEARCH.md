# Third-Party API Research: Supplements, Enhancements & Substitutes

> Research conducted Feb 2026. Guardr currently uses HIBP, Intelligence X, Leak Lookup, Google Gemini, and has config stubs for DeHashed, Serper, OpenAI, Anthropic, Mistral, xAI, ScrapingBee, Firecrawl, Exa, and Tavily.

---

## 1. Breach Data & Credential Monitoring

### Currently Used
| API | Status | What it does |
|-----|--------|-------------|
| **Have I Been Pwned (HIBP)** | Active (Python) | Email breach lookup against known breaches |
| **Leak Lookup** | Active (Python) | Additional breach data via email search |
| **Intelligence X** | Active (Python) | Dark web pastes, leaks, dumpster buckets |
| **DeHashed** | Config stub only | Breach search (not implemented in Rust) |

### Recommended Additions

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **SpyCloud** | Substitute/Enhance | Enterprise-grade breach + infostealer log recovery. Includes session token & cookie monitoring, goes beyond email-only lookups. Great for detecting ATO (account takeover) risk. | Enterprise pricing (custom) | High |
| **BreachDirectory** | Supplement | Supports search by domain, email, username, IP. Fills gaps HIBP doesn't cover (username-based search). | Basic: 500 req/mo free, Unlimited plans available | Medium |
| **Hudson Rock** | Enhance | Specializes in infostealer malware logs — detects if a person's device was compromised, not just if their email appeared in a breach. Unique signal for Guardr's risk scoring. | Enterprise (custom) | Medium |
| **Breachsense** | Substitute | Dark web breach alerts with stealer-log access and session token/cookie monitoring. Real-time alerting pipeline. | Enterprise (custom) | Low |
| **SOCRadar** | Enhance | Digital risk protection + dark web monitoring. Monitors for impersonation, brand abuse, and leaked credentials in a single platform. | Enterprise (custom) | Low |

### Recommendation
Keep HIBP as the primary (most trusted, well-known brand). Add **BreachDirectory** for username/domain search capabilities HIBP lacks. Consider **SpyCloud** or **Hudson Rock** for infostealer-log intelligence — this is a rapidly growing threat vector that purely breach-based APIs miss.

---

## 2. Dark Web & Threat Intelligence

### Currently Used
| API | Status | What it does |
|-----|--------|-------------|
| **Intelligence X** | Active (Python) | Searches dark web, pastes, leaks |

### Recommended Additions

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **Blackhole Feeds** | Supplement | Real-time threat intel from underground markets, ransomware groups, fraud forums. 5.2M daily intelligence findings, 99.7% accuracy. SIEM/SOAR integration ready. | Enterprise (custom) | Medium |
| **Onion Sentinel (Apify)** | Supplement | Automated .onion site discovery & keyword monitoring via Tor. Good for monitoring if a user's data surfaces on dark web marketplaces. | Pay-per-use via Apify | Low |
| **Shodan** | Enhance | Internet-wide device/service scanner. Could detect exposed personal devices, open cameras, or IoT risks associated with a target's IP/network. | Free tier available, Membership $59/mo | Low |

### Recommendation
Intelligence X is strong for dark web. **Blackhole Feeds** would be the best complement for real-time underground market monitoring (romance scam rings, fraud-as-a-service, stolen identity sales).

---

## 3. AI / ML for Risk Analysis

### Currently Used
| API | Status | What it does |
|-----|--------|-------------|
| **Google Gemini (1.5 Flash)** | Active (Python) | AI risk assessment, NLP analysis, dating profile analysis |
| **OpenAI** | Config stub | Not implemented |
| **Anthropic** | Config stub | Not implemented |
| **Mistral (via GitHub Models)** | Active (Python) | Lady Guardr coding assistant |
| **xAI** | Config stub | Not implemented |

### Recommended Additions

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **OpenAI Moderation API** | Supplement | **Free.** Detects harassment, hate speech, violence, self-harm in conversations. Perfect for Guardr's conversation analysis — add as a pre-filter before Gemini's deeper analysis. | Free (unlimited) | **High** |
| **ToxTex** | Supplement | Multi-model toxicity detection (combines OpenAI, Gemini, Claude, Grok). Analyzes meaning and intent, not just keywords. Great for detecting manipulation tactics. | Free: 1K tokens, Starter: $5/100K tokens | Medium |
| **Anthropic Claude** | Substitute | Strong at nuanced conversation analysis and safety-oriented reasoning. Could be used as a fallback or for second-opinion risk assessment when Gemini's output is uncertain. | API pay-per-token | Medium |
| **ToxicityAPI.com** | Supplement | Privacy-first (zero data retention) toxicity detection. Detects modern slang, memes, internet-specific patterns. Good for younger demographics. | Basic: $8.99/mo (50K req) | Low |
| **Copyleaks Text Moderation** | Supplement | Context-aware detection across 10 categories with pinpointed highlighting. Good for generating detailed conversation red-flag reports. | Enterprise pricing | Low |

### Recommendation
The **OpenAI Moderation API is free** and should be added immediately as a first-pass filter for conversation analysis. It catches harassment, threats, and self-harm content that Guardr's current keyword-based detection (`detect_pressure_indicators`) could miss. Keep Gemini for the deeper risk narrative and reasoning layer.

---

## 4. Identity Verification & Catfish Detection

### Currently Used
None — Guardr currently does mock OSINT analysis in the `/check` endpoint.

### Recommended Additions (New Category)

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **FaceCheck.id** | New | Reverse image face search API. User uploads a profile photo, API searches across social media, news, mugshots to find where that face appears. Core catfish detection tool. | $0.10/search (3 credits) | **High** |
| **FaceFinderAi** | New | 212M+ indexed face vectors, ~350ms search. Privacy-focused with self-hosted option. Good for OSINT attribution — "is this person who they say they are?" | Enterprise (custom) | High |
| **Reality Defender (RealAPI)** | New | Deepfake detection for photos and videos. Detects AI-generated profile pictures (increasingly common in romance scams). Returns manipulation probability + explainable indicators. | Free: 50 scans/mo, $399/mo for 1K scans | **High** |
| **Veriff** | New | Enterprise identity verification used by dating platforms. Biometric selfie vs. profile photo match, liveness detection, ID document verification across 230+ countries. | Enterprise (custom) | Medium |
| **TruthScan** | New | AI-powered fake photo detection. Claims 99%+ accuracy for AI-generated images, stolen photos, and fraudulent profiles. Purpose-built for dating platforms. | Contact for pricing | Medium |
| **TrueMatch** | New | Real-time verification APIs with trust indicators. Reports 3x more matches for verified profiles. Dating-specific verification. | Contact for pricing | Low |

### Recommendation
This is Guardr's **biggest gap**. The current `/check` endpoint is a mock. Adding **FaceCheck.id** (reverse image search) + **Reality Defender** (deepfake detection) would give Guardr real, differentiated catfish detection capabilities that justify the subscription pricing. These two together cover "is this a stolen photo?" and "is this an AI-generated photo?" — the two primary catfishing vectors.

---

## 5. People Search & Digital Footprint

### Currently Used
None in the Rust API. Python scripts reference some manual OSINT techniques.

### Recommended Additions (New Category)

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **Minerva OSINT API** | New | Email OSINT across 70+ platforms. Checks if an email is registered on social media, dating sites, etc. Verifies "does this person have a real digital presence?" | €0.10/request, €49.99/500 credits | **High** |
| **1Lookup** | New | Reverse phone + email lookup. 95% accuracy, 1000+ data sources, 10B+ records. Identifies phone owner, carrier, location history. | Pay-per-use | Medium |
| **Whitepages Pro API** | New | 350M identity records, 454M emails. People search by name, phone, address. Validates "does this person actually exist at the location they claim?" | 14-day free trial, then subscription | Medium |
| **OsintCat API** | New | All-in-one OSINT: email, phone, IP, username, and breach lookups in a single API. Reduces integration complexity. | Free, Premium, Enterprise tiers | Low |

### Recommendation
**Minerva OSINT** is the best fit — checking email registration across 70+ platforms directly validates whether a dating profile's email has a legitimate digital footprint. This is a strong catfish signal: real people have accounts on multiple platforms; fake profiles typically don't.

---

## 6. Web Scraping & Social Media Data

### Currently Used
| API | Status | What it does |
|-----|--------|-------------|
| **ScrapingBee** | Config stub | General web scraping |
| **Firecrawl** | Config stub | Web scraping / crawling |

### Recommended Additions

| API | Category | Why | Pricing | Priority |
|-----|----------|-----|---------|----------|
| **SociaVault** | Substitute | Specialized social media scraping for 25+ platforms (TikTok, Instagram, LinkedIn, Twitter, etc.). Simple REST API, pay-as-you-go, 99.9% uptime. Better than general scrapers for profile verification. | $29/6,000 credits | Medium |
| **ScrapeCreators** | Substitute | Social media data extraction. Reported 10x cheaper than competitors with lower failure rates. | Pay-per-use | Medium |
| **Apify Social Media Scraper** | Supplement | Broad actor marketplace for scraping specific platforms. Good for one-off deep-dives on specific profiles. | Pay-per-use | Low |

### Recommendation
If Guardr needs to cross-reference dating profiles against social media, **SociaVault** is the most pragmatic choice — purpose-built for social media with a simple REST API. General scrapers like ScrapingBee add complexity for this use case.

---

## 7. Search APIs

### Currently Used
| API | Status | What it does |
|-----|--------|-------------|
| **Serper** | Config stub | Google search API |
| **Exa** | Config stub | AI-powered search |
| **Tavily** | Config stub | AI-powered search |

### Recommendation
These are already well-covered in the config. **Serper** is the best for raw Google results (cheapest, fastest). **Tavily** is best for AI-augmented search summaries. No urgent additions needed — activate the existing stubs.

---

## Priority Implementation Roadmap

### Phase 1 — Quick Wins (free or cheap, high impact)
1. **OpenAI Moderation API** — Free. Add to conversation analysis for harassment/threat detection.
2. **FaceCheck.id** — $0.10/search. Reverse image search for catfish detection.
3. **BreachDirectory** — Free tier. Username/domain breach search.

### Phase 2 — Core Differentiation (medium cost, high value)
4. **Reality Defender** — Deepfake/AI-generated image detection. Free tier for 50 scans/mo.
5. **Minerva OSINT** — Email digital footprint verification across 70+ platforms.
6. **FaceFinderAi** — Second reverse image search provider for redundancy.

### Phase 3 — Enterprise Features (higher cost, premium tier)
7. **SpyCloud** — Infostealer log monitoring for enterprise customers.
8. **Blackhole Feeds** — Real-time dark web marketplace monitoring.
9. **SociaVault** — Social media profile scraping for cross-referencing.
10. **Veriff** — Full identity document + biometric verification.

---

## Architecture Consideration

The current `OsintConfig` struct in `src/config.rs` already follows a good pattern — all API keys are `Option<String>`. New providers should follow the same pattern:

```
[osint]
facecheck_api_key = ""
reality_defender_api_key = ""
minerva_api_key = ""
breachdirectory_api_key = ""
sociavault_api_key = ""
```

Each new provider should be implemented as an independent module that gracefully degrades when the API key is not set, consistent with the existing architecture.
