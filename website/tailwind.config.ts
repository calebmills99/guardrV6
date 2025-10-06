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