'use client';

import { 
  Shield, 
  Heart, 
  Users, 
  ArrowRight,
  Lock,
  Globe
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';

export default function About() {
  const team = [
    {
      name: 'Daniel Obregón',
      role: 'Founder & CEO',
      bio: 'Former cybersecurity engineer turned dating safety advocate after experiencing online harassment in the LGBTQ+ community.',
      image: '/api/placeholder/200/200',
      expertise: ['Cybersecurity', 'AI/ML', 'LGBTQ+ Advocacy']
    },
    {
      name: 'Midnight',
      role: 'Chief Safety Officer (Mascot)',
      bio: 'A loyal boxer dog with a rainbow collar who reminds us that protection should come from love, not fear.',
      image: '/api/placeholder/200/200',
      expertise: ['Emotional Support', 'Security Awareness', 'Community Building']
    }
  ];

  const values = [
    {
      title: 'LGBTQ+ First',
      description: 'Every feature is designed with LGBTQ+ safety needs at the forefront.',
      icon: Heart,
      color: 'text-pink-600'
    },
    {
      title: 'Privacy by Design',
      description: 'Zero-knowledge architecture means we never store your personal data.',
      icon: Lock,
      color: 'text-blue-600'
    },
    {
      title: 'Community Driven',
      description: 'Built by and for the community, with continuous feedback integration.',
      icon: Users,
      color: 'text-green-600'
    },
    {
      title: 'Transparency',
      description: 'Open about our methods, pricing, and the real statistics we\'re addressing.',
      icon: Globe,
      color: 'text-purple-600'
    }
  ];

  const milestones = [
    {
      year: '2023',
      title: 'The Catalyst',
      description: 'After a close friend became a victim of an elaborate catfishing scheme that led to financial and emotional damage, I realized existing dating safety tools weren\'t enough.'
    },
    {
      year: '2024',
      title: 'Research & Development',
      description: 'Spent months researching LGBTQ+ dating safety statistics, interviewing community members, and developing the AI algorithms that power Guardr.'
    },
    {
      year: '2024',
      title: 'Beta Launch',
      description: 'Launched with 100 beta users from the LGBTQ+ community. The feedback was overwhelming - 96% said they felt safer dating with Guardr.'
    },
    {
      year: '2025',
      title: 'Full Launch',
      description: 'Launching Guardr publicly with the mission to protect 1 million safer daters by 2026.'
    }
  ];

  const stats = [
    {
      number: '69%',
      label: 'LGBTQ+ harassment rate',
      description: 'The statistic that drives our pricing and mission'
    },
    {
      number: '1,000+',
      label: 'Beta users protected',
      description: 'Early adopters who helped shape Guardr'
    },
    {
      number: '99.2%',
      label: 'AI accuracy rate',
      description: 'Continuously improving through community feedback'
    },
    {
      number: '$6.99',
      label: 'Monthly investment',
      description: 'Affordable protection for everyone who needs it'
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
              Our <span className="gradient-text">Story</span>
            </h1>
            
            <p className="text-xl lg:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
              Born from personal experience, built for community protection, powered by love and technology.
            </p>

            <div className="flex justify-center mb-16">
              <div className="flex items-center gap-4 bg-white rounded-full px-6 py-3 shadow-lg">
                <div className="w-12 h-12 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-full flex items-center justify-center">
                  <Shield className="h-6 w-6 text-white" />
                </div>
                <div className="text-left">
                  <div className="text-sm text-gray-500">Our Mission</div>
                  <div className="font-semibold text-gray-900">"You are an adult. You decide."</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Founder Story */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-16">
              <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
                Why I Built Guardr
              </h2>
              <p className="text-xl text-gray-600">
                A personal story about turning pain into protection.
              </p>
            </div>

            <Card className="p-8 lg:p-12 mb-16">
              <div className="prose prose-lg max-w-none">
                <p className="text-gray-700 mb-6">
                  My name is Daniel, and I'm a cybersecurity engineer who happened to be gay in an era where online dating 
                  became the primary way to meet people. Like many in the LGBTQ+ community, I thought I was being careful. 
                  I thought my technical background would protect me.
                </p>
                
                <p className="text-gray-700 mb-6">
                  I was wrong.
                </p>
                
                <p className="text-gray-700 mb-6">
                  In 2023, a close friend fell victim to an elaborate catfishing scheme. It wasn't just emotional manipulation—
                  it involved identity theft, financial fraud, and months of psychological recovery. The tools available to 
                  verify someone's identity were either invasive, expensive, or simply didn't work.
                </p>
                
                <p className="text-gray-700 mb-6">
                  That's when I learned the statistic that changed everything: <strong>69% of LGBTQ+ individuals experience 
                  harassment in online dating</strong>. Not 6.9%. Not 0.69%. Sixty-nine percent. More than two-thirds of our 
                  community faces danger just for trying to find love.
                </p>
                
                <p className="text-gray-700 mb-6">
                  I realized that my cybersecurity skills, combined with modern AI technology, could create something better. 
                  Not just another background check service, but a comprehensive safety platform designed specifically for 
                  the unique challenges LGBTQ+ people face in dating.
                </p>
                
                <p className="text-gray-700">
                  Guardr isn't just a product—it's my contribution to a community that deserves to love and be loved safely. 
                  The $6.99 price isn't arbitrary; it's a constant reminder of why this protection is essential.
                </p>
              </div>
            </Card>

            <div className="text-center">
              <blockquote className="text-2xl font-medium text-gray-900 mb-4">
                "Technology should protect the vulnerable, not exploit them."
              </blockquote>
              <p className="text-gray-600">- Daniel Obregón, Founder</p>
            </div>
          </div>
        </div>
      </section>

      {/* Team Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Meet Our Team
            </h2>
            <p className="text-xl text-gray-600">
              A small but passionate team dedicated to dating safety.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
            {team.map((member, index) => (
              <Card key={index} hover className="text-center h-full">
                <div className="w-32 h-32 bg-gradient-to-br from-primary-400 to-secondary-400 rounded-full flex items-center justify-center mx-auto mb-6">
                  <span className="text-white text-3xl font-bold">
                    {member.name.split(' ').map(n => n[0]).join('')}
                  </span>
                </div>
                
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  {member.name}
                </h3>
                
                <Badge variant="primary" size="sm" pill className="mb-4">
                  {member.role}
                </Badge>
                
                <p className="text-gray-600 mb-4">
                  {member.bio}
                </p>
                
                <div className="flex flex-wrap justify-center gap-2">
                  {member.expertise.map((skill, skillIndex) => (
                    <Badge key={skillIndex} variant="secondary" size="sm">
                      {skill}
                    </Badge>
                  ))}
                </div>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Values Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Our Values
            </h2>
            <p className="text-xl text-gray-600">
              The principles that guide every decision we make.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {values.map((value, index) => {
              const Icon = value.icon;
              
              return (
                <Card key={index} hover className="h-full">
                  <div className="flex items-start gap-4">
                    <div className={`w-12 h-12 rounded-lg bg-gray-100 flex items-center justify-center ${value.color}`}>
                      <Icon className="h-6 w-6" />
                    </div>
                    <div>
                      <h3 className="text-xl font-semibold text-gray-900 mb-2">
                        {value.title}
                      </h3>
                      <p className="text-gray-600">
                        {value.description}
                      </p>
                    </div>
                  </div>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* Timeline Section */}
      <section className="py-20 lg:py-32 bg-gray-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Our Journey
            </h2>
            <p className="text-xl text-gray-600">
              From personal crisis to community protection.
            </p>
          </div>

          <div className="max-w-4xl mx-auto">
            <div className="space-y-8">
              {milestones.map((milestone, index) => (
                <div key={index} className="flex items-start gap-8">
                  <div className="flex-shrink-0">
                    <div className="w-16 h-16 bg-primary-600 text-white rounded-full flex items-center justify-center font-bold">
                      {milestone.year}
                    </div>
                  </div>
                  <Card className="flex-grow">
                    <h3 className="text-xl font-semibold text-gray-900 mb-2">
                      {milestone.title}
                    </h3>
                    <p className="text-gray-600">
                      {milestone.description}
                    </p>
                  </Card>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-20 lg:py-32 bg-white">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold text-gray-900 mb-6">
              Impact by the Numbers
            </h2>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
            {stats.map((stat, index) => (
              <Card key={index} className="text-center" hover>
                <div className="text-4xl font-bold text-primary-600 mb-2">
                  {stat.number}
                </div>
                <h3 className="text-lg font-semibold text-gray-900 mb-2">
                  {stat.label}
                </h3>
                <p className="text-sm text-gray-600">
                  {stat.description}
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
            Join Our Mission
          </h2>
          
          <p className="text-xl lg:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
            Help us create a world where everyone can date safely and authentically.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button 
              variant="secondary" 
              size="xl"
              className="text-lg px-8 py-4"
              icon={ArrowRight}
              iconPosition="right"
            >
              Start Protecting Yourself
            </Button>
            <Button 
              variant="outline" 
              size="xl"
              className="text-lg px-8 py-4 border-white text-white hover:bg-white hover:text-primary-600"
            >
              Contact Us
            </Button>
          </div>

          <div className="mt-12 text-sm opacity-75">
            Because you are an adult. You decide. And you deserve to feel safe.
          </div>
        </div>
      </section>
    </div>
  );
}