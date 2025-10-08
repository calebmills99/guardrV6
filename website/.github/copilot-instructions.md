# Copilot Instructions for Guardr Frontend

## Project Context
**Guardr** marketing site and demo - AI-powered dating safety for LGBTQ+ community.

### Tech Stack
- **Framework:** Next.js 15.4.6 (App Router)
- **React:** 19.x
- **Styling:** Tailwind CSS (nightlife/neon theme)
- **Deployment:** DigitalOcean App Platform (migrated from Vercel)
- **Backend API:** Flask on localhost:5000 (prod: DO droplet)

## Critical Rules

### Environment
- `NEXT_PUBLIC_API_URL` must be set in `.env.local`
- Dev: `http://localhost:5000`
- Prod: Will be DO droplet URL
- **DO NOT** commit `.env.local`

### Design System

**Theme: Nightlife Palette**
- Dark backgrounds: `bg-[color:var(--surface-100)]` through `var(--surface-400)`
- Primary: Purple/magenta (`bg-primary-500`)
- Secondary: Pink (`bg-secondary-500`)
- Accent: Teal/cyan (`bg-accent-500`)
- Neon effects: `shadow-glow-primary`, `shadow-glow-accent`

**Typography:**
- Headings: `text-white`, bold
- Body: `text-white/85` (85% opacity)
- Small text: `text-white/80`

**Components:**
- Cards: `glass` effect with `backdrop-blur`
- Buttons: `btn-hover-lift` for 3D lift effect
- Borders: `border-white/10` or `border-white/5`

### Code Patterns

**Form Handling:**
```typescript
// Demo form on landing page
interface FormData {
  name: string;        // REQUIRED
  location?: string;   // Optional but helps disambiguation
}

// API call structure
const response = await fetch(`${API_URL}/api/check`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ name, location })
});

// Response structure
{
  risk_level: "HIGH" | "MEDIUM" | "LOW",
  risk_score: 0-100,
  person_verification: {...},
  safety_tips: string[]
}
```

**Component Structure:**
```tsx
// Use server components by default
// Mark with 'use client' ONLY when needed (forms, state)

// Shared components in src/components/
// Page components in src/app/[route]/page.tsx
```

### File Organization
```
src/
├── app/              # Next.js 15 App Router
│   ├── layout.tsx    # Root layout with theme
│   ├── page.tsx      # Landing page with demo form
│   ├── how-it-works/
│   ├── pricing/
│   └── safety-tips/
├── components/       # Reusable components
│   ├── ui/          # Button, Card, Input, etc.
│   └── layout/      # Header, Footer
└── lib/             # Utilities
    └── utils.ts     # cn() helper, etc.
```

### Common Pitfalls

1. **API Timeout:** OSINT calls take 60-120s
   - Set fetch timeout to 120000ms minimum
   - Show loading state with 2-minute messaging

2. **Mobile First:** Design for mobile FIRST
   - Use responsive Tailwind classes: `sm:`, `md:`, `lg:`
   - Test on mobile viewport (375px)

3. **Removed Content:**
   - NO "About" page with Daniel Obregón placeholder (removed)
   - Don't recreate it without asking

4. **API URL:**
   - Use `process.env.NEXT_PUBLIC_API_URL`
   - Falls back to `http://localhost:5000` if not set

5. **Styling:**
   - Use Tailwind utilities, NOT CSS modules
   - Use theme variables, NOT hardcoded colors
   - Neon effects via utility classes, not inline styles

### Brand Voice
- **Empowering, not alarmist:** "You are an adult. You decide."
- **LGBTQ+ focused** but universal safety
- **2-minute scan is a FEATURE** (captive audience for engagement)
- **Mobile-first:** "It has to be a mobile app day one"
- **Pricing:** $6.99 = 69% LGBTQ+ harassment rate reminder

### Safety Tips Categories
Rotate these 4 categories:
1. `smart_habits` - Practical safety advice
2. `friendly_reminders` - Gentle prompts
3. `did_you_know` - Stats and education
4. `you_decide` - Empowering choice

### Testing
```bash
npm run dev        # Dev server on 3001/3002
npm run build      # Production build
npm run lint       # ESLint check
```

### Deployment
- **Platform:** DigitalOcean App Platform
- **Build Command:** `npm run build`
- **Output:** `.next` directory
- **Node Version:** 22.x
- **Auto-deploy:** GitHub push to `main` branch

### When Making Changes
1. Check localhost:3001 first
2. API must be running on localhost:5000
3. Test mobile viewport (Chrome DevTools)
4. Verify dark theme consistency
5. Commit with clear messages for auto-deploy
