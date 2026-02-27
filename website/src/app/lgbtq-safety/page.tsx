'use client';

import {
  Shield,
  Heart,
  Phone,
  AlertTriangle,
  ArrowRight,
  MapPin,
  Users,
  MessageSquare,
  Smartphone,
  CheckCircle,
  XCircle,
  Globe
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';
import Link from 'next/link';

const crisisResources = [
  {
    name: 'The Trevor Project',
    description: 'Crisis intervention and suicide prevention for LGBTQ+ youth. Free, confidential, 24/7.',
    phone: '1-866-488-7386',
    extra: 'Text START to 678-678',
  },
  {
    name: 'Crisis Text Line',
    description: 'Free, 24/7 crisis support via text. Available to everyone.',
    phone: null,
    extra: 'Text HOME to 741741',
  },
  {
    name: 'Trans Lifeline',
    description: 'Peer support for the trans community. Run by and for trans people.',
    phone: '1-877-565-8860',
    extra: null,
  },
  {
    name: 'SAGE LGBT Elder Hotline',
    description: 'Support for LGBTQ+ elders. Confidential, in English and Spanish.',
    phone: '1-877-360-5428',
    extra: null,
  },
  {
    name: 'National Coalition of Anti-Violence Programs (NCAVP)',
    description: 'Support for LGBTQ+ survivors of violence. Local referrals and advocacy.',
    phone: '1-212-714-1141',
    extra: 'Or find your local AVP at avp.org',
  },
];

const safeMeetingTips = [
  {
    icon: MapPin,
    title: 'Meet in Public First',
    description: 'Choose busy, well-lit, LGBTQ+-friendly venues. Coffee shops, bars, or parks in queer neighborhoods. Avoid private homes for first meetings.',
  },
  {
    icon: Users,
    title: 'Tell Someone Your Plans',
    description: 'Share your date\'s profile, location, and expected return time with a trusted friend. Consider a check-in call or text mid-date.',
  },
  {
    icon: Smartphone,
    title: 'Arrange Your Own Ride',
    description: 'Drive yourself or use your own rideshare. Never get picked up or dropped off at your home on a first meet.',
  },
  {
    icon: MessageSquare,
    title: 'Keep Chats on the App',
    description: 'Stay on Grindr, Scruff, HER, or Tinder until you\'ve verified the person. Be cautious of anyone pushing for phone numbers or off-app messaging too fast.',
  },
  {
    icon: Shield,
    title: 'Verify Before You Meet',
    description: 'Use Guardr to check profiles before meeting. Catfish, scammers, and bad actors are real—knowledge is your best defense.',
  },
];

const redFlags = [
  { flag: 'Refuses video calls', severity: 'high' },
  { flag: 'Asks for money or "investments"', severity: 'high' },
  { flag: 'Pushes to meet immediately without rapport', severity: 'medium' },
  { flag: 'Inconsistent stories or details that don\'t add up', severity: 'high' },
  { flag: 'Love bombing—excessive flattery too early', severity: 'high' },
  { flag: 'No social media presence or very limited', severity: 'medium' },
  { flag: 'Avoids specific questions about local places', severity: 'medium' },
  { flag: 'Pressure to share personal info (address, workplace) early', severity: 'high' },
];

const platformTips = [
  {
    name: 'Grindr / Scruff',
    tips: 'Verify location claims. Be cautious of profiles with only torso shots. Watch for "traveling" or "visiting" profiles—common scam setup.',
  },
  {
    name: 'HER / Tinder / Bumble',
    tips: 'Check that photos match across platforms. Catfish often use stolen or heavily edited images. Reverse image search helps.',
  },
];

export default function LgbtqSafety() {
  return (
    <div className="min-h-screen">
      {/* Hero */}
      <section className="relative py-24 lg:py-36 overflow-hidden bg-hero-night">
        <div className="absolute inset-0 opacity-30 bg-grid-pattern" />
        <div className="absolute top-20 left-1/4 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-1/4 w-80 h-80 bg-secondary-500/10 rounded-full blur-3xl" />

        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <Badge variant="secondary" pill className="mb-6 text-sm">
            <Heart className="w-4 h-4 mr-1 inline" />
            Built for Our Community
          </Badge>

          <h1 className="text-5xl lg:text-7xl font-bold mb-6">
            <span className="gradient-text">LGBTQ+ Safety</span>
            <br />
            Resources
          </h1>

          <p className="text-xl lg:text-2xl text-white/80 max-w-3xl mx-auto mb-10">
            LGBTQ+ individuals face higher risks of catfishing, fraud, and violence via dating apps.
            Guardr empowers you with knowledge so you don&apos;t walk into a dangerous situation.
          </p>

          <div className="flex flex-wrap justify-center gap-6 text-sm text-white/60">
            <span className="flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 text-warning-500" />
              69% of LGBTQ+ users report online harassment
            </span>
            <span className="flex items-center gap-2">
              <CheckCircle className="w-4 h-4 text-success-500" />
              Digital self-defense is your right
            </span>
          </div>
        </div>
      </section>

      {/* Why This Matters */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">Why This Matters</h2>
              <p className="text-lg text-white/70 max-w-2xl mx-auto">
                Dating apps don&apos;t verify users. Guardr fills that gap.
              </p>
            </div>

            <Card variant="glass" padding="xl">
              <p className="text-lg text-white/80 leading-relaxed mb-6">
                Queer folks face disproportionate risks in online dating: catfishing, romance scams,
                harassment, and violence. Apps like Grindr, Tinder, and HER offer little to no
                identity verification—leaving you to figure out who&apos;s real on your own.
              </p>
              <p className="text-lg text-white/80 leading-relaxed mb-6">
                Think of Guardr like <strong className="text-white">digital self-defense</strong>—not
                unlike checking someone&apos;s social media before meeting them. We use publicly
                available data and AI to help you assess risk before the first date. Built
                specifically for the queer community&apos;s unique security concerns.
              </p>
              <p className="text-white/60 leading-relaxed">
                You deserve to connect without compromise. Knowledge is power.
              </p>
            </Card>
          </div>
        </div>
      </section>

      {/* Crisis Resources */}
      <section className="py-20 lg:py-28 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <Badge variant="danger" pill className="mb-4">
                <AlertTriangle className="w-4 h-4 mr-1 inline" />
                Crisis Resources
              </Badge>
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">
                LGBTQ+ Emergency Resources
              </h2>
              <p className="text-white/70 max-w-2xl mx-auto">
                If you or someone you know is in crisis, these organizations offer
                free, confidential support 24/7.
              </p>
            </div>

            <div className="space-y-4 mb-8">
              {crisisResources.map((resource) => (
                <Card key={resource.name} variant="bordered" padding="lg">
                  <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                    <div className="flex-1">
                      <h3 className="text-lg font-semibold mb-1">{resource.name}</h3>
                      <p className="text-white/60 text-sm">{resource.description}</p>
                    </div>
                    <div className="flex flex-col sm:items-end gap-1">
                      {resource.phone && (
                        <a
                          href={`tel:${resource.phone.replace(/[^0-9+]/g, '')}`}
                          className="flex items-center gap-2 text-primary-400 font-semibold hover:underline"
                        >
                          <Phone className="w-4 h-4" />
                          {resource.phone}
                        </a>
                      )}
                      {resource.extra && (
                        <span className="text-sm text-accent-400 font-medium">
                          {resource.extra}
                        </span>
                      )}
                    </div>
                  </div>
                </Card>
              ))}
            </div>

            <Card variant="glass" padding="lg" className="text-center border border-danger-500/30">
              <p className="text-white/80 font-medium">
                <AlertTriangle className="w-5 h-5 inline mr-2 text-danger-500" />
                If you are in <strong>immediate danger</strong>, please call <strong>911</strong> or your
                local emergency number.
              </p>
            </Card>
          </div>
        </div>
      </section>

      {/* Safe Meeting Tips */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <Badge variant="primary" pill className="mb-4">
                <MapPin className="w-4 h-4 mr-1 inline" />
                Safe Meeting
              </Badge>
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">Safe Meeting Tips</h2>
              <p className="text-white/70 max-w-2xl mx-auto">
                Practical steps to stay safe when meeting someone from a dating app.
              </p>
            </div>

            <div className="space-y-4">
              {safeMeetingTips.map((tip) => (
                <Card key={tip.title} variant="glass" padding="lg">
                  <div className="flex items-start gap-4">
                    <div className="w-12 h-12 rounded-xl bg-primary-500/20 flex items-center justify-center flex-shrink-0">
                      <tip.icon className="w-6 h-6 text-primary-400" />
                    </div>
                    <div>
                      <h3 className="text-lg font-semibold mb-2">{tip.title}</h3>
                      <p className="text-white/70 leading-relaxed">{tip.description}</p>
                    </div>
                  </div>
                </Card>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Red Flags */}
      <section className="py-20 lg:py-28 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <Badge variant="warning" pill className="mb-4">
                <XCircle className="w-4 h-4 mr-1 inline" />
                Watch For
              </Badge>
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">Red Flags in Queer Dating</h2>
              <p className="text-white/70 max-w-2xl mx-auto">
                If you see these, slow down or reconsider.
              </p>
            </div>

            <div className="grid sm:grid-cols-2 gap-4">
              {redFlags.map((item) => (
                <Card key={item.flag} variant="bordered" padding="md">
                  <div className="flex items-center gap-3">
                    <XCircle className={`w-5 h-5 flex-shrink-0 ${item.severity === 'high' ? 'text-danger-500' : 'text-warning-500'}`} />
                    <span className="text-white/80">{item.flag}</span>
                  </div>
                </Card>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Platform-Specific */}
      <section className="py-20 lg:py-28">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <Badge variant="secondary" pill className="mb-4">
                <Globe className="w-4 h-4 mr-1 inline" />
                By Platform
              </Badge>
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">Platform-Specific Guidance</h2>
            </div>

            <div className="space-y-4">
              {platformTips.map((platform) => (
                <Card key={platform.name} variant="glass" padding="lg">
                  <h3 className="text-lg font-semibold mb-2">{platform.name}</h3>
                  <p className="text-white/70">{platform.tips}</p>
                </Card>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-20 lg:py-32 bg-cta-flare text-white relative overflow-hidden">
        <div className="absolute inset-0 opacity-40 bg-grid-pattern" />
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Your Safety Should Never Be a Gamble
          </h2>

          <p className="text-xl lg:text-2xl mb-10 max-w-3xl mx-auto opacity-90">
            Guardr verifies profiles before you meet. AI-powered protection built for the LGBTQ+ community.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link href="/pricing">
              <Button
                variant="secondary"
                size="xl"
                className="text-lg px-8 py-4"
                icon={ArrowRight}
                iconPosition="right"
              >
                Get AI Protection ($6.99/mo)
              </Button>
            </Link>
            <Link href="/safety-tips">
              <Button
                variant="outline"
                size="xl"
                className="text-lg px-8 py-4 border-white text-white hover:bg-white hover:text-primary-600"
              >
                More Safety Tips
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
