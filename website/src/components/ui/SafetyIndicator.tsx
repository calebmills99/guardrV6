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
      color: 'text-green-600',
      bgColor: 'bg-green-100',
      borderColor: 'border-green-300',
      pulseColor: 'bg-green-400',
    },
    medium: {
      icon: Shield,
      label: 'Medium Safety', 
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-100',
      borderColor: 'border-yellow-300',
      pulseColor: 'bg-yellow-400',
    },
    low: {
      icon: AlertTriangle,
      label: 'Low Safety',
      color: 'text-orange-600',
      bgColor: 'bg-orange-100',
      borderColor: 'border-orange-300',
      pulseColor: 'bg-orange-400',
    },
    danger: {
      icon: XCircle,
      label: 'High Risk',
      color: 'text-red-600',
      bgColor: 'bg-red-100',
      borderColor: 'border-red-300',
      pulseColor: 'bg-red-400',
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
        'inline-flex items-center gap-2 rounded-full border transition-all duration-200',
        currentConfig.bgColor,
        currentConfig.borderColor,
        currentSize.container,
        'relative overflow-hidden',
        className
      )}
    >
      {/* Pulse animation background */}
      <div
        className={cn(
          'absolute inset-0 rounded-full opacity-20 safety-pulse',
          currentConfig.pulseColor
        )}
      />
      
      {/* Content */}
      <div className="relative flex items-center gap-2">
        <Icon
          size={currentSize.icon}
          className={cn(currentConfig.color)}
        />
        
        {showLabel && (
          <span className={cn(currentConfig.color, currentSize.text, 'font-medium')}>
            {currentConfig.label}
          </span>
        )}
        
        {showScore && score !== undefined && (
          <span className={cn(currentConfig.color, currentSize.score)}>
            {score}%
          </span>
        )}
      </div>
    </div>
  );
};

export default SafetyIndicator;