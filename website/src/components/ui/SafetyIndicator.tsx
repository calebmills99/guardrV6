'use client';

import { cn } from '@/lib/utils';
import { Shield, AlertTriangle, XCircle, CheckCircle } from 'lucide-react';

interface SafetyIndicatorProps {
  level: 'high' | 'medium' | 'low' | 'danger';
  score?: number;
  size?: 'sm' | 'md' | 'lg';
  showScore?: boolean;
  showLabel?: boolean;
  className?: string;
}

const SafetyIndicator: React.FC<SafetyIndicatorProps> = ({
  level,
  score,
  size = 'md',
  showScore = true,
  showLabel = true,
  className,
}) => {
  const config = {
    high: {
      icon: CheckCircle,
      label: 'High Safety',
      textColor: 'text-emerald-200',
      glow: 'shadow-[0_0_30px_rgba(16,185,129,0.45)]',
      bgColor: 'bg-emerald-500/10',
      borderColor: 'border-emerald-300/40',
    },
    medium: {
      icon: Shield,
      label: 'Medium Safety',
      textColor: 'text-warning-100',
      glow: 'shadow-[0_0_30px_rgba(245,158,11,0.45)]',
      bgColor: 'bg-warning-500/10',
      borderColor: 'border-warning-500/40',
    },
    low: {
      icon: AlertTriangle,
      label: 'Low Safety',
      textColor: 'text-orange-200',
      glow: 'shadow-[0_0_30px_rgba(249,115,22,0.45)]',
      bgColor: 'bg-orange-500/10',
      borderColor: 'border-orange-400/40',
    },
    danger: {
      icon: XCircle,
      label: 'High Risk',
      textColor: 'text-danger-100',
      glow: 'shadow-[0_0_30px_rgba(239,68,68,0.45)]',
      bgColor: 'bg-danger-500/10',
      borderColor: 'border-danger-500/40',
    },
  };

  const sizeConfig = {
    sm: {
      container: 'px-2 py-1',
      icon: 16,
      text: 'text-xs',
      score: 'text-xs font-medium',
    },
    md: {
      container: 'px-3 py-2',
      icon: 20,
      text: 'text-sm',
      score: 'text-sm font-semibold',
    },
    lg: {
      container: 'px-4 py-3',
      icon: 24,
      text: 'text-base',
      score: 'text-base font-semibold',
    },
  };

  const currentConfig = config[level];
  const currentSize = sizeConfig[size];
  const Icon = currentConfig.icon;

  return (
    <div
      className={cn(
        'inline-flex items-center gap-2 rounded-full border transition-all duration-200 relative overflow-hidden backdrop-blur-lg',
        'bg-gradient-to-r from-white/5 via-transparent to-white/5',
        currentConfig.bgColor,
        currentConfig.borderColor,
        currentConfig.glow,
        currentSize.container,
        className
      )}
    >
      <div className="absolute inset-0 opacity-30 mix-blend-screen bg-white/5" />

      <div className="relative flex items-center gap-2">
        <Icon size={currentSize.icon} className={cn(currentConfig.textColor)} />

        {showLabel && (
          <span className={cn(currentConfig.textColor, currentSize.text, 'font-semibold tracking-wide')}>
            {currentConfig.label}
          </span>
        )}

        {showScore && score !== undefined && (
          <span className={cn(currentConfig.textColor, currentSize.score)}>
            {score}%
          </span>
        )}
      </div>
    </div>
  );
};

export default SafetyIndicator;