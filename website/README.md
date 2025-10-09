This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load fonts.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out the Next.js GitHub repository - your feedback and contributions are welcome!

## Deployment

### DigitalOcean App Platform

Guardr is deployed on DigitalOcean App Platform with automatic SSL and custom domain support.

#### Frontend (Web Service)
- **Build Command:** `npm run build`
- **Run Command:** `npm start`
- **Port:** 3000
- **Cost:** $5/month
- **Environment Variables:**
  - `NEXT_PUBLIC_API_URL=${guardr-api.PRIVATE_URL}` (internal networking)

#### Backend (Python Service)
- **Build Command:** `pip install -r requirements.txt`
- **Run Command:** `gunicorn -w 2 -b 0.0.0.0:8080 --timeout 120 guardr_api:app`
- **Port:** 8080
- **Cost:** $5/month
- **Timeout:** 120 seconds (required for OSINT operations)

#### Configuration
- **Domain:** guardr.app
- **SSL:** Automatic (Let's Encrypt)
- **Total Cost:** $10/month
- **Auto-deploy:** Push to `main` branch triggers deployment

#### Internal Networking
The frontend connects to the backend using DigitalOcean's internal networking:
```
NEXT_PUBLIC_API_URL=${guardr-api.PRIVATE_URL}
```

This ensures secure, low-latency communication between services without exposing the backend to the public internet.
