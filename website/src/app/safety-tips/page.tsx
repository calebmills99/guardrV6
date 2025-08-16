'use client';

import { 
  Shield, 
  Search, 
  Lock, 
  Heart, 
  Users, 
  Eye,
  ArrowRight,
  CheckCircle,
  AlertTriangle,
  MessageSquare,
  MapPin,
  Camera,
  Phone,
  Clock,
  UserCheck,
  Flag,
  Lightbulb,
  BookOpen,
  HelpCircle,
  Globe
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';

export default function SafetyTips() {
  const safetyCategories = [
    {
      title: 'Profile Safety',
      icon: UserCheck,
      color: 'text-blue-600',
      bgColor: 'bg-blue-100',
      tips: [
        {
          title: 'Verify Profile Photos',
          description: 'Use reverse image search to check if photos appear elsewhere online.',
          importance: 'high'
        },
        {
          title: 'Check Social Media Consistency',
          description: 'Look for consistent information across different social platforms.',
          importance: 'high'
        },
        {
          title: 'Watch for Limited Photos',
          description: 'Be cautious of profiles with only 1-2 photos or professional-looking shots.',
          importance: 'medium'
        },
        {
          title: 'Verify Location Claims',
          description: 'Ask specific questions about local places and landmarks.',
          importance: 'medium'
        }
      ]
    },
    {
      title: 'Communication Safety',
      icon: MessageSquare,
      color: 'text-green-600',
      bgColor: 'bg-green-100',
      tips: [
        {
          title: 'Start with App Messaging',
          description: 'Keep conversations on the dating platform initially.',
          importance: 'high'
        },
        {
          title: 'Avoid Sharing Personal Info',
          description: 'Don\'t share your last name, address, workplace, or financial information.',
          importance: 'high'
        },
        {
          title: 'Watch for Love Bombing',
          description: 'Be wary of excessive flattery or emotional intensity early on.',
          importance: 'medium'
        },
        {
          title: 'Trust Your Instincts',
          description: 'If something feels off, it probably is. Don\'t ignore red flags.',
          importance: 'high'
        }
      ]
    },
    {
      title: 'Meeting Safety',
      icon: MapPin,
      color: 'text-purple-600',
      bgColor: 'bg-purple-100',
      tips: [
        {
          title: 'Meet in Public Places',
          description: 'Always choose busy, well-lit public venues for first meetings.',
          importance: 'high'
        },
        {
          title: 'Tell Someone Your Plans',
          description: 'Share your date details with a trusted friend or family member.',
          importance: 'high'
        },
        {
          title: 'Arrange Your Own Transportation',
          description: 'Drive yourself or use your own rideshare. Don\'t get picked up.',
          importance: 'high'
        },
        {
          title: 'Stay Sober',
          description: 'Limit alcohol consumption and never leave drinks unattended.',
          importance: 'medium'
        }
      ]
    },
    {
      title: 'LGBTQ+ Specific Safety',
      icon: Heart,
      color: 'text-pink-600',
      bgColor: 'bg-pink-100',
      tips: [
        {
          title: 'Control Your Coming Out',
          description: 'Only share your LGBTQ+ status when you feel safe and ready.',
          importance: 'high'
        },
        {
          title: 'Research Date Locations',
          description: 'Choose LGBTQ+-friendly venues and neighborhoods when possible.',
          importance: 'medium'
        },
        {
          title: 'Have an Exit Strategy',
          description: 'Always have a plan to leave safely if you feel uncomfortable.',
          importance: 'high'
        },
        {
          title: 'Connect with Community',
          description: 'Use LGBTQ+ dating platforms and communities for additional safety.',
          importance: 'medium'
        }
      ]
    }
  ];

  const redFlags = [
    {
      flag: 'Refuses Video Calls',
      description: 'Won\'t do video chats despite repeated requests',
      severity: 'high'
    },
    {
      flag: 'Asks for Money',
      description: 'Requests financial help or investment opportunities',
      severity: 'high'
    },
    {
      flag: 'Limited Photos',
      description: 'Has very few photos or they all look professional',
      severity: 'medium'
    },
    {
      flag: 'Pushes for Quick Meeting',
      description: 'Wants to meet immediately without building rapport',
      severity: 'medium'
    },
    {
      flag: 'Inconsistent Stories',
      description: 'Details about their life don\'t add up or change',
      severity: 'high'
    },
    {
      flag: 'Avoids Specific Questions',
      description: 'Deflects when asked about local places or details',
      severity: 'medium'
    },
    {
      flag: 'Love Bombing',
      description: 'Excessive flattery and emotional intensity too early',
      severity: 'high'
    },
    {
      flag: 'No Social Media Presence',
      description: 'Claims to have no social media or very limited presence',
      severity: 'medium'
    }
  ];

  const emergencyPlan = [
    {
      step: '1',
      title: 'Create a Safety Plan',
      description: 'Before any date, create a detailed plan including location, time, and expected return.',
      icon: BookOpen
    },
    {
      step: '2',
      title: 'Share Your Plan',
      description: 'Send your date details to at least one trusted friend or family member.',
      icon: Users
    },
    {
      step: '3',
      title: 'Set Check-in Times',
      description: 'Establish specific times to check in with your safety contact.',
      icon: Clock
    },
    {
      step: '4',
      title: 'Know Emergency Contacts',
      description: 'Have local emergency numbers saved and easily accessible.',
      icon: Phone
    },
    {
      step: '5',
      title: 'Trust Your Instincts',
      description: 'If something feels wrong, leave immediately. Your safety comes first.',
      icon: AlertTriangle
    }
  ];

  const lgbtqResources = [
    {
      name: 'Trans Lifeline',
      description: 'Crisis support for transgender people',
      contact: '877-565-8860',
      website: 'translifeline.org'
    },
    {
      name: 'LGBT National Hotline',
      description: '24/7 support for LGBTQ+ individuals',
      contact: '1-888-843-4564',
      website: 'lgbthotline.org'
    },
    {
      name: 'GLAAD',
      description: 'Media advocacy and LGBTQ+ resources',
      contact: 'Contact via website',
      website: 'glaad.org'
    },
    {
      name: 'The Trevor Project',
      description: 'Crisis intervention for LGBTQ+ youth',
      contact: '1-866-488-7386',
      website: 'thetrevorproject.org'
    }
  ];

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'high':
        return 'text-red-600 bg-red-100';
      case 'medium':
        return 'text-orange-600 bg-orange-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const getImportanceColor = (importance: string) => {
    switch (importance) {
      case 'high':
        return 'border-red-300 bg-red-50';
      case 'medium':
        return 'border-orange-300 bg-orange-50';
      default:
        return 'border-gray-300 bg-gray-50';
    }
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
              Stay <span className="gradient-text">Safe</span> While Dating
            </h1>
            
            <p className="text-xl lg:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
              Essential safety tips for online dating, with special focus on LGBTQ+ community protection.
            </p>

            <div className="flex flex-wrap justify-center gap-6 mb-16 text-sm text-gray-600">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>69% harassment awareness</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>LGBTQ+ focused tips</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle className="h-4 w-4 text-green-500" />
                <span>Community reviewed</span>
              </div>
            </div>

            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-6 max-w-3xl mx-auto mb-16">
              <div className="flex items-start gap-3">
                <AlertTriangle className="h-6 w-6 text-yellow-600 flex-shrink-0 mt-0.5" />
                <div className="text-left">
                  <h3 className="text-lg font-semibold text-yellow-800 mb-2">
                    Important Safety Reminder
                  </h3>
                  <p className="text-yellow-700">
                    LGBTQ+ individuals face a 69% harassment rate in online dating. These tips can help reduce your risk, 
                    but always trust your instincts and prioritize your safety above all else.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Safety Categories */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Comprehensive Safety Guide
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Essential safety practices organized by category for easy reference.
            </p>
          </div>

          <div className="space-y-16">
            {safetyCategories.map((category, categoryIndex) => {
              const Icon = category.icon;
              
              return (
                <div key={categoryIndex}>
                  <div className="flex items-center gap-4 mb-8">
                    <div className={`w-16 h-16 rounded-2xl ${category.bgColor} flex items-center justify-center`}>
                      <Icon className={`h-8 w-8 ${category.color}`} />
                    </div>
                    <h3 className="text-3xl font-bold text-gray-900">
                      {category.title}
                    </h3>
                  </div>
                  
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {category.tips.map((tip, tipIndex) => (
                      <Card 
                        key={tipIndex} 
                        className={`border-l-4 ${getImportanceColor(tip.importance)}`}
                        hover
                      >
                        <div className="flex items-start justify-between mb-3">
                          <h4 className="text-lg font-semibold text-gray-900">
                            {tip.title}
                          </h4>
                          <Badge 
                            variant={tip.importance === 'high' ? 'danger' : 'warning'} 
                            size="sm"
                            pill
                          >
                            {tip.importance}
                          </Badge>
                        </div>
                        <p className="text-gray-600">
                          {tip.description}
                        </p>
                      </Card>
                    ))}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Red Flags Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Red Flags to Watch For
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Warning signs that indicate potential danger or deception.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {redFlags.map((flag, index) => (
              <Card key={index} className="h-full" hover>
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-start gap-3">
                    <Flag className="h-5 w-5 text-red-600 flex-shrink-0 mt-0.5" />
                    <h3 className="text-lg font-semibold text-gray-900">
                      {flag.flag}
                    </h3>
                  </div>
                  <Badge 
                    variant={flag.severity === 'high' ? 'danger' : 'warning'} 
                    size="sm"
                    pill
                  >
                    {flag.severity}
                  </Badge>
                </div>
                <p className="text-gray-600">
                  {flag.description}
                </p>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Emergency Plan Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Create Your Safety Plan
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Follow these steps to create a comprehensive safety plan for every date.
            </p>
          </div>

          <div className="max-w-4xl mx-auto">
            <div className="space-y-8">
              {emergencyPlan.map((step, index) => {
                const Icon = step.icon;
                
                return (
                  <div key={index} className="flex items-start gap-6">
                    <div className="flex-shrink-0">
                      <div className="flex items-center justify-center w-12 h-12 bg-primary-600 text-white rounded-full font-bold text-lg">
                        {step.step}
                      </div>
                    </div>
                    <Card className="flex-grow">
                      <div className="flex items-start gap-4">
                        <Icon className="h-6 w-6 text-primary-600 flex-shrink-0 mt-1" />
                        <div>
                          <h3 className="text-xl font-semibold text-gray-900 mb-2">
                            {step.title}
                          </h3>
                          <p className="text-gray-600">
                            {step.description}
                          </p>
                        </div>
                      </div>
                    </Card>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      </section>

      {/* LGBTQ+ Resources Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              LGBTQ+ Safety Resources
            </h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Important contacts and resources for LGBTQ+ community members.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto">
            {lgbtqResources.map((resource, index) => (
              <Card key={index} hover className="h-full">
                <div className="flex items-start gap-4">
                  <div className="w-12 h-12 bg-pride-red rounded-full flex items-center justify-center">
                    <Heart className="h-6 w-6 text-white" />
                  </div>
                  <div className="flex-grow">
                    <h3 className="text-lg font-semibold text-gray-900 mb-2">
                      {resource.name}
                    </h3>
                    <p className="text-gray-600 mb-3">
                      {resource.description}
                    </p>
                    <div className="space-y-1 text-sm">
                      <div className="flex items-center gap-2">
                        <Phone className="h-4 w-4 text-gray-400" />
                        <span className="text-gray-700">{resource.contact}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <Globe className="h-4 w-4 text-gray-400" />
                        <span className="text-blue-600">{resource.website}</span>
                      </div>
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
            Ready for AI-Powered Protection?
          </h2>
          
          <p className="text-xl lg:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
            These tips are essential, but Guardr's AI can automate many safety checks for you.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button 
              variant="secondary" 
              size="xl"
              className="text-lg px-8 py-4"
              icon={ArrowRight}
              iconPosition="right"
            >
              Get AI Protection ($6.99/mo)
            </Button>
            <Button 
              variant="outline" 
              size="xl"
              className="text-lg px-8 py-4 border-white text-white hover:bg-white hover:text-primary-600"
            >
              Learn How Guardr Works
            </Button>
          </div>

          <div className="mt-12 text-sm opacity-75">
            Remember: You are an adult. You decide. Trust your instincts and prioritize your safety.
          </div>
        </div>
      </section>
    </div>
  );
}