'use client';

import { cn } from '@/lib/utils';

interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  pill?: boolean;
}

const Badge: React.FC<BadgeProps> = ({
  className,
  variant = 'default',
  size = 'md',
  pill = false,
  children,
  ...props
}) => {
  const variantClasses = {
    default: 'bg-white/10 text-white/80 border-white/15',
    primary: 'bg-primary-500/20 text-primary-100 border-primary-400/30',
    secondary: 'bg-secondary-500/20 text-secondary-100 border-secondary-400/30',
    success: 'bg-success-500/15 text-success-100 border-success-500/40',
    warning: 'bg-warning-500/15 text-warning-100 border-warning-500/40',
    danger: 'bg-danger-500/15 text-danger-100 border-danger-500/40',
  };

  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-2.5 py-1 text-sm',
    lg: 'px-3 py-1.5 text-base',
  };

  return (
    <span
      className={cn(
        'inline-flex items-center font-medium border',
        pill ? 'rounded-full' : 'rounded-md',
        variantClasses[variant],
        sizeClasses[size],
        className
      )}
      {...props}
    >
      {children}
    </span>
  );
};

export default Badge;