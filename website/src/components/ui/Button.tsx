'use client';

import { forwardRef } from 'react';
import { cn } from '@/lib/utils';
import { LucideIcon } from 'lucide-react';

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg' | 'xl';
  fullWidth?: boolean;
  loading?: boolean;
  icon?: LucideIcon;
  iconPosition?: 'left' | 'right';
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      variant = 'primary',
      size = 'md',
      fullWidth = false,
      loading = false,
      icon: Icon,
      iconPosition = 'left',
      children,
      disabled,
      ...props
    },
    ref
  ) => {
    const baseClasses = cn(
      'inline-flex items-center justify-center gap-2 rounded-lg font-medium',
      'transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-0 focus:ring-primary-200/60',
      'disabled:opacity-50 disabled:cursor-not-allowed',
      'btn-hover-lift',
      {
        'w-full': fullWidth,
        'cursor-not-allowed': loading || disabled,
      }
    );

    const variantClasses = {
      primary:
        'bg-primary-500 text-white shadow-[0_0_25px_var(--glow-primary)] hover:bg-primary-400 active:bg-primary-600',
      secondary:
        'bg-secondary-500 text-white shadow-[0_0_25px_var(--glow-secondary)] hover:bg-secondary-400 active:bg-secondary-600',
      outline:
        'border border-white/30 bg-transparent text-white hover:bg-white/10 active:bg-white/15',
      ghost:
        'text-white/80 hover:bg-white/10 active:bg-white/15',
      danger:
        'bg-danger-500 text-white hover:bg-danger-700 active:bg-danger-700/90',
    };

    const sizeClasses = {
      sm: 'px-3 py-1.5 text-sm',
      md: 'px-4 py-2 text-sm',
      lg: 'px-6 py-3 text-base',
      xl: 'px-8 py-4 text-lg',
    };

    const iconSize = {
      sm: 16,
      md: 16,
      lg: 20,
      xl: 24,
    };

    return (
      <button
        className={cn(
          baseClasses,
          variantClasses[variant],
          sizeClasses[size],
          className
        )}
        disabled={disabled || loading}
        ref={ref}
        {...props}
      >
        {loading && (
          <svg
            className="animate-spin -ml-1 mr-2 h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        )}
        
        {Icon && iconPosition === 'left' && !loading && (
          <Icon size={iconSize[size]} />
        )}
        
        {children}
        
        {Icon && iconPosition === 'right' && !loading && (
          <Icon size={iconSize[size]} />
        )}
      </button>
    );
  }
);

Button.displayName = 'Button';

export default Button;