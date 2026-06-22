import { cn } from '@/lib/utils';

interface EagleLogoProps {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'full' | 'icon';
  className?: string;
}

const sizes = {
  sm: 24,
  md: 32,
  lg: 40,
  xl: 56,
};

export default function EagleLogo({ size = 'md', variant = 'icon', className }: EagleLogoProps) {
  const iconSize = sizes[size];

  return (
    <div className={cn('flex items-center gap-2.5', className)}>
      <img
        src="/logo.svg"
        alt="Crawlingo Logo"
        width={iconSize}
        height={iconSize}
        className="rounded-lg object-contain shrink-0"
      />
      {variant === 'full' && (
        <span className={cn('font-bold tracking-tight text-black dark:text-white', size === 'sm' ? 'text-sm' : size === 'md' ? 'text-base' : size === 'lg' ? 'text-lg' : 'text-2xl')}>
          Crawlingo
        </span>
      )}
    </div>
  );
}
