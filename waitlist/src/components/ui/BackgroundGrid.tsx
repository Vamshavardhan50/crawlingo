import { cn } from '@/lib/utils';

interface BackgroundGridProps {
  children?: React.ReactNode;
  className?: string;
  pattern?: 'grid' | 'dot';
}

export default function BackgroundGrid({
  children,
  className,
  pattern = 'grid',
}: BackgroundGridProps) {
  return (
    <div
      className={cn(
        'relative min-h-screen w-full bg-white dark:bg-black transition-colors duration-300',
        className
      )}
    >
      {/* Background SVG Grid / Dot */}
      <div
        className={cn(
          'absolute inset-0 pointer-events-none opacity-[0.04] dark:opacity-[0.07]',
          pattern === 'grid'
            ? 'bg-[linear-gradient(to_right,#808080_1px,transparent_1px),linear-gradient(to_bottom,#808080_1px,transparent_1px)] bg-[size:40px_40px]'
            : 'bg-[radial-gradient(#808080_1px,transparent_1px)] bg-[size:24px_24px]'
        )}
      />
      {/* Radial Gradient Overlay for fading effect */}
      <div className="absolute inset-0 pointer-events-none bg-[radial-gradient(ellipse_at_center,transparent_30%,#ffffff_90%)] dark:bg-[radial-gradient(ellipse_at_center,transparent_30%,#000000_90%)]" />
      
      <div className="relative z-10">{children}</div>
    </div>
  );
}
