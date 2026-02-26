'use client';

import { useState } from 'react';
import {
  Mail,
  MessageSquare,
  Send,
  Phone,
  Clock,
  Shield,
  Heart,
  ArrowRight,
  ExternalLink,
  AlertTriangle,
  Users,
  Globe,
  HelpCircle,
  Sparkles
} from 'lucide-react';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import Badge from '@/components/ui/Badge';
import Input from '@/components/ui/Input';
import Link from 'next/link';

const contactCards = [
  {
    icon: Mail,
    title: 'Email Us',
    detail: 'support@guardr.app',
    sub: 'For general inquiries and support',
    color: 'text-primary-400',
    href: 'mailto:support@guardr.app',
  },
  {
    icon: Globe,
    title: 'Social Media',
    detail: '@GuardrApp',
    sub: 'Twitter/X, Instagram, TikTok, Reddit',
    color: 'text-secondary-400',
    href: 'https://twitter.com/GuardrApp',
  },
  {
    icon: Clock,
    title: 'Response Time',
    detail: 'Within 24 hours',
    sub: 'We aim to reply as fast as possible',
    color: 'text-accent-400',
    href: null,
  },
];

const crisisResources = [
  {
    name: 'The Trevor Project',
    description: 'Crisis intervention and suicide prevention for LGBTQ+ youth',
    phone: '1-866-488-7386',
    extra: 'Text START to 678-678',
  },
  {
    name: 'Crisis Text Line',
    description: 'Free, 24/7 crisis support via text',
    phone: null,
    extra: 'Text HOME to 741741',
  },
  {
    name: 'Trans Lifeline',
    description: 'Peer support for the trans community',
    phone: '1-877-565-8860',
    extra: null,
  },
];

const subjectOptions = [
  'General Inquiry',
  'Technical Support',
  'Partnership Opportunity',
  'Press & Media',
  'Bug Report',
  'Feature Request',
  'Safety Concern',
];

export default function Contact() {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    subject: 'General Inquiry',
    message: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const mailtoBody = encodeURIComponent(
      `Name: ${formData.name}\nSubject: ${formData.subject}\n\n${formData.message}`
    );
    const mailtoSubject = encodeURIComponent(`[Guardr] ${formData.subject}`);
    window.location.href = `mailto:support@guardr.app?subject=${mailtoSubject}&body=${mailtoBody}`;
  };

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
            <MessageSquare className="w-4 h-4 mr-1 inline" />
            Get in Touch
          </Badge>

          <h1 className="text-5xl lg:text-7xl font-bold mb-6">
            <span className="gradient-text">We&apos;re Here</span>
            <br />
            for You
          </h1>

          <p className="text-xl lg:text-2xl text-white/80 max-w-3xl mx-auto">
            Questions, feedback, partnership ideas, or just want to say hi?
            We&apos;d love to hear from you.
          </p>
        </div>
      </section>

      {/* Contact Info Cards */}
      <section className="py-16 lg:py-20">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid sm:grid-cols-3 gap-6 max-w-4xl mx-auto">
            {contactCards.map((card) => (
              <Card key={card.title} variant="glass" padding="lg" className="text-center">
                <div className="w-14 h-14 rounded-xl bg-surface-300 flex items-center justify-center mx-auto mb-4">
                  <card.icon className={`w-7 h-7 ${card.color}`} />
                </div>
                <h3 className="text-lg font-semibold mb-1">{card.title}</h3>
                {card.href ? (
                  <a
                    href={card.href}
                    className={`${card.color} font-medium hover:underline`}
                    target={card.href.startsWith('http') ? '_blank' : undefined}
                    rel={card.href.startsWith('http') ? 'noopener noreferrer' : undefined}
                  >
                    {card.detail}
                  </a>
                ) : (
                  <p className={`${card.color} font-medium`}>{card.detail}</p>
                )}
                <p className="text-sm text-white/50 mt-1">{card.sub}</p>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Contact Form */}
      <section className="py-16 lg:py-24 bg-surface-200/50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-2xl mx-auto">
            <div className="text-center mb-12">
              <h2 className="text-3xl lg:text-4xl font-bold mb-4">Send Us a Message</h2>
              <p className="text-white/70">
                Fill out the form below and we&apos;ll get back to you as soon as possible.
              </p>
            </div>

            <Card variant="glass" padding="xl">
              <form onSubmit={handleSubmit} className="space-y-6">
                <div className="grid sm:grid-cols-2 gap-6">
                  <Input
                    label="Your Name"
                    placeholder="Alex Johnson"
                    leftIcon={Users}
                    value={formData.name}
                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    required
                  />
                  <Input
                    label="Email Address"
                    type="email"
                    placeholder="alex@example.com"
                    leftIcon={Mail}
                    value={formData.email}
                    onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                    required
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-white/80 mb-2">
                    Subject
                  </label>
                  <select
                    className="w-full rounded-xl bg-surface-300 border border-surface-400 text-white px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                    value={formData.subject}
                    onChange={(e) => setFormData({ ...formData, subject: e.target.value })}
                  >
                    {subjectOptions.map((option) => (
                      <option key={option} value={option}>
                        {option}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-white/80 mb-2">
                    Message
                  </label>
                  <textarea
                    className="w-full rounded-xl bg-surface-300 border border-surface-400 text-white px-4 py-3 h-36 resize-none focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                    placeholder="Tell us what's on your mind..."
                    value={formData.message}
                    onChange={(e) => setFormData({ ...formData, message: e.target.value })}
                    required
                  />
                </div>

                <Button
                  type="submit"
                  variant="primary"
                  size="lg"
                  fullWidth
                  icon={Send}
                  iconPosition="right"
                >
                  Send Message
                </Button>

                <p className="text-xs text-center text-white/40">
                  This opens your default email client with the form details pre-filled.
                </p>
              </form>
            </Card>
          </div>
        </div>
      </section>

      {/* Emergency Resources */}
      <section className="py-20 lg:py-28">
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

      {/* CTA */}
      <section className="py-20 lg:py-32 bg-cta-flare text-white relative overflow-hidden">
        <div className="absolute inset-0 opacity-40 bg-grid-pattern" />
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
          <div className="w-32 h-1 pride-gradient mx-auto mb-8 rounded-full shadow-glow-neutral-strong" />

          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Stay Safe Out There
          </h2>

          <p className="text-xl lg:text-2xl mb-10 max-w-3xl mx-auto opacity-90">
            Check out our safety tips while you wait for a response.
          </p>

          <Link href="/safety-tips">
            <Button
              variant="secondary"
              size="xl"
              className="text-lg px-8 py-4"
              icon={ArrowRight}
              iconPosition="right"
            >
              View Safety Tips
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
