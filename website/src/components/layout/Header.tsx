'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import { Menu, X, Shield, Heart } from 'lucide-react';
import Button from '@/components/ui/Button';

const Header: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const pathname = usePathname();

  const navigation = [
    { name: 'How It Works', href: '/how-it-works' },
    { name: 'Pricing', href: '/pricing' },
    { name: 'Safety Tips', href: '/safety-tips' },
    { name: 'About', href: '/about' },
  ];

  const isActive = (href: string) => pathname === href;

  return (
    <header className="sticky top-0 z-50 border-b border-white/10 bg-[rgba(13,7,25,0.85)] backdrop-blur-2xl">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-2 group">
            <div className="relative">
              <div className="w-10 h-10 bg-gradient-to-br from-primary-500 to-secondary-500 rounded-xl flex items-center justify-center shadow-[0_0_25px_var(--glow-primary)] group-hover:scale-105 transition-transform duration-200">
                <Shield className="h-6 w-6 text-white" />
              </div>
              <div className="absolute -top-1 -right-1 w-4 h-4 bg-secondary-500 rounded-full shadow-[0_0_12px_rgba(255,40,193,0.6)]">
                <Heart className="h-3 w-3 text-white absolute top-0.5 left-0.5" />
              </div>
            </div>
            <div className="hidden sm:block">
              <span className="text-xl font-bold text-white">Guardr</span>
              <div className="w-16 h-0.5 pride-gradient rounded-full shadow-[0_0_12px_rgba(255,255,255,0.35)]"></div>
            </div>
          </Link>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex space-x-8">
            {navigation.map((item) => (
              <Link
                key={item.name}
                href={item.href}
                className={cn(
                  'text-sm font-medium transition-colors duration-200 text-white/70 hover:text-white',
                  isActive(item.href)
                    ? 'text-white border-b-2 border-secondary-400 pb-4'
                    : 'text-white/70'
                )}
              >
                {item.name}
              </Link>
            ))}
          </nav>

          {/* Desktop CTA */}
          <div className="hidden md:flex items-center space-x-4">
            <Link
              href="/contact"
              className="text-sm font-medium text-white/70 hover:text-white transition-colors duration-200"
            >
              Contact
            </Link>
            <Button size="sm" className="text-sm">
              Start Protecting ($6.99/mo)
            </Button>
          </div>

          {/* Mobile menu button */}
          <button
            type="button"
            className="md:hidden p-2 rounded-md text-white hover:text-secondary-200 hover:bg-white/10 transition-colors duration-200"
            onClick={() => setIsOpen(!isOpen)}
          >
            <span className="sr-only">Open main menu</span>
            {isOpen ? (
              <X className="h-6 w-6" />
            ) : (
              <Menu className="h-6 w-6" />
            )}
          </button>
        </div>

        {/* Mobile Navigation */}
        {isOpen && (
          <div className="md:hidden border-t border-white/10 py-4 space-y-4">
            {navigation.map((item) => (
              <Link
                key={item.name}
                href={item.href}
                className={cn(
                  'block px-4 py-2 text-base font-medium rounded-md transition-colors duration-200',
                  isActive(item.href)
                    ? 'text-white bg-white/10'
                    : 'text-white/70 hover:text-white hover:bg-white/10'
                )}
                onClick={() => setIsOpen(false)}
              >
                {item.name}
              </Link>
            ))}
            <div className="px-4 pt-4 border-t border-white/10 space-y-2">
              <Link
                href="/contact"
                className="block px-4 py-2 text-base font-medium text-white/70 hover:text-white hover:bg-white/10 rounded-md transition-colors duration-200"
                onClick={() => setIsOpen(false)}
              >
                Contact
              </Link>
              <Button fullWidth size="md">
                Start Protecting ($6.99/mo)
              </Button>
            </div>
          </div>
        )}
      </div>
    </header>
  );
};

export default Header;