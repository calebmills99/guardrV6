'use client';

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
  Brain,
  Database,
  Smartphone,
  Eye,
  AlertTriangle,
  MessageSquare,
  Camera,
  Link as LinkIcon,
  Globe,
  Monitor
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import SafetyIndicator from '@/components/ui/SafetyIndicator';
import Badge from '@/components/ui/Badge';

export default function HowItWorks() {
  const detailedSteps = [
    {
      number: '01',
      title: 'Profile Connection & Setup',
      description: 'Connect your dating profiles securely using our encrypted API integrations. We support all major platforms.',
      icon: Users,
      features: [
        'Encrypted profile linking',
        'Multi-platform support',
        'OAuth secure authentication',
        'Zero stored passwords'
      ],
      time: '2 minutes'
    },
    {
      number: '02',
      title: 'AI-Powered Analysis',
      description: 'Our advanced AI systems analyze profile photos, verify social media presence, and cross-reference public data.',
      icon: Brain,
      features: [
        'Reverse image search',
        'Facial recognition verification',
        'Social media cross-referencing',
        'OSINT data correlation'
      ],
      time: '<5 seconds'
    },
    {
      number: '03',
      title: 'Risk Assessment Generation',
      description: 'Generate comprehensive safety reports with clear risk scores and actionable recommendations.',
      icon: Shield,
      features: [
        'Multi-factor risk scoring',
        'Clear safety recommendations',
        'Red flag identification',
        'Confidence indicators'
      ],
      time: 'Instant'
    },
    {
      number: '04',
      title: 'Safety Monitoring & Alerts',
      description: 'Continuous monitoring with SMS alerts, emergency contacts, and real-time safety check-ins.',
      icon: Smartphone,
      features: [
        'SMS safety alerts',
        'Emergency contact system',
        'Location check-ins',
        'Panic button integration'
      ],
      time: '24/7'
    },
  ];

  const techStack = [
    {
      category: 'AI & Machine Learning',
      icon: Brain,
      technologies: [
        'Computer Vision for image verification',
        'Natural Language Processing for profile analysis',
        'Behavioral pattern recognition',
        'Anomaly detection algorithms'
      ]
    },
    {
      category: 'Data Sources',
      icon: Database,
      technologies: [
        'Public social media APIs',
        'Data breach monitoring databases',
        'Open source intelligence (OSINT)',
        'Reverse image search engines'
      ]
    },
    {
      category: 'Security & Privacy',
      icon: Lock,
      technologies: [
        'End-to-end encryption',
        'Zero-knowledge architecture',
        'GDPR & CCPA compliance',
        'SOC 2 Type II certified'
      ]
    },
    {
      category: 'Platform Integration',
      icon: LinkIcon,
      technologies: [
        'REST API integrations',
        'OAuth 2.0 authentication',
        'Webhook real-time updates',
        'Cross-platform compatibility'
      ]
    }
  ];

  const safetyFeatures = [
    {
      title: 'Photo Verification',
      description: 'Advanced reverse image search and facial recognition to detect stolen or fake photos.',
      icon: Camera,
      accuracy: '99.2%',
      color: 'text-blue-600'
    },
    {
      title: 'Social Media Cross-Check',
      description: 'Verify profile consistency across multiple social platforms and dating apps.',
      icon: Globe,
      accuracy: '96.8%',
      color: 'text-green-600'
    },
    {
      title: 'Behavioral Analysis',
      description: 'Identify suspicious messaging patterns and potential catfishing behaviors.',
      icon: MessageSquare,
      accuracy: '94.1%',
      color: 'text-purple-600'
    },
    {
      title: 'Real-Time Monitoring',
      description: 'Continuous monitoring for new data breaches and safety alerts.',
      icon: Monitor,
      accuracy: '24/7',
      color: 'text-orange-600'
    }
  ];

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative bg-gradient-to-br from-primary-50 via-white to-secondary-50 py-20 lg:py-32 overflow-hidden">
        <div className="absolute inset-0 bg-grid-pattern opacity-5"></div>
        
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
          <div className="text-center">
            <div className="w-24 h-1 pride-gradient mx-auto mb-6 rounded-full"></div>
            
            <h1 className="text-5xl lg:text-6xl font-bold text-gray-900 mb-6">
              How <span className="gradient-text">Guardr</span> Works
            </h1>
            
            <p className="text-xl lg:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
              Advanced AI technology meets dating safety. Here's how we protect you every step of the way.
            </p>

            <div className="flex flex-wrap justify-center gap-6 mb-16 text-sm text-gray-600">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>99.2% AI accuracy</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>&lt;5 second analysis</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>12+ platform support</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Detailed Steps Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <Badge variant="primary" size="lg" pill className="mb-4">
              The Guardr Process
            </Badge>
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Four Steps to Safer Dating
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Our streamlined process provides comprehensive safety analysis in seconds.
            </p>
          </div>

          <div className="space-y-16">
            {detailedSteps.map((step, index) => {
              const Icon = step.icon;
              const isEven = index % 2 === 0;
              
              return (
                <div key={index} className={`grid grid-cols-1 lg:grid-cols-2 gap-12 items-center ${isEven ? '' : 'lg:grid-flow-col-dense'}`}>
                  <div className={isEven ? '' : 'lg:col-start-2'}>
                    <div className="flex items-center gap-4 mb-6">
                      <div className="flex items-center justify-center w-16 h-16 bg-primary-600 text-white rounded-full font-bold text-xl">
                        {step.number}
                      </div>
                      <div>
                        <Badge variant="secondary" size="sm" pill>
                          {step.time}
                        </Badge>
                      </div>
                    </div>
                    
                    <div className="flex items-center gap-3 mb-4">
                      <Icon className="h-8 w-8 text-primary-600" />
                      <h3 className="text-3xl font-bold text-gray-900">
                        {step.title}
                      </h3>
                    </div>
                    
                    <p className="text-lg text-gray-600 mb-6">
                      {step.description}
                    </p>
                    
                    <ul className="space-y-3">
                      {step.features.map((feature, featureIndex) => (
                        <li key={featureIndex} className="flex items-center gap-3">
                          <CheckCircle className="h-5 w-5 text-green-500 flex-shrink-0" />
                          <span className="text-gray-700">{feature}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                  
                  <div className={isEven ? 'lg:col-start-2' : ''}>
                    <Card className="p-8 h-full">
                      <div className="text-center">
                        <div className="w-24 h-24 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-full flex items-center justify-center mx-auto mb-6">
                          <Icon className="h-12 w-12 text-white" />
                        </div>
                        <h4 className="text-xl font-semibold text-gray-900 mb-4">
                          Step {step.number}
                        </h4>
                        <div className="flex justify-center">
                          <SafetyIndicator 
                            level={index === 0 ? 'medium' : index === 1 ? 'high' : index === 2 ? 'high' : 'high'} 
                            score={index === 0 ? 75 : index === 1 ? 95 : index === 2 ? 88 : 92}
                            size="lg"
                          />
                        </div>
                      </div>
                    </Card>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Technology Stack Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Powered by Advanced Technology
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Our comprehensive tech stack ensures the highest accuracy and reliability in dating safety.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {techStack.map((tech, index) => {
              const Icon = tech.icon;
              return (
                <Card key={index} hover className="h-full">
                  <div className="flex items-center gap-4 mb-6">
                    <div className="w-12 h-12 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-lg flex items-center justify-center">
                      <Icon className="h-6 w-6 text-white" />
                    </div>
                    <h3 className="text-xl font-semibold text-gray-900">
                      {tech.category}
                    </h3>
                  </div>
                  
                  <ul className="space-y-3">
                    {tech.technologies.map((technology, techIndex) => (
                      <li key={techIndex} className="flex items-start gap-3">
                        <CheckCircle className="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" />
                        <span className="text-gray-700">{technology}</span>
                      </li>
                    ))}
                  </ul>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* Safety Features Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Advanced Safety Features
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Multi-layered protection systems working together to keep you safe.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {safetyFeatures.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index} hover className="h-full">
                  <div className="flex items-start gap-4">
                    <div className={`w-12 h-12 rounded-lg bg-gray-100 flex items-center justify-center ${feature.color}`}>
                      <Icon className="h-6 w-6" />
                    </div>
                    <div className="flex-grow">
                      <div className="flex items-center justify-between mb-2">
                        <h3 className="text-lg font-semibold text-gray-900">
                          {feature.title}
                        </h3>
                        <Badge variant="success" size="sm" pill>
                          {feature.accuracy}
                        </Badge>
                      </div>
                      <p className="text-gray-600">
                        {feature.description}
                      </p>
                    </div>
                  </div>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 lg:py-32 bg-gradient-to-br from-primary-600 to-secondary-600 text-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full"></div>
          
          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Ready to Experience Guardr?
          </h2>
          
          <p className="text-xl lg:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
            Join thousands of users who trust Guardr to make their dating experience safer and more confident.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button 
              variant="secondary" 
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
              className="text-lg px-8 py-4 border-white text-white hover:bg-white hover:text-primary-600"
            >
              See Pricing Options
            </Button>
          </div>

          <div className="mt-12 flex justify-center items-center flex-wrap gap-8 text-sm opacity-75">
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              No long-term contracts
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              Cancel anytime
            </div>
            <div className="flex items-center">
              <CheckCircle className="h-4 w-4 mr-2" />
              30-day money back guarantee
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}