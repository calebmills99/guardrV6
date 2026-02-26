'use client';

import {
  Shield,
  Heart,
  Lock,
  Users,
  Eye,
  ArrowRight,
  CheckCircle,
  Fingerprint,
  Scale,
  Sparkles,
  Globe,
  Search,
  Zap,
  Target,
  Star
} from 'lucide-react';
import Image from 'next/image';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';
import Link from 'next/link';

const differentiators = [
  {
    icon: Fingerprint,
    title: 'AI-Powered Verification',
    description: 'OSINT-backed identity checks analyze digital footprints, reverse-image search photos, and cross-reference public data to surface red flags before you meet.',
    color: 'text-primary-400',
    glow: 'shadow-glow-primary',
  },
  {
    icon: Lock,
    title: 'Privacy-First Design',
    description: 'All queried data is hashed on ingestion. Reports auto-expire after 30 days. We never store raw PII or sell your data to anyone, ever.',
    color: 'text-secondary-400',
    glow: 'shadow-glow-secondary',
  },
  {
    icon: Users,
    title: 'LGBTQ+ Centered',
    description: 'Built by and for marginalized communities who face disproportionate risk in online dating. Every design decision starts with queer safety.',
    color: 'text-accent-400',
    glow: 'shadow-glow-accent',
  },
  {
    icon: Scale,
    title: 'Ethical & Legal',
    description: 'Fully GDPR and CCPA compliant. We use only publicly available data, with transparent consent flows and zero backdoors.',
    color: 'text-success-500',
    glow: 'shadow-glow-neutral',
  },
];

const coreValues = [
  { icon: Shield, label: 'Safety', description: 'Protection is not optional—it\'s foundational.' },
  { icon: Lock, label: 'Privacy', description: 'Your data belongs to you. Period.' },
  { icon: Heart, label: 'Inclusion', description: 'Every identity deserves to date without fear.' },
  { icon: Eye, label: 'Transparency', description: 'No black boxes. You see what we see.' },
];

export default function About() {
  return (
    <div className="min-h-screen">
      {/* Hero */}
      <section className="relative py-24 lg:py-36 overflow-hidden bg-hero-night">
        <div className="absolute inset-0 opacity-30 bg-grid-pattern" />
        <div className="absolute top-20 left-1/4 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-1/4 w-80 h-80 bg-secondary-500/10 rounded-full blur-3xl" />

        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <Badge variant="primary" pill className="mb-6 text-sm">
            <Shield className="w-4 h-4 mr-1 inline" />
            Our Mission
          </Badge>

          <h1 className="text-5xl lg:text-7xl font-bold mb-6">
            <span className="gradient-text">Digital Safety</span>
            <br />
            for the Modern Age
          </h1>

          <p className="text-xl lg:text-2xl text-white/80 max-w-3xl mx-auto mb-10">
            Guardr is AI-powered dating safety built for the LGBTQ+ community.
            Because everyone deserves to connect without compromise.
          </p>

          <div className="flex flex-wrap justify-center gap-6 text-sm text-white/60">
            <span className="flex items-center gap-2">
              <CheckCircle className="w-4 h-4 text-success-500" />
              69% of LGBTQ+ users face harassment online
            </span>
            <span className="flex items-center gap-2">
              <CheckCircle className="w-4 h-4 text-success-500" />
              Zero raw PII stored
            </span>
            <span className="flex items-center gap-2">
              <CheckCircle className="w-4 h-4 text-success-500" />
              GDPR &amp; CCPA compliant
            </span>
          </div>
        </div>
      </section>

      {/* Our Story */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-12 h-12 rounded-xl bg-primary-500/20 flex items-center justify-center">
                <Sparkles className="w-6 h-6 text-primary-400" />
              </div>
              <h2 className="text-3xl lg:text-4xl font-bold">Our Story</h2>
            </div>

            <Card variant="glass" padding="xl" className="mb-8">
              <p className="text-lg text-white/80 leading-relaxed mb-6">
                Guardr was inspired by real experiences in online dating. Too many people—especially
                in the LGBTQ+ community—have encountered catfish profiles, bad actors, and situations
                that put their safety at risk. We built Guardr because the dating apps weren&apos;t
                doing enough.
              </p>
              <p className="text-lg text-white/80 leading-relaxed mb-6">
                What started as a simple idea—&ldquo;What if you could verify someone before the first
                date?&rdquo;—became a full AI-powered safety platform. We combine OSINT intelligence,
                facial recognition cross-referencing, and real-time risk assessment to give you the
                confidence to swipe right.
              </p>
              <p className="text-lg text-white/80 leading-relaxed">
                Our tagline says it all: <strong className="text-white">2FA for your heart.</strong>{' '}
                Just like two-factor authentication protects your accounts, Guardr adds a second
                layer of verification to your dating life.
              </p>
            </Card>
          </div>
        </div>
      </section>

      {/* Founder */}
      <section className="py-20 lg:py-28 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="flex flex-col md:flex-row items-center gap-10">
              <div className="w-48 h-48 lg:w-56 lg:h-56 rounded-2xl overflow-hidden flex-shrink-0 shadow-glow-primary">
                <Image
                  src="/images/founder.png"
                  alt="Caleb Mills - Founder of Guardr"
                  width={224}
                  height={224}
                  className="w-full h-full object-cover"
                />
              </div>
              <div>
                <Badge variant="primary" pill className="mb-3">
                  <Users className="w-4 h-4 mr-1 inline" />
                  Founder
                </Badge>
                <h2 className="text-3xl lg:text-4xl font-bold mb-4">Caleb Mills</h2>
                <p className="text-lg text-white/80 leading-relaxed mb-4">
                  After years of navigating online dating as a member of the LGBTQ+ community,
                  Caleb experienced firsthand the gaps in safety that existing platforms leave wide
                  open. Catfish profiles, bad actors, and a lack of accountability drove him to
                  build something better.
                </p>
                <p className="text-white/60 leading-relaxed">
                  Combining a background in technology with a passion for community safety,
                  Caleb created Guardr to be the tool he wished existed—AI-powered protection
                  that puts the LGBTQ+ community first.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Meet Midnight */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto text-center">
            <div className="w-48 h-48 lg:w-64 lg:h-64 rounded-3xl overflow-hidden mx-auto mb-8 shadow-glow-primary-lg">
              <Image
                src="/images/midnight.png"
                alt="Midnight - Guardr's protective mascot dog wearing a rainbow pride collar with a G tag"
                width={256}
                height={256}
                className="w-full h-full object-cover"
                priority
              />
            </div>

            <Badge variant="secondary" pill className="mb-4">
              <Star className="w-4 h-4 mr-1 inline" />
              Meet Our Mascot
            </Badge>

            <h2 className="text-3xl lg:text-4xl font-bold mb-6">Meet Midnight</h2>

            <Card variant="glass" padding="xl" className="max-w-2xl mx-auto">
              <p className="text-lg text-white/80 leading-relaxed mb-4">
                Midnight is our protective guardian—a loyal companion who represents trust,
                loyalty, and inclusivity. She watches over your connections, helping verify
                profiles and ensuring safer dating experiences.
              </p>
              <p className="text-white/60">
                Just like the best guard dogs, Midnight is always alert, always caring,
                and always on your side. She&apos;s the spirit of what Guardr stands for:
                fierce protection with unconditional love.
              </p>
            </Card>
          </div>
        </div>
      </section>

      {/* How We're Different */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <Badge variant="primary" pill className="mb-4">
              <Target className="w-4 h-4 mr-1 inline" />
              What Sets Us Apart
            </Badge>
            <h2 className="text-3xl lg:text-4xl font-bold mb-4">How We&apos;re Different</h2>
            <p className="text-lg text-white/70 max-w-2xl mx-auto">
              Guardr isn&apos;t another dating app. It&apos;s digital self-defense.
            </p>
          </div>

          <div className="grid md:grid-cols-2 gap-6 max-w-5xl mx-auto">
            {differentiators.map((item) => (
              <Card key={item.title} variant="glass" padding="lg" hover className={item.glow}>
                <div className="flex items-start gap-4">
                  <div className={`w-12 h-12 rounded-xl bg-surface-300 flex items-center justify-center flex-shrink-0 ${item.color}`}>
                    <item.icon className="w-6 h-6" />
                  </div>
                  <div>
                    <h3 className="text-xl font-semibold mb-2">{item.title}</h3>
                    <p className="text-white/70 leading-relaxed">{item.description}</p>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Core Values */}
      <section className="py-20 lg:py-28 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl lg:text-4xl font-bold mb-4">Core Values</h2>
            <p className="text-lg text-white/70 max-w-2xl mx-auto">
              Every feature, every line of code, every decision is guided by these principles.
            </p>
          </div>

          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6 max-w-5xl mx-auto">
            {coreValues.map((value) => (
              <Card key={value.label} variant="bordered" padding="lg" className="text-center">
                <div className="w-14 h-14 rounded-xl bg-primary-500/20 flex items-center justify-center mx-auto mb-4">
                  <value.icon className="w-7 h-7 text-primary-400" />
                </div>
                <h3 className="text-lg font-semibold mb-2">{value.label}</h3>
                <p className="text-sm text-white/60">{value.description}</p>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Stats */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid sm:grid-cols-3 gap-8 max-w-4xl mx-auto text-center">
            <div>
              <div className="text-4xl lg:text-5xl font-bold gradient-text mb-2">69%</div>
              <p className="text-white/60">of LGBTQ+ users report online harassment</p>
            </div>
            <div>
              <div className="text-4xl lg:text-5xl font-bold gradient-text mb-2">&lt;5s</div>
              <p className="text-white/60">average AI analysis time per profile</p>
            </div>
            <div>
              <div className="text-4xl lg:text-5xl font-bold gradient-text mb-2">99.2%</div>
              <p className="text-white/60">verification accuracy rate</p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-20 lg:py-32 bg-cta-flare text-white relative overflow-hidden">
        <div className="absolute inset-0 opacity-40 bg-grid-pattern" />
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <h2 className="text-4xl lg:text-5xl font-bold mb-6">See How Guardr Works</h2>

          <p className="text-xl lg:text-2xl mb-10 max-w-3xl mx-auto opacity-90">
            Ready to add a second layer of protection to your dating life?
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link href="/how-it-works">
              <Button
                variant="secondary"
                size="xl"
                className="text-lg px-8 py-4"
                icon={ArrowRight}
                iconPosition="right"
              >
                See How It Works
              </Button>
            </Link>
            <Link href="/pricing">
              <Button
                variant="outline"
                size="xl"
                className="text-lg px-8 py-4 border-white text-white hover:bg-white hover:text-primary-600"
              >
                View Pricing
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
