import { Info, AlertTriangle, CheckCircle, XCircle } from 'lucide-react';
import { cn } from '@/lib/utils';

interface CalloutProps {
  type?: 'info' | 'warning' | 'success' | 'error';
  title?: string;
  children: React.ReactNode;
  className?: string;
}

const icons = {
  info: Info,
  warning: AlertTriangle,
  success: CheckCircle,
  error: XCircle,
};

const colors = {
  info: 'text-black dark:text-white',
  warning: 'text-amber-500',
  success: 'text-green-500',
  error: 'text-red-500',
};

export default function Callout({
  type = 'info',
  title,
  children,
  className,
}: CalloutProps) {
  const Icon = icons[type];

  return (
    <div className={cn(`callout callout-${type} transition-all duration-200`, className)}>
      <div className="flex gap-3">
        <Icon size={18} className={cn('mt-0.5 shrink-0', colors[type])} />
        <div className="min-w-0">
          {title && (
            <p className="font-semibold text-black dark:text-white text-sm mb-1">{title}</p>
          )}
          <div className="text-[13px] text-gray-600 dark:text-gray-400 leading-relaxed">
            {children}
          </div>
        </div>
      </div>
    </div>
  );
}
