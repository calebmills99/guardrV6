'use client';

import Link from 'next/link';
import { Shield, Heart, Twitter, Instagram, Mail, Github } from 'lucide-react';

const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    product: [
      { name: 'How It Works', href: '/how-it-works' },
      { name: 'Pricing', href: '/pricing' },
      { name: 'Safety Tips', href: '/safety-tips' },
      { name: 'Features', href: '/#features' },
    ],
    company: [
      { name: 'About', href: '/about' },
      { name: 'Contact', href: '/contact' },
      { name: 'Blog', href: '/blog' },
      { name: 'Careers', href: '/careers' },
    ],
    legal: [
      { name: 'Privacy Policy', href: '/privacy' },
      { name: 'Terms of Service', href: '/terms' },
      { name: 'LGBTQ+ Safety', href: '/lgbtq-safety' },
      { name: 'Data Protection', href: '/data-protection' },
    ],
    support: [
      { name: 'Help Center', href: '/help' },
      { name: 'Community', href: '/community' },
      { name: 'Status', href: '/status' },
      { name: 'Report Issue', href: '/report' },
    ],
  };

  const socialLinks = [
    { name: 'Twitter', href: '#', icon: Twitter },
    { name: 'Instagram', href: '#', icon: Instagram },
    { name: 'Email', href: 'mailto:hello@guardr.app', icon: Mail },
    { name: 'GitHub', href: '#', icon: Github },
  ];

  return (
    <footer className="border-t border-white/10 bg-[color:var(--surface-200)]/85 text-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {/* Main Footer Content */}
        <div className="grid grid-cols-2 md:grid-cols-6 gap-8">
          {/* Brand Section */}
          <div className="col-span-2 md:col-span-2">
            <Link href="/" className="flex items-center space-x-2 group mb-4">
              <div className="relative">
                <div className="w-10 h-10 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-xl flex items-center justify-center shadow-glow-primary group-hover:scale-105 transition-transform duration-200">
                  <Shield className="h-6 w-6 text-white" />
                </div>
                <div className="absolute -top-1 -right-1 w-4 h-4 bg-secondary-500 rounded-full shadow-glow-accent">
                  <Heart className="h-3 w-3 text-white absolute top-0.5 left-0.5" />
                </div>
              </div>
              <div>
                <span className="text-xl font-bold text-white">Guardr</span>
                <div className="w-16 h-0.5 pride-gradient rounded-full shadow-glow-neutral-sm"></div>
              </div>
            </Link>

            <p className="text-white/85 text-sm mb-4 max-w-sm">
              AI-powered digital safety for online dating. Built with love for the LGBTQ+ community and all safer daters.
            </p>

            <div className="flex space-x-4">
              {socialLinks.map((social) => {
                const Icon = social.icon;
                return (
                  <Link
                    key={social.name}
                    href={social.href}
                    className="text-white/70 hover:text-white transition-colors duration-200"
                    aria-label={social.name}
                  >
                    <Icon className="h-5 w-5" />
                  </Link>
                );
              })}
            </div>
          </div>

          {/* Product Links */}
          <div>
            <h3 className="text-sm font-semibold text-white mb-4">Product</h3>
            <ul className="space-y-3">
              {footerLinks.product.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-sm text-white/85 hover:text-white transition-colors duration-200"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Company Links */}
          <div>
            <h3 className="text-sm font-semibold text-white mb-4">Company</h3>
            <ul className="space-y-3">
              {footerLinks.company.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-sm text-white/85 hover:text-white transition-colors duration-200"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Legal Links */}
          <div>
            <h3 className="text-sm font-semibold text-white mb-4">Legal</h3>
            <ul className="space-y-3">
              {footerLinks.legal.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-sm text-white/85 hover:text-white transition-colors duration-200"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Support Links */}
          <div>
            <h3 className="text-sm font-semibold text-white mb-4">Support</h3>
            <ul className="space-y-3">
              {footerLinks.support.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-sm text-white/85 hover:text-white transition-colors duration-200"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom Section */}
        <div className="mt-12 pt-8 border-t border-white/10">
          <div className="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
            <div className="flex items-center space-x-2">
              <p className="text-sm text-white/85">
                © {currentYear} Guardr. Built with
              </p>
              <Heart className="h-4 w-4 text-pride-red" />
              <p className="text-sm text-white/85">
                for safer dating.
              </p>
            </div>

            <div className="flex items-center space-x-6">
              <p className="text-xs text-white/75">
                69% harassment rate represents real LGBTQ+ dating dangers
              </p>
              <div className="w-16 h-1 pride-gradient rounded-full opacity-80 shadow-glow-neutral"></div>
            </div>
          </div>

          <div className="mt-4 text-center">
            <p className="text-xs text-white/75">
              &ldquo;You are an adult. You decide.&rdquo; - Empowering safer choices in digital dating.
            </p>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;