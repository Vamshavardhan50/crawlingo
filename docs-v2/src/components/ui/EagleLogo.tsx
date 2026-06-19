import { cn } from '@/lib/utils';

interface EagleLogoProps {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'full' | 'icon';
  className?: string;
}

const sizes = {
  sm: { icon: 24, text: 'text-sm' },
  md: { icon: 32, text: 'text-base' },
  lg: { icon: 40, text: 'text-lg' },
  xl: { icon: 56, text: 'text-2xl' },
};

export default function EagleLogo({ size = 'md', variant = 'icon', className }: EagleLogoProps) {
  const { icon, text } = sizes[size];

  return (
    <div className={cn('flex items-center gap-2.5', className)}>
      <svg
        viewBox="0 0 40 40"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className="shrink-0"
        width={icon}
        height={icon}
      >
        <rect width="40" height="40" rx="10" className="fill-black dark:fill-white" />
        {/* Eagle head - sharp geometric profile */}
        <path
          d="M28 11C28 11 24 11 22 14L18 20L14 18V24L18 26L14 30H17L21 26L25 28L29 24V18L26 15L28 11Z"
          className="fill-white dark:fill-black"
        />
        {/* Eye */}
        <circle cx="26" cy="14.5" r="1.5" className="fill-black dark:fill-white" />
        {/* Beak */}
        <path
          d="M28 11L32 13L28 15"
          stroke="white"
          strokeWidth="1.5"
          strokeLinecap="round"
          strokeLinejoin="round"
          className="stroke-white dark:stroke-black"
        />
      </svg>
      {variant === 'full' && (
        <span className={cn('font-bold tracking-tight text-black dark:text-white', text)}>
          Crawlingo
        </span>
      )}
    </div>
  );
}
