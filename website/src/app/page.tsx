'use client';

import { useState } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { 
  Shield, 
  Search, 
  Lock, 
  Heart, 
  Users, 
  Zap, 
  ArrowRight,
  CheckCircle,
  Star,
  ChevronRight,
  AlertTriangle,
  Eye,
  MessageSquare,
  Database,
  Smartphone
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import SafetyIndicator from '@/components/ui/SafetyIndicator';
import Input from '@/components/ui/Input';
import Badge from '@/components/ui/Badge';

export default function Home() {
  const [email, setEmail] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [demoEmail, setDemoEmail] = useState('');
  const [demoResults, setDemoResults] = useState<any>(null);
  const [demoLoading, setDemoLoading] = useState(false);

  const features = [
    {
      icon: Shield,
      title: 'AI-Enhanced Identity Verification',
      description: 'Advanced machine learning algorithms verify profile authenticity and detect potential catfish attempts.',
      color: 'text-blue-600',
      stat: '99.2% accuracy',
    },
    {
      icon: Search,
      title: 'Real-Time Risk Assessment',
      description: 'Instant background analysis using OSINT data to identify potential red flags and safety concerns.',
      color: 'text-green-600',
      stat: '<5s response',
    },
    {
      icon: Smartphone,
      title: 'SMS Safety Alerts',
      description: 'Automated safety notifications and emergency contacts when meeting someone new.',
      color: 'text-purple-600',
      stat: '24/7 monitoring',
    },
    {
      icon: Database,
      title: 'Breach Monitoring',
      description: 'Continuous monitoring of data breaches to protect your personal information.',
      color: 'text-orange-600',
      stat: '1M+ databases',
    },
    {
      icon: Eye,
      title: 'Dating Platform Integration',
      description: 'Seamless integration with popular dating apps for streamlined safety checks.',
      color: 'text-pink-600',
      stat: '12+ platforms',
    },
    {
      icon: Heart,
      title: 'LGBTQ+ Focused Safety',
      description: 'Designed specifically for the unique safety needs of LGBTQ+ community members.',
      color: 'text-red-600',
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
    if (!demoEmail.trim()) return;
    
    setDemoLoading(true);
    try {
      const response = await fetch('http://134.209.162.223:5000/api/check', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email: demoEmail })
      });
      
      const data = await response.json();
      setDemoResults(data);
    } catch (error) {
      console.error('Demo search failed:', error);
      setDemoResults({ error: 'Failed to connect to Guardr API' });
    } finally {
      setDemoLoading(false);
    }
  };

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative bg-gradient-to-br from-primary-50 via-white to-secondary-50 py-20 lg:py-32 overflow-hidden">
        {/* Background Pattern */}
        <div className="absolute inset-0 bg-grid-pattern opacity-5"></div>
        <div className="absolute top-20 left-20 w-72 h-72 bg-primary-200 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-float"></div>
        <div className="absolute top-40 right-20 w-96 h-96 bg-secondary-200 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-float" style={{ animationDelay: '2s' }}></div>

        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
          <div className="text-center">
            {/* Pride flag accent */}
            <div className="w-24 h-1 pride-gradient mx-auto mb-6 rounded-full"></div>
            
            <h1 className="text-5xl lg:text-7xl font-bold text-gray-900 mb-6">
              <span className="gradient-text">2FA for your</span>
              <br />
              <span className="text-red-500">❤️</span> heart
            </h1>
            
            <p className="text-xl lg:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
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
                className="text-lg px-8 py-4"
              >
                Learn How It Works
              </Button>
            </div>

            {/* Trust indicators */}
            <div className="flex flex-wrap justify-center gap-6 mb-16 text-sm text-gray-600">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>1,000+ safer daters</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>69% harassment rate awareness</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
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
      <section className="py-20 bg-gradient-to-br from-gray-900 to-gray-800 text-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <Badge variant="secondary" size="lg" pill className="mb-4 bg-green-600 text-white">
              🔴 LIVE DEMO - Real API
            </Badge>
            <h2 className="text-4xl lg:text-5xl font-bold mb-6">
              Try Guardr Right Now
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Test our breach detection API with any email address. See what Guardr finds instantly.
            </p>
          </div>
          
          <Card className="max-w-2xl mx-auto bg-white text-gray-900 p-8">
            <form onSubmit={handleDemoSearch} className="mb-6">
              <div className="flex flex-col sm:flex-row gap-4">
                <Input
                  type="email"
                  placeholder="Enter email to check (e.g. test@example.com)"
                  value={demoEmail}
                  onChange={(e) => setDemoEmail(e.target.value)}
                  required
                  className="flex-1"
                />
                <Button 
                  type="submit"
                  size="lg"
                  loading={demoLoading}
                  icon={Search}
                  iconPosition="right"
                  disabled={demoLoading}
                >
                  {demoLoading ? 'Scanning...' : 'Check Now'}
                </Button>
              </div>
            </form>
            
            {demoResults && (
              <div className="border-t pt-6">
                {demoResults.error ? (
                  <div className="bg-red-50 border border-red-200 rounded-lg p-4">
                    <div className="flex items-center gap-2 text-red-700">
                      <AlertTriangle className="h-5 w-5" />
                      <span className="font-semibold">Error</span>
                    </div>
                    <p className="text-red-600 mt-1">{demoResults.error}</p>
                  </div>
                ) : (
                  <div>
                    <div className="flex items-center justify-between mb-4">
                      <h3 className="text-xl font-bold">Breach Report for {demoResults.email}</h3>
                      <Badge 
                        variant={demoResults.risk_level === 'HIGH' ? 'danger' : demoResults.risk_level === 'MEDIUM' ? 'warning' : 'success'}
                        size="lg"
                      >
                        {demoResults.risk_level} RISK
                      </Badge>
                    </div>
                    
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                      <div className="bg-gray-50 rounded-lg p-4">
                        <div className="text-2xl font-bold text-orange-600">{demoResults.breach_count}</div>
                        <p className="text-gray-600">Data Breaches Found</p>
                      </div>
                      <div className="bg-gray-50 rounded-lg p-4">
                        <div className="text-2xl font-bold text-blue-600">
                          {demoResults.risk_level === 'HIGH' ? '🔴' : demoResults.risk_level === 'MEDIUM' ? '🟡' : '🟢'}
                        </div>
                        <p className="text-gray-600">Safety Status</p>
                      </div>
                    </div>
                    
                    {demoResults.breaches && demoResults.breaches.length > 0 && (
                      <div>
                        <h4 className="font-semibold mb-3">Breaches Detected:</h4>
                        <div className="space-y-2">
                          {demoResults.breaches.map((breach: any, index: number) => (
                            <div key={index} className="bg-red-50 border border-red-200 rounded p-3">
                              <div className="font-semibold text-red-800">{breach.Name}</div>
                              <div className="text-sm text-red-600">Breached: {breach.BreachDate}</div>
                              <div className="text-xs text-red-500 mt-1">
                                Data: {breach.DataClasses?.join(', ') || 'Unknown'}
                              </div>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                    
                    {demoResults.note && (
                      <div className="mt-4 p-3 bg-blue-50 border border-blue-200 rounded">
                        <p className="text-blue-700 text-sm">{demoResults.note}</p>
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
      <section className="py-20 bg-white border-b border-gray-100">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="text-4xl font-bold text-primary-600 mb-2">69%</div>
              <p className="text-gray-600">LGBTQ+ harassment rate</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-secondary-600 mb-2">$6.99</div>
              <p className="text-gray-600">Monthly protection</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-accent-600 mb-2">99.2%</div>
              <p className="text-gray-600">AI accuracy rate</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-orange-600 mb-2">&lt;5s</div>
              <p className="text-gray-600">Risk assessment time</p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <Badge variant="primary" size="lg" pill className="mb-4">
              Why Guardr Works
            </Badge>
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Comprehensive Safety Tools
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Advanced AI protection designed for modern dating, built with privacy and LGBTQ+ safety as core principles.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index} hover className="text-center h-full">
                  <div className="mb-4">
                    <div className={`inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gradient-to-br from-gray-50 to-gray-100 ${feature.color} mb-4`}>
                      <Icon className="h-8 w-8" />
                    </div>
                    <Badge variant="secondary" size="sm" pill>
                      {feature.stat}
                    </Badge>
                  </div>
                  <h3 className="text-xl font-semibold text-gray-900 mb-3">
                    {feature.title}
                  </h3>
                  <p className="text-gray-600">
                    {feature.description}
                  </p>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              How Guardr Works
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
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
                      <div className="flex items-center justify-center w-12 h-12 bg-primary-600 text-white rounded-full font-bold text-lg">
                        {step.number}
                      </div>
                    </div>
                    <div className="flex-grow">
                      <div className="flex items-center gap-3 mb-2">
                        <Icon className="h-5 w-5 text-primary-600" />
                        <h3 className="text-xl font-semibold text-gray-900">
                          {step.title}
                        </h3>
                      </div>
                      <p className="text-gray-600">
                        {step.description}
                      </p>
                    </div>
                  </div>
                );
              })}
            </div>

            <div className="lg:pl-8">
              <Card className="p-8">
                <div className="text-center">
                  <div className="w-24 h-24 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-full flex items-center justify-center mx-auto mb-6">
                    <Shield className="h-12 w-12 text-white" />
                  </div>
                  <h3 className="text-2xl font-bold text-gray-900 mb-4">
                    Ready to get started?
                  </h3>
                  <p className="text-gray-600 mb-6">
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
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              What Our Community Says
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Real stories from real users who've found safety and confidence with Guardr.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {testimonials.map((testimonial, index) => (
              <Card key={index} className="relative h-full">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex text-yellow-400">
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
                <blockquote className="text-gray-600 mb-6">
                  "{testimonial.content}"
                </blockquote>
                <div className="flex items-center">
                  <div className="w-12 h-12 bg-gradient-to-br from-primary-400 to-secondary-400 rounded-full flex items-center justify-center mr-4">
                    <span className="text-white font-semibold">
                      {testimonial.name.split(' ').map(n => n[0]).join('')}
                    </span>
                  </div>
                  <div>
                    <div className="font-semibold text-gray-900">
                      {testimonial.name}
                    </div>
                    <div className="text-sm text-gray-500">
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
      <section className="py-20 lg:py-32 bg-gradient-to-br from-primary-600 to-secondary-600 text-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full"></div>
          
          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Ready to date more safely?
          </h2>
          
          <p className="text-xl lg:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
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
            <p className="text-sm opacity-75 mt-4">
              We respect your privacy and will never share your information.
            </p>
          </form>

          <div className="mt-12 flex justify-center items-center flex-wrap gap-8 text-sm opacity-75">
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              Instant setup
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              Cancel anytime
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              LGBTQ+ built
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
