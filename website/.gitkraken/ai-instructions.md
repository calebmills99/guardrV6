# AI Instructions for GitKraken - Guardr Frontend

## Project Context
Guardr - AI-powered dating safety platform for LGBTQ+ community (Next.js 15 frontend).

## Commit Message Guidelines

### Format
```
<type>: <subject>

<body (optional)>
```

### Types
- `feat:` New feature (e.g., "feat: add live demo form to landing page")
- `fix:` Bug fix (e.g., "fix: API timeout handling for 120s OSINT calls")
- `style:` Design/styling changes (e.g., "style: update neon glow effects")
- `refactor:` Code restructuring (e.g., "refactor: extract form validation logic")
- `perf:` Performance improvements (e.g., "perf: optimize mobile viewport rendering")
- `docs:` Documentation (e.g., "docs: update deployment instructions")
- `chore:` Maintenance (e.g., "chore: update dependencies")

### Subject Rules
- Use imperative mood: "add" not "added" or "adds"
- No period at the end
- Capitalize first letter
- Max 50 characters
- Be specific: "fix: handle Master Caleb Stewart name collision" not "fix: bug"

### Examples
```
feat: add location field to safety check form

Helps with name disambiguation when common names (like "John Smith")
are searched. Optional field improves OSINT accuracy.

---

style: retheme marketing site with nightlife palette

Replaced light theme with dark neon theme using purple/pink/cyan
gradient system. Improved mobile contrast ratios.

---

fix: increase API timeout to 120 seconds

OSINT calls take 60-120s for real investigation. Previous 30s
timeout caused false failures. Added loading messaging.
```

## Code Review Focus Areas

1. **Mobile-First Design**
   - Check responsive Tailwind classes (sm:, md:, lg:)
   - Verify 375px viewport works
   - Test touch targets (min 44px)

2. **Theme Consistency**
   - Use CSS variables: `var(--surface-*)`, `var(--primary-*)`
   - No hardcoded colors
   - Neon effects: `shadow-glow-*` classes

3. **API Integration**
   - Timeout: 120s minimum
   - Loading states with "2-minute" messaging
   - Error handling for network failures
   - NEXT_PUBLIC_API_URL from env

4. **Performance**
   - Server components by default
   - 'use client' only when needed
   - Image optimization with next/image
   - No unnecessary useState/useEffect

5. **Brand Voice**
   - Empowering, not alarmist
   - LGBTQ+ focused messaging
   - "You are an adult. You decide." tone

## Code Explanation Context

When explaining code:
- Mention if it's App Router (Next.js 15) pattern
- Highlight Tailwind theme variables
- Note mobile-first responsive patterns
- Call out LGBTQ+ safety-focused design decisions
- Reference 2-minute OSINT scan UX considerations

## Branch Naming
- `feat/description` - New features
- `fix/description` - Bug fixes
- `style/description` - UI/design changes
- `refactor/description` - Code improvements
- `docs/description` - Documentation

Examples:
- `feat/add-age-filter-field`
- `fix/api-timeout-handling`
- `style/mobile-form-spacing`

## Don't Reference
- Vercel (migrated to DigitalOcean)
- "About" page with Daniel Obregón (removed)
- Old light theme (replaced with dark nightlife)
- CSS modules (use Tailwind utilities)

## Key Concepts to Know
- **PERA Cycle**: Plan → Execute → Review → Adjust (backend AI loop)
- **OSINT**: Open Source Intelligence (takes 60-120s)
- **Nightlife Palette**: Dark theme with neon purple/pink/cyan
- **$6.99 Pricing**: Reference to 69% LGBTQ+ harassment rate
- **Mobile-First**: "It has to be a mobile app day one"
