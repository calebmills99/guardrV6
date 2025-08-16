'use client';

import { useState } from 'react';
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
  X,
  Crown,
  Smartphone,
  Eye,
  Database,
  Brain
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';

export default function Pricing() {
  const [billingCycle, setBillingCycle] = useState<'monthly' | 'yearly'>('monthly');

  const plans = [
    {
      name: 'Essential Safety',
      price: billingCycle === 'monthly' ? 6.99 : 69.99,
      originalPrice: billingCycle === 'yearly' ? 83.88 : null,
      popular: true,
      description: 'Perfect for regular daters who want basic protection',
      features: [
        'AI photo verification',
        'Basic risk assessment',
        '5 profile checks per month',
        'SMS safety alerts',
        'Emergency contact system',
        'Basic social media cross-check',
        'Mobile app access',
        'Email support'
      ],
      limitations: [],
      color: 'primary'
    },
    {
      name: 'Premium Protection',
      price: billingCycle === 'monthly' ? 12.99 : 129.99,
      originalPrice: billingCycle === 'yearly' ? 155.88 : null,
      popular: false,
      description: 'Advanced protection for serious daters and LGBTQ+ community',
      features: [
        'Everything in Essential Safety',
        'Advanced AI analysis',
        'Unlimited profile checks',
        'Real-time breach monitoring',
        'Deep social media analysis',
        'Behavioral pattern detection',
        'Priority SMS alerts',
        'Location check-in system',
        'Advanced reporting',
        '24/7 priority support',
        'Dating coach consultation'
      ],
      limitations: [],
      color: 'secondary'
    },
    {
      name: 'Enterprise Safety',
      price: 'Custom',
      originalPrice: null,
      popular: false,
      description: 'For dating platforms and organizations',
      features: [
        'API integration',
        'Bulk user protection',
        'Custom AI training',
        'White-label solutions',
        'Advanced analytics dashboard',
        'Custom reporting',
        'Dedicated account manager',
        'SLA guarantees',
        'Custom integrations',
        'Training and onboarding'
      ],
      limitations: [],
      color: 'accent'
    }
  ];

  const faqs = [
    {
      question: 'Why $6.99? What does the 69% represent?',
      answer: 'Our pricing reflects the 69% harassment rate faced by LGBTQ+ individuals in online dating. This statistic represents a real safety crisis in our community, and our pricing serves as a constant reminder of why this protection is essential.'
    },
    {
      question: 'How accurate is the AI verification?',
      answer: 'Our AI systems achieve 99.2% accuracy in photo verification and 96.8% accuracy in social media cross-referencing. We continuously improve our algorithms using machine learning and community feedback.'
    },
    {
      question: 'Do you store my dating profile data?',
      answer: 'No, we use a zero-knowledge architecture. We analyze your data in real-time but don\'t store your dating profiles, messages, or personal information. All analysis happens on encrypted, temporary sessions.'
    },
    {
      question: 'Which dating platforms do you support?',
      answer: 'We support 12+ major platforms including Tinder, Bumble, Hinge, Grindr, HER, OkCupid, and more. Our API integrations are constantly expanding based on user requests.'
    },
    {
      question: 'Can I cancel anytime?',
      answer: 'Yes, you can cancel your subscription at any time with no penalties. We believe in earning your trust every month, not locking you into contracts.'
    },
    {
      question: 'Is there a free trial?',
      answer: 'We offer a 7-day free trial for new users. This includes 3 free profile checks so you can experience the full Guardr protection before committing.'
    }
  ];

  const testimonials = [
    {
      name: 'Sarah M.',
      role: 'Premium User',
      content: 'The $6.99 price point makes this accessible to everyone who needs protection. As someone in the LGBTQ+ community, I appreciate that the pricing reflects our actual risks.',
      rating: 5
    },
    {
      name: 'Alex R.',
      role: 'Essential User',
      content: 'Best investment I\'ve made for my dating life. The peace of mind is worth way more than $6.99/month. Already prevented two potential catfish situations.',
      rating: 5
    },
    {
      name: 'Jordan K.',
      role: 'Premium User',
      content: 'Upgraded to Premium after the first month. The unlimited checks and advanced analysis have been game-changing for my dating confidence.',
      rating: 5
    }
  ];

  const formatPrice = (price: number | string) => {
    if (typeof price === 'string') return price;
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(price);
  };

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative bg-gradient-to-br from-primary-50 via-white to-secondary-50 py-20 lg:py-32 overflow-hidden">
        <div className="absolute inset-0 bg-grid-pattern opacity-5"></div>
        
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
          <div className="text-center">
            <div className="w-24 h-1 pride-gradient mx-auto mb-6 rounded-full"></div>
            
            <h1 className="text-5xl lg:text-6xl font-bold text-gray-900 mb-6">
              Simple, <span className="gradient-text">Transparent</span> Pricing
            </h1>
            
            <p className="text-xl lg:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
              Starting at $6.99/month - a price that reflects the 69% harassment rate in LGBTQ+ online dating.
            </p>

            {/* Billing Toggle */}
            <div className="flex items-center justify-center gap-4 mb-12">
              <span className={`text-lg ${billingCycle === 'monthly' ? 'text-gray-900 font-semibold' : 'text-gray-500'}`}>
                Monthly
              </span>
              <button
                onClick={() => setBillingCycle(billingCycle === 'monthly' ? 'yearly' : 'monthly')}
                className={`relative inline-flex items-center h-6 rounded-full w-11 transition-colors duration-200 ${
                  billingCycle === 'yearly' ? 'bg-primary-600' : 'bg-gray-300'
                }`}
              >
                <span
                  className={`inline-block w-4 h-4 transform bg-white rounded-full transition-transform duration-200 ${
                    billingCycle === 'yearly' ? 'translate-x-6' : 'translate-x-1'
                  }`}
                />
              </button>
              <span className={`text-lg ${billingCycle === 'yearly' ? 'text-gray-900 font-semibold' : 'text-gray-500'}`}>
                Yearly
              </span>
              {billingCycle === 'yearly' && (
                <Badge variant="success" size="sm" pill>
                  Save 17%
                </Badge>
              )}
            </div>

            <div className="flex flex-wrap justify-center gap-6 mb-16 text-sm text-gray-600">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>7-day free trial</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>Cancel anytime</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>30-day money back</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Pricing Cards */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-7xl mx-auto">
            {plans.map((plan, index) => {
              const isPopular = plan.popular;
              const cardVariant = isPopular ? 'elevated' : 'bordered';
              
              return (
                <Card 
                  key={index} 
                  variant={cardVariant}
                  className={`relative h-full ${isPopular ? 'ring-2 ring-primary-500' : ''}`}
                  hover
                >
                  {isPopular && (
                    <div className="absolute -top-4 left-1/2 transform -translate-x-1/2">
                      <Badge variant="primary" size="lg" pill className="px-6">
                        Most Popular
                      </Badge>
                    </div>
                  )}
                  
                  <div className="text-center mb-6">
                    <h3 className="text-2xl font-bold text-gray-900 mb-2">
                      {plan.name}
                    </h3>
                    <p className="text-gray-600 mb-4">
                      {plan.description}
                    </p>
                    
                    <div className="mb-4">
                      {typeof plan.price === 'number' ? (
                        <div>
                          <span className="text-4xl font-bold text-gray-900">
                            {formatPrice(plan.price)}
                          </span>
                          <span className="text-gray-600">
                            /{billingCycle === 'monthly' ? 'month' : 'year'}
                          </span>
                          {plan.originalPrice && (
                            <div className="text-sm text-gray-500 line-through">
                              {formatPrice(plan.originalPrice)}/year
                            </div>
                          )}
                        </div>
                      ) : (
                        <div>
                          <span className="text-4xl font-bold text-gray-900">
                            {plan.price}
                          </span>
                          <div className="text-sm text-gray-600">
                            Contact us for pricing
                          </div>
                        </div>
                      )}
                    </div>
                  </div>

                  <ul className="space-y-3 mb-8 flex-grow">
                    {plan.features.map((feature, featureIndex) => (
                      <li key={featureIndex} className="flex items-start gap-3">
                        <CheckCircle className="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" />
                        <span className="text-gray-700">{feature}</span>
                      </li>
                    ))}
                  </ul>

                  <Button 
                    variant={isPopular ? 'primary' : 'outline'}
                    size="lg"
                    fullWidth
                    className="mt-auto"
                  >
                    {plan.name === 'Enterprise Safety' ? 'Contact Sales' : 'Start Free Trial'}
                  </Button>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* Value Proposition */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Why Guardr is Worth Every Penny
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Compare the cost of protection against the cost of becoming a victim.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <Card className="text-center">
              <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <AlertTriangle className="h-8 w-8 text-red-600" />
              </div>
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                Identity Theft Cost
              </h3>
              <p className="text-3xl font-bold text-red-600 mb-2">$1,100+</p>
              <p className="text-sm text-gray-600">Average recovery cost</p>
            </Card>

            <Card className="text-center">
              <div className="w-16 h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Heart className="h-8 w-8 text-orange-600" />
              </div>
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                Emotional Damage
              </h3>
              <p className="text-3xl font-bold text-orange-600 mb-2">Priceless</p>
              <p className="text-sm text-gray-600">Trauma and trust issues</p>
            </Card>

            <Card className="text-center">
              <div className="w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Users className="h-8 w-8 text-yellow-600" />
              </div>
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                Lost Time
              </h3>
              <p className="text-3xl font-bold text-yellow-600 mb-2">Months</p>
              <p className="text-sm text-gray-600">Recovery and healing</p>
            </Card>

            <Card className="text-center ring-2 ring-green-500">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Shield className="h-8 w-8 text-green-600" />
              </div>
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                Guardr Protection
              </h3>
              <p className="text-3xl font-bold text-green-600 mb-2">$6.99</p>
              <p className="text-sm text-gray-600">Monthly peace of mind</p>
            </Card>
          </div>
        </div>
      </section>

      {/* Testimonials */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              What Our Users Say About Value
            </h2>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {testimonials.map((testimonial, index) => (
              <Card key={index} className="h-full">
                <div className="flex text-yellow-400 mb-4">
                  {[...Array(testimonial.rating)].map((_, i) => (
                    <Star key={i} className="h-5 w-5 fill-current" />
                  ))}
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

      {/* FAQ Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Frequently Asked Questions
            </h2>
          </div>

          <div className="max-w-4xl mx-auto space-y-6">
            {faqs.map((faq, index) => (
              <Card key={index}>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">
                  {faq.question}
                </h3>
                <p className="text-gray-600">
                  {faq.answer}
                </p>
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
            Start Your Free Trial Today
          </h2>
          
          <p className="text-xl lg:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
            7 days free, then just $6.99/month. Because your safety is worth more than a coffee.
          </p>

          <Button 
            variant="secondary" 
            size="xl"
            className="text-lg px-8 py-4"
            icon={ArrowRight}
            iconPosition="right"
          >
            Start Free Trial Now
          </Button>

          <div className="mt-8 text-sm opacity-75">
            No credit card required • Cancel anytime • 30-day money back guarantee
          </div>
        </div>
      </section>
    </div>
  );
}