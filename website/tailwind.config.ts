import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
        surface: {
          100: '#140b25',
          200: '#1a1130',
          300: '#20163b',
          400: '#2a1f4c',
        },
        night: {
          600: '#34145a',
          700: '#1b0f33',
          800: '#1f0f34',
          900: '#1f0b32',
          950: '#140922',
        },
        // Electric queer nightlife inspired palette
        primary: {
          50: '#f5edff',
          100: '#e6d7ff',
          200: '#ceb1ff',
          300: '#b38aff',
          400: '#9a66ff',
          500: '#804bff',
          600: '#6937e6',
          700: '#5329ba',
          800: '#3d1c87',
          900: '#27115a',
          950: '#170a38',
        },
        secondary: {
          50: '#ffebfb',
          100: '#ffd1f4',
          200: '#ffa4e8',
          300: '#ff76db',
          400: '#ff4ace',
          500: '#ff28c1',
          600: '#e213ab',
          700: '#b50c88',
          800: '#820863',
          900: '#520541',
          950: '#2f0226',
        },
        accent: {
          50: '#e6fbff',
          100: '#c7f5ff',
          200: '#8be9ff',
          300: '#4cd9ff',
          400: '#1ec6ff',
          500: '#00aee6',
          600: '#008ec4',
          700: '#006c96',
          800: '#004a68',
          900: '#00293b',
          950: '#001521',
        },
        success: {
          100: '#d1fae5',
          500: '#10b981',
          700: '#047857',
        },
        warning: {
          100: '#fef3c7',
          500: '#f59e0b',
          700: '#b45309',
        },
        danger: {
          100: '#fee2e2',
          500: '#ef4444',
          700: '#b91c1c',
        },
        // Pride colors for special elements
        pride: {
          red: '#e40303',
          orange: '#ff8c00',
          yellow: '#ffed00',
          green: '#008018',
          blue: '#004cff',
          purple: '#732982',
        },
      },
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        display: ['Cal Sans', 'Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      },
      backgroundImage: {
        'hero-night': 'linear-gradient(135deg, #1f0b32 0%, #140922 48%, #1f0f34 100%)',
        'cta-flare': 'linear-gradient(135deg, #1b0f33 0%, #34145a 60%, #ff28c1 100%)',
      },
      blur: {
        hero: '180px',
        orb: '200px',
      },
      boxShadow: {
        'glow-primary': '0 0 25px var(--glow-primary)',
        'glow-primary-lg': '0 0 45px var(--glow-primary)',
        'glow-secondary': '0 0 25px var(--glow-secondary)',
        'glow-neutral': '0 0 20px rgba(255, 255, 255, 0.35)',
        'glow-neutral-strong': '0 0 25px rgba(255, 255, 255, 0.45)',
        'glow-accent': '0 0 12px rgba(255, 40, 193, 0.6)',
        'glow-neutral-sm': '0 0 12px rgba(255, 255, 255, 0.35)',
        'feature-icon': '0 0 35px rgba(255, 255, 255, 0.15)',
        'card-base': '0 25px 60px rgba(8, 3, 20, 0.55)',
        'card-glass': '0 35px 80px rgba(10, 4, 30, 0.55)',
        'card-bordered': '0 15px 45px rgba(6, 2, 18, 0.4)',
        'card-elevated': '0 40px 90px rgba(12, 4, 28, 0.6)',
        'safety-high': '0 0 30px rgba(16, 185, 129, 0.45)',
        'safety-medium': '0 0 30px rgba(245, 158, 11, 0.45)',
        'safety-elevated': '0 0 30px rgba(249, 115, 22, 0.45)',
        'safety-critical': '0 0 30px rgba(239, 68, 68, 0.45)',
      },
      dropShadow: {
        'pink-glow': '0 0 18px rgba(255, 72, 206, 0.5)',
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.5s ease-out',
        'float': 'float 6s ease-in-out infinite',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
};
export default config;