'use client';

import { forwardRef } from 'react';
import { cn } from '@/lib/utils';

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'glass' | 'bordered' | 'elevated';
  padding?: 'none' | 'sm' | 'md' | 'lg' | 'xl';
  hover?: boolean;
}

const Card = forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      variant = 'default',
      padding = 'md',
      hover = false,
      children,
      ...props
    },
    ref
  ) => {
    const baseClasses = cn(
      'rounded-2xl transition-all duration-200 border border-white/5 backdrop-blur',
      {
        'hover:shadow-lg hover:-translate-y-1': hover,
      }
    );

    const variantClasses = {
      default:
        'bg-[color:var(--surface-200)]/90 text-white shadow-[0_25px_60px_rgba(8,3,20,0.55)]',
      glass: 'glass text-white shadow-[0_35px_80px_rgba(10,4,30,0.55)]',
      bordered:
        'bg-[color:var(--surface-100)]/85 text-white border border-white/10 shadow-[0_15px_45px_rgba(6,2,18,0.4)]',
      elevated:
        'bg-[color:var(--surface-300)]/95 text-white shadow-[0_40px_90px_rgba(12,4,28,0.6)]',
    };

    const paddingClasses = {
      none: '',
      sm: 'p-4',
      md: 'p-6',
      lg: 'p-8',
      xl: 'p-10',
    };

    return (
      <div
        className={cn(
          baseClasses,
          variantClasses[variant],
          paddingClasses[padding],
          className
        )}
        ref={ref}
        {...props}
      >
        {children}
      </div>
    );
  }
);

Card.displayName = 'Card';

export default Card;