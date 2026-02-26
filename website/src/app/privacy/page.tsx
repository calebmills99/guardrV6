'use client';

import {
  Shield,
  Lock,
  Eye,
  Clock,
  Database,
  Trash2,
  Users,
  Mail,
  Cookie,
  FileText,
  ArrowRight,
  CheckCircle,
  AlertTriangle
} from 'lucide-react';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';
import Button from '@/components/ui/Button';
import Link from 'next/link';

const highlights = [
  {
    icon: Lock,
    title: 'We never sell your data',
    description: 'Your information is never shared with advertisers or third-party data brokers.',
    color: 'text-primary-400',
  },
  {
    icon: Database,
    title: 'OSINT data is hashed',
    description: 'All queried data is hashed on ingestion—raw PII is never stored in our systems.',
    color: 'text-secondary-400',
  },
  {
    icon: Clock,
    title: 'Reports auto-expire',
    description: 'Safety reports automatically expire after 30 days. No indefinite data retention.',
    color: 'text-accent-400',
  },
];

const sections = [
  {
    number: '1',
    title: 'Information We Collect',
    icon: Database,
    content: [
      {
        subtitle: 'Account Information',
        text: 'When you create an account, we collect your email address, display name, and hashed password. We use OAuth where available to minimize credential storage.',
      },
      {
        subtitle: 'Usage Data',
        text: 'We collect anonymized analytics on feature usage, session duration, and error reports to improve the platform. This data is never tied to your identity.',
      },
      {
        subtitle: 'OSINT Queries',
        text: 'When you run a safety check, we query publicly available data sources. The input data (names, usernames, photos) is immediately hashed. We analyze the results to generate a safety score but do not store the raw query inputs.',
      },
    ],
  },
  {
    number: '2',
    title: 'How We Use Your Information',
    icon: Eye,
    content: [
      {
        subtitle: 'Service Delivery',
        text: 'To provide AI-powered identity verification, risk assessment, and safety reports as part of the core Guardr service.',
      },
      {
        subtitle: 'Safety Analysis',
        text: 'To analyze patterns across public data sources and generate risk scores. This analysis is ephemeral—it happens in real-time and results are stored only as hashed summaries.',
      },
      {
        subtitle: 'Platform Improvement',
        text: 'Aggregated, anonymized usage statistics help us improve our AI models and user experience. Individual data is never used for this purpose.',
      },
    ],
  },
  {
    number: '3',
    title: 'OSINT Data Handling',
    icon: Shield,
    content: [
      {
        subtitle: 'Hashed Inputs',
        text: 'All inputs to our OSINT analysis pipeline (names, usernames, email addresses, phone numbers) are hashed using SHA-256 before storage. The original values are discarded after analysis.',
      },
      {
        subtitle: 'No Raw PII Storage',
        text: 'We never store raw personally identifiable information from OSINT queries. Safety reports contain risk assessments and scores, not the underlying personal data.',
      },
      {
        subtitle: 'Ephemeral Analysis',
        text: 'OSINT data is processed in real-time memory and is not written to persistent storage. Only the resulting safety score and categorical risk indicators are retained.',
      },
    ],
  },
  {
    number: '4',
    title: 'Data Retention',
    icon: Trash2,
    content: [
      {
        subtitle: 'Safety Reports',
        text: 'Safety reports automatically expire and are permanently deleted after 30 days from generation. This cannot be extended.',
      },
      {
        subtitle: 'Account Data',
        text: 'Your account information is retained as long as your account is active. Upon account deletion, all associated data is permanently removed within 30 days.',
      },
      {
        subtitle: 'Analytics',
        text: 'Anonymized, aggregated analytics data may be retained indefinitely as it cannot be traced back to individual users.',
      },
    ],
  },
  {
    number: '5',
    title: 'Third-Party Services',
    icon: Users,
    content: [
      {
        subtitle: 'Data Breach Checking',
        text: 'We use secure API integrations to check for data breaches associated with email addresses. These queries use k-anonymity protocols—we never send full email addresses to third parties.',
      },
      {
        subtitle: 'No Marketing Sharing',
        text: 'We do not share, sell, or provide your data to any third party for marketing, advertising, or data brokerage purposes. Ever.',
      },
      {
        subtitle: 'Infrastructure',
        text: 'Our infrastructure providers (hosting, CDN) process data only as needed to deliver the service and are bound by strict data processing agreements.',
      },
    ],
  },
  {
    number: '6',
    title: 'Your Rights',
    icon: CheckCircle,
    content: [
      {
        subtitle: 'Access & Portability',
        text: 'You can request a copy of all data we hold about you at any time. We will provide it in a machine-readable format within 30 days.',
      },
      {
        subtitle: 'Correction',
        text: 'You can update or correct your personal information through your account settings or by contacting us.',
      },
      {
        subtitle: 'Deletion',
        text: 'You can request complete deletion of your account and all associated data. Under GDPR and CCPA, this is your right and we honor it without question.',
      },
    ],
  },
  {
    number: '7',
    title: 'Cookies & Tracking',
    icon: Cookie,
    content: [
      {
        subtitle: 'Minimal Cookies',
        text: 'We use only essential cookies for authentication and session management. No third-party tracking cookies are used.',
      },
      {
        subtitle: 'Analytics',
        text: 'We use privacy-respecting analytics that do not track individual users across sites and do not use cookies for analytics purposes.',
      },
    ],
  },
  {
    number: '8',
    title: 'Security Measures',
    icon: Lock,
    content: [
      {
        subtitle: 'Encryption',
        text: 'All data is encrypted in transit (TLS 1.3) and at rest (AES-256). API keys and credentials are stored in secure vaults, never in code.',
      },
      {
        subtitle: 'Infrastructure',
        text: 'We use secure, SOC 2 compliant infrastructure with regular security audits, penetration testing, and vulnerability scanning.',
      },
    ],
  },
  {
    number: '9',
    title: "Children's Privacy",
    icon: AlertTriangle,
    content: [
      {
        subtitle: 'Age Requirement',
        text: 'Guardr is intended for users aged 18 and older. We do not knowingly collect information from anyone under 18. If we discover we have collected data from a minor, we will delete it immediately.',
      },
    ],
  },
  {
    number: '10',
    title: 'Changes to This Policy',
    icon: FileText,
    content: [
      {
        subtitle: 'Notification',
        text: 'We will notify you of material changes to this policy via email and/or a prominent notice on our platform at least 30 days before changes take effect.',
      },
    ],
  },
  {
    number: '11',
    title: 'Contact Us',
    icon: Mail,
    content: [
      {
        subtitle: 'Questions?',
        text: 'If you have questions about this privacy policy or our data practices, contact us at support@guardr.app or visit our contact page.',
      },
    ],
  },
];

export default function Privacy() {
  return (
    <div className="min-h-screen">
      {/* Hero */}
      <section className="relative py-24 lg:py-32 overflow-hidden bg-hero-night">
        <div className="absolute inset-0 opacity-30 bg-grid-pattern" />
        <div className="absolute top-20 left-1/4 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl" />

        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <Badge variant="primary" pill className="mb-6 text-sm">
            <Lock className="w-4 h-4 mr-1 inline" />
            Privacy Policy
          </Badge>

          <h1 className="text-5xl lg:text-7xl font-bold mb-6">
            <span className="gradient-text">Your Privacy</span>
            <br />
            Matters
          </h1>

          <p className="text-lg text-white/60">
            Last updated: January 2026
          </p>
        </div>
      </section>

      {/* Highlights */}
      <section className="py-16 lg:py-20">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid sm:grid-cols-3 gap-6 max-w-4xl mx-auto">
            {highlights.map((item) => (
              <Card key={item.title} variant="glass" padding="lg" className="text-center">
                <div className="w-14 h-14 rounded-xl bg-surface-300 flex items-center justify-center mx-auto mb-4">
                  <item.icon className={`w-7 h-7 ${item.color}`} />
                </div>
                <h3 className="text-lg font-semibold mb-2">{item.title}</h3>
                <p className="text-sm text-white/60">{item.description}</p>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Policy Sections */}
      <section className="py-16 lg:py-24 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-3xl mx-auto space-y-8">
            {sections.map((section) => (
              <Card key={section.number} variant="bordered" padding="lg">
                <div className="flex items-center gap-4 mb-6">
                  <div className="w-10 h-10 rounded-lg bg-primary-500/20 flex items-center justify-center flex-shrink-0">
                    <span className="text-primary-400 font-bold text-sm">{section.number}</span>
                  </div>
                  <div className="flex items-center gap-3">
                    <section.icon className="w-5 h-5 text-primary-400" />
                    <h2 className="text-xl font-semibold">{section.title}</h2>
                  </div>
                </div>

                <div className="space-y-4 pl-14">
                  {section.content.map((item) => (
                    <div key={item.subtitle}>
                      <h3 className="text-sm font-semibold text-white/90 mb-1">
                        {item.subtitle}
                      </h3>
                      <p className="text-sm text-white/60 leading-relaxed">{item.text}</p>
                    </div>
                  ))}
                </div>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl lg:text-4xl font-bold mb-4">Have Questions?</h2>
          <p className="text-lg text-white/70 mb-8 max-w-2xl mx-auto">
            We&apos;re committed to transparency. If anything about our data practices
            is unclear, reach out.
          </p>
          <Link href="/contact">
            <Button
              variant="primary"
              size="lg"
              icon={ArrowRight}
              iconPosition="right"
            >
              Contact Us
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
