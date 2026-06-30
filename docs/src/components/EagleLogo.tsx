import { cn } from '@/lib/cn';

interface EagleLogoProps {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'full' | 'icon';
  className?: string;
}

const sizes = {
  sm: 22,
  md: 28,
  lg: 36,
  xl: 48,
};

export default function EagleLogo({ size = 'md', variant = 'icon', className }: EagleLogoProps) {
  const iconSize = sizes[size];

  return (
    <div className={cn('flex items-center gap-2', className)}>
      <img
        src="/logo.svg"
        alt="Crawlingo"
        width={iconSize}
        height={iconSize}
        className="rounded-md object-contain shrink-0"
      />
      {variant === 'full' && (
        <span
          className={cn(
            'font-bold tracking-tight text-foreground',
            size === 'sm' && 'text-sm',
            size === 'md' && 'text-base',
            size === 'lg' && 'text-lg',
            size === 'xl' && 'text-2xl'
          )}
        >
          Crawlingo
        </span>
      )}
    </div>
  );
}
