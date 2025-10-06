'use client';

import { forwardRef } from 'react';
import { cn } from '@/lib/utils';
import { LucideIcon } from 'lucide-react';

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helper?: string;
  leftIcon?: LucideIcon;
  rightIcon?: LucideIcon;
  variant?: 'default' | 'ghost';
  inputSize?: 'sm' | 'md' | 'lg';
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      type = 'text',
      label,
      error,
      helper,
      leftIcon: LeftIcon,
      rightIcon: RightIcon,
      variant = 'default',
      inputSize = 'md',
      disabled,
      ...props
    },
    ref
  ) => {
    const inputSizeClasses = {
      sm: 'px-3 py-2 text-sm',
      md: 'px-4 py-3 text-base',
      lg: 'px-5 py-4 text-lg',
    };

    const iconSize = {
      sm: 16,
      md: 20,
      lg: 24,
    };

    const baseInputClasses = cn(
      'w-full rounded-xl border transition-all duration-200 backdrop-blur',
      'focus:outline-none focus:ring-2 focus:ring-primary-200/70 focus:border-primary-300/60',
      'disabled:opacity-50 disabled:cursor-not-allowed',
      {
        'border-white/15 bg-[color:var(--surface-200)]/85 text-white placeholder:text-white/40':
          variant === 'default' && !error,
        'border-danger-500/70 bg-danger-500/10 text-white placeholder:text-danger-200': error,
        'border-white/10 bg-white/10 text-white placeholder:text-white/50': variant === 'ghost' && !error,
        'pl-12': LeftIcon,
        'pr-12': RightIcon,
      },
      inputSizeClasses[inputSize]
    );

    return (
      <div className="w-full">
        {label && (
          <label className="block text-sm font-medium text-white/80 mb-1">
            {label}
          </label>
        )}
        
        <div className="relative">
          {LeftIcon && (
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <LeftIcon size={iconSize[inputSize]} className="text-white/40" />
          </div>
        )}
          
          <input
            type={type}
            className={cn(baseInputClasses, className)}
            disabled={disabled}
            ref={ref}
            {...props}
          />
          
          {RightIcon && (
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
              <RightIcon size={iconSize[inputSize]} className="text-white/40" />
          </div>
        )}
        </div>
        
        {error && (
          <p className="mt-1 text-sm text-danger-500" role="alert">
            {error}
          </p>
        )}

        {helper && !error && (
          <p className="mt-1 text-sm text-white/60">
            {helper}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

export default Input;