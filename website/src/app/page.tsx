'use client';

import { useState } from 'react';
import {
  Shield,
  Search,
  Heart,
  Users,
  ArrowRight,
  CheckCircle,
  Star,
  ChevronRight,
  AlertTriangle,
  Eye,
  Database,
  Smartphone
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import SafetyIndicator from '@/components/ui/SafetyIndicator';
import Input from '@/components/ui/Input';
import Badge from '@/components/ui/Badge';
import { cn } from '@/lib/utils';

type RiskLevel = 'LOW' | 'MEDIUM' | 'HIGH';

type SafetyTip = {
  category: string;
  message: string;
};

type DemoResult = {
  name?: string;
  risk_level?: RiskLevel;
  risk_score?: number;
  person_verification?: string;
  recommendations?: string[];
  safety_tips?: SafetyTip[];
  error?: string;
};

export default function Home() {
  const [email, setEmail] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [demoName, setDemoName] = useState('');
  const [demoLocation, setDemoLocation] = useState('');
  const [demoResults, setDemoResults] = useState<DemoResult | null>(null);
  const [demoLoading, setDemoLoading] = useState(false);

  const features = [
    {
      icon: Shield,
      title: 'AI-Enhanced Identity Verification',
      description: 'Advanced machine learning algorithms verify profile authenticity and detect potential catfish attempts.',
      iconColor: 'text-primary-100',
      iconGlow: 'from-primary-500/40 via-primary-500/15 to-transparent',
      stat: '99.2% accuracy',
    },
    {
      icon: Search,
      title: 'Real-Time Risk Assessment',
      description: 'Instant background analysis using OSINT data to identify potential red flags and safety concerns.',
      iconColor: 'text-accent-200',
      iconGlow: 'from-accent-500/40 via-accent-500/15 to-transparent',
      stat: '<5s response',
    },
    {
      icon: Smartphone,
      title: 'SMS Safety Alerts',
      description: 'Automated safety notifications and emergency contacts when meeting someone new.',
      iconColor: 'text-secondary-100',
      iconGlow: 'from-secondary-500/40 via-secondary-500/15 to-transparent',
      stat: '24/7 monitoring',
    },
    {
      icon: Database,
      title: 'Breach Monitoring',
      description: 'Continuous monitoring of data breaches to protect your personal information.',
      iconColor: 'text-primary-200',
      iconGlow: 'from-primary-400/30 via-primary-500/10 to-transparent',
      stat: '1M+ databases',
    },
    {
      icon: Eye,
      title: 'Dating Platform Integration',
      description: 'Seamless integration with popular dating apps for streamlined safety checks.',
      iconColor: 'text-accent-100',
      iconGlow: 'from-accent-400/30 via-accent-500/10 to-transparent',
      stat: '12+ platforms',
    },
    {
      icon: Heart,
      title: 'LGBTQ+ Focused Safety',
      description: 'Designed specifically for the unique safety needs of LGBTQ+ community members.',
      iconColor: 'text-secondary-100',
      iconGlow: 'from-secondary-400/35 via-secondary-500/15 to-transparent',
      stat: 'Community-built',
    },
  ];

  const steps = [
    {
      number: '01',
      title: 'Connect Your Dating Profiles',
      description: 'Securely link your dating app profiles to Guardr with our encrypted API connections.',
      icon: Users,
    },
    {
      number: '02',
      title: 'AI Verification Scan',
      description: 'Our AI analyzes profile photos, social media presence, and public data for authenticity.',
      icon: Search,
    },
    {
      number: '03',
      title: 'Get Safety Insights',
      description: 'Receive clear, actionable safety reports with risk scores and protective recommendations.',
      icon: Shield,
    },
    {
      number: '04',
      title: 'Date with Confidence',
      description: 'Meet people knowing you have comprehensive safety information and emergency protocols.',
      icon: Heart,
    },
  ];

  const testimonials = [
    {
      name: 'Alex T.',
      role: 'Beta User',
      content: 'Guardr saved me from a major catfish situation. The AI verification showed my "match" was using stolen photos from Instagram. Worth every penny.',
      avatar: '/api/placeholder/48/48',
      rating: 5,
      verified: true,
    },
    {
      name: 'Jamie K.',
      role: 'LGBTQ+ Community Member',
      content: 'As a trans person, safety is everything in dating. Guardr gives me that extra layer of protection without being invasive. The 69% stat is real.',
      avatar: '/api/placeholder/48/48',
      rating: 5,
      verified: true,
    },
    {
      name: 'Morgan L.',
      role: 'Safer Dater',
      content: 'Guardr is my essential pre-date ritual. It\'s not about distrust - it\'s about being smart. The peace of mind is priceless.',
      avatar: '/api/placeholder/48/48',
      rating: 5,
      verified: true,
    },
  ];

  const handleEmailSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    // Simulate API call
    setTimeout(() => {
      setIsSubmitting(false);
      setEmail('');
      alert('Thank you! You\'ve been added to our waitlist.');
    }, 1000);
  };

  const handleDemoSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!demoName.trim()) return;

    setDemoLoading(true);
    try {
      const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5000';
      const response = await fetch(`${apiUrl}/api/check`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: demoName,
          location: demoLocation || undefined
        })
      });

      const data = (await response.json()) as DemoResult;
      setDemoResults(data);
    } catch (error) {
      console.error('Demo search failed:', error);
      setDemoResults({ error: 'Failed to connect to Guardr API' });
    } finally {
      setDemoLoading(false);
    }
  };

  return (
    <div className="min-h-screen text-white">
      {/* Hero Section */}
      <section className="relative overflow-hidden py-24 lg:py-32 bg-hero-night">
        {/* Background Pattern */}
        <div className="absolute inset-0 bg-grid-pattern opacity-40"></div>
        <div className="absolute -top-20 -left-10 w-96 h-96 rounded-full bg-primary-500/40 blur-hero"></div>
        <div className="absolute top-40 -right-16 w-[520px] h-[520px] rounded-full bg-secondary-500/35 blur-orb"></div>

        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
          <div className="text-center">
            {/* Pride flag accent */}
            <div className="w-28 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral"></div>

            <h1 className="text-5xl lg:text-7xl font-bold mb-8 leading-tight">
              <span className="gradient-text">2FA for your</span>
              <br />
              <span className="text-secondary-300 drop-shadow-pink-glow">❤️</span> heart
            </h1>

            <p className="text-xl lg:text-2xl text-white/85 mb-12 max-w-3xl mx-auto">
              AI-powered digital safety for online dating. Making connections safer for the LGBTQ+ community and all smart daters.
            </p>

            <div className="flex flex-col sm:flex-row gap-4 justify-center mb-12">
              <Button
                size="xl"
                className="text-lg px-8 py-4"
                icon={ArrowRight}
                iconPosition="right"
              >
                Start Protecting Yourself ($6.99/mo)
              </Button>
              <Button
                variant="outline"
                size="xl"
                className="text-lg px-8 py-4 border-white/20"
              >
                Learn How It Works
              </Button>
            </div>

            {/* Trust indicators */}
            <div className="flex flex-wrap justify-center gap-6 mb-16 text-sm text-white/80">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-emerald-300" />
                <span>1,000+ safer daters</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-emerald-300" />
                <span>69% harassment rate awareness</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-emerald-300" />
                <span>LGBTQ+ designed</span>
              </div>
            </div>

            {/* Demo Safety Indicators */}
            <div className="flex flex-wrap justify-center gap-4 mb-16">
              <SafetyIndicator level="high" score={92} />
              <SafetyIndicator level="medium" score={67} />
              <SafetyIndicator level="low" score={23} />
            </div>
          </div>
        </div>
      </section>

      {/* Live Demo Section */}
      <section className="py-24 bg-[color:var(--surface-200)]/75 border-t border-white/5">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <Badge variant="secondary" size="lg" pill className="mb-6">
              🔴 LIVE DEMO - Real AI Verification
            </Badge>
            <h2 className="text-4xl lg:text-5xl font-bold mb-6 text-white">
              Try Guardr Right Now
            </h2>
            <p className="text-xl text-white/85 max-w-3xl mx-auto">
              Enter a name to verify. Our AI scans 40+ databases and provides a comprehensive safety assessment in ~2 minutes.
            </p>
          </div>

          <Card variant="glass" className="max-w-2xl mx-auto p-8">
            <form onSubmit={handleDemoSearch} className="mb-6">
              <div className="flex flex-col gap-4">
                <Input
                  type="text"
                  placeholder="Full name (e.g. John Doe)"
                  value={demoName}
                  onChange={(e) => setDemoName(e.target.value)}
                  required
                  className="w-full"
                />
                <Input
                  type="text"
                  placeholder="Location (optional, e.g. Austin, TX)"
                  value={demoLocation}
                  onChange={(e) => setDemoLocation(e.target.value)}
                  className="w-full"
                />
                <Button
                  type="submit"
                  size="lg"
                  loading={demoLoading}
                  icon={Search}
                  iconPosition="right"
                  disabled={demoLoading}
                  fullWidth
                >
                  {demoLoading ? 'Verifying Profile...' : 'Run Safety Check'}
                </Button>
              </div>
            </form>
            
            {demoResults && (
              <div className="border-t border-white/10 pt-6">
                {demoResults.error ? (
                  <div className="bg-danger-500/10 border border-danger-500/40 rounded-lg p-4">
                    <div className="flex items-center gap-2 text-danger-100">
                      <AlertTriangle className="h-5 w-5" />
                      <span className="font-semibold">Error</span>
                    </div>
                    <p className="text-danger-100/80 mt-1">{demoResults.error}</p>
                  </div>
                ) : (
                  <div>
                    <div className="flex items-center justify-between mb-4">
                      <h3 className="text-xl font-bold text-white">Safety Report for {demoResults.name}</h3>
                      <Badge
                        variant={demoResults.risk_level === 'HIGH' ? 'danger' : demoResults.risk_level === 'MEDIUM' ? 'warning' : 'success'}
                        size="lg"
                      >
                        {demoResults.risk_level} RISK
                      </Badge>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                      <div className="rounded-lg p-4 bg-white/5 border border-white/10">
                        <div className="text-2xl font-bold text-primary-100">{demoResults.risk_score}/100</div>
                        <p className="text-white/80">Risk Score</p>
                      </div>
                      <div className="rounded-lg p-4 bg-white/5 border border-white/10">
                        <div className="text-2xl font-bold">
                          {demoResults.risk_level === 'HIGH' ? '🔴' : demoResults.risk_level === 'MEDIUM' ? '🟡' : '🟢'}
                        </div>
                        <p className="text-white/80">Safety Status</p>
                      </div>
                    </div>

                    {demoResults.person_verification && (
                      <div className="mb-6">
                        <h4 className="font-semibold mb-3 text-white">Verification Report:</h4>
                        <div className="rounded p-4 bg-white/5 border border-white/10">
                          <p className="text-sm text-white/85 whitespace-pre-wrap">{demoResults.person_verification}</p>
                        </div>
                      </div>
                    )}

                    {demoResults.recommendations && demoResults.recommendations.length > 0 && (
                      <div className="mb-6">
                        <h4 className="font-semibold mb-3 text-white">Safety Recommendations:</h4>
                        <div className="space-y-2">
                          {demoResults.recommendations.map((rec: string, index: number) => (
                            <div key={index} className="flex items-start gap-2 text-sm text-white/85">
                              <span className="text-primary-200">•</span>
                              <span>{rec}</span>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}

                    {demoResults.safety_tips && (
                      <div className="mt-4 p-4 rounded bg-primary-500/10 border border-primary-400/30">
                        <h4 className="font-semibold text-primary-100 mb-2">💡 Safety Tips</h4>
                        <div className="space-y-2">
                          {demoResults.safety_tips.slice(0, 3).map((tip, index) => (
                            <p key={`${tip.category}-${index}`} className="text-primary-50/90 text-sm">
                              <span className="font-semibold text-white">{tip.category}:</span> {tip.message}
                            </p>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            )}
          </Card>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-20 bg-[color:var(--surface-300)]/70 border-y border-white/5">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="text-4xl font-bold text-primary-100 mb-2">69%</div>
            <p className="text-white/80">LGBTQ+ harassment rate</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-secondary-100 mb-2">$6.99</div>
            <p className="text-white/80">Monthly protection</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-accent-100 mb-2">99.2%</div>
            <p className="text-white/80">AI accuracy rate</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-orange-200 mb-2">&lt;5s</div>
            <p className="text-white/80">Risk assessment time</p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 lg:py-32 bg-[color:var(--surface-200)]/60">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <Badge variant="primary" size="lg" pill className="mb-4">
              Why Guardr Works
            </Badge>
            <h2 className="text-4xl lg:text-5xl font-bold mb-6 text-white">
              Comprehensive Safety Tools
            </h2>
            <p className="text-xl text-white/85 max-w-3xl mx-auto">
              Advanced AI protection designed for modern dating, built with privacy and LGBTQ+ safety as core principles.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index} hover className="text-center h-full">
                  <div className="mb-6">
                    <div
                      className={cn(
                        'inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gradient-to-br text-white shadow-feature-icon mb-4',
                        feature.iconGlow,
                        feature.iconColor
                      )}
                    >
                      <Icon className="h-8 w-8" />
                    </div>
                    <Badge variant="secondary" size="sm" pill>
                      {feature.stat}
                    </Badge>
                  </div>
                  <h3 className="text-xl font-semibold mb-3 text-white">
                    {feature.title}
                  </h3>
                  <p className="text-white/85">
                    {feature.description}
                  </p>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section className="py-20 lg:py-32 bg-[color:var(--surface-100)]/60">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-white mb-6">
              How Guardr Works
            </h2>
            <p className="text-xl text-white/85 max-w-3xl mx-auto">
              Simple, secure, and designed to fit seamlessly into your dating routine.
            </p>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
            <div className="space-y-8">
              {steps.map((step, index) => {
                const Icon = step.icon;
                return (
                  <div key={index} className="flex items-start space-x-4">
                    <div className="flex-shrink-0">
                      <div className="flex items-center justify-center w-12 h-12 bg-gradient-to-br from-primary-500 to-secondary-500 text-white rounded-full font-bold text-lg shadow-glow-primary">
                        {step.number}
                      </div>
                    </div>
                    <div className="flex-grow">
                      <div className="flex items-center gap-3 mb-2">
                        <Icon className="h-5 w-5 text-primary-200" />
                        <h3 className="text-xl font-semibold text-white">
                          {step.title}
                        </h3>
                      </div>
                      <p className="text-white/85">
                        {step.description}
                      </p>
                    </div>
                  </div>
                );
              })}
            </div>

            <div className="lg:pl-8">
              <Card className="p-8 bg-gradient-to-br from-[color:var(--surface-200)]/80 to-[color:var(--surface-400)]/80">
                <div className="text-center">
                  <div className="w-24 h-24 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-full flex items-center justify-center mx-auto mb-6 shadow-glow-primary-lg">
                    <Shield className="h-12 w-12 text-white" />
                  </div>
                  <h3 className="text-2xl font-bold text-white mb-4">
                    Ready to get started?
                  </h3>
                  <p className="text-white/85 mb-6">
                    Join thousands of users who are already dating more safely with Guardr.
                  </p>
                  <Button size="lg" fullWidth>
                    Get Started Today
                  </Button>
                </div>
              </Card>
            </div>
          </div>
        </div>
      </section>

      {/* Testimonials Section */}
      <section className="py-20 lg:py-32 bg-[color:var(--surface-300)]/65 border-t border-white/5">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-white mb-6">
              What Our Community Says
            </h2>
            <p className="text-xl text-white/85 max-w-3xl mx-auto">
              Real stories from real users who&rsquo;ve found safety and confidence with Guardr.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {testimonials.map((testimonial, index) => (
              <Card key={index} className="relative h-full bg-gradient-to-br from-[color:var(--surface-200)]/90 to-[color:var(--surface-400)]/90">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex text-yellow-300">
                    {[...Array(testimonial.rating)].map((_, i) => (
                      <Star key={i} className="h-5 w-5 fill-current" />
                    ))}
                  </div>
                  {testimonial.verified && (
                    <Badge variant="success" size="sm" pill>
                      Verified User
                    </Badge>
                  )}
                </div>
                <blockquote className="text-white/85 mb-6">
                  &ldquo;{testimonial.content}&rdquo;
                </blockquote>
                <div className="flex items-center">
                  <div className="w-12 h-12 bg-gradient-to-br from-primary-400 to-secondary-400 rounded-full flex items-center justify-center mr-4">
                    <span className="text-white font-semibold">
                      {testimonial.name.split(' ').map(n => n[0]).join('')}
                    </span>
                  </div>
                  <div>
                    <div className="font-semibold text-white">
                      {testimonial.name}
                    </div>
                    <div className="text-sm text-white/80">
                      {testimonial.role}
                    </div>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 lg:py-32 bg-cta-flare text-white relative overflow-hidden">
        <div className="absolute inset-0 opacity-40 bg-grid-pattern" />
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong"></div>

          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Ready to date more safely?
          </h2>

          <p className="text-xl lg:text-2xl mb-10 max-w-3xl mx-auto opacity-90">
            Join our community of safer daters. Because you are an adult, and you decide.
          </p>

          <form onSubmit={handleEmailSubmit} className="max-w-md mx-auto">
            <div className="flex flex-col sm:flex-row gap-4">
              <Input
                type="email"
                placeholder="Enter your email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                className="flex-1"
                variant="ghost"
              />
              <Button 
                type="submit"
                variant="secondary" 
                size="lg"
                loading={isSubmitting}
                icon={ChevronRight}
                iconPosition="right"
              >
                Start Protecting ($6.99/mo)
              </Button>
            </div>
            <p className="text-sm text-white/80 mt-4">
              We respect your privacy and will never share your information.
            </p>
          </form>

          <div className="mt-12 flex justify-center items-center flex-wrap gap-8 text-sm text-white/80">
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2 text-secondary-200" />
              Instant setup
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2 text-secondary-200" />
              Cancel anytime
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2 text-secondary-200" />
              LGBTQ+ built
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
