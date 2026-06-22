import React from 'react';
import { motion, useMotionValue, useMotionTemplate } from 'framer-motion';
import { cn } from '@/lib/utils';

interface HoverBorderCardProps {
  children: React.ReactNode;
  className?: string;
  containerClassName?: string;
  onClick?: () => void;
}

export default function HoverBorderCard({
  children,
  className,
  containerClassName,
  onClick,
}: HoverBorderCardProps) {
  const mouseX = useMotionValue(0);
  const mouseY = useMotionValue(0);

  function handleMouseMove({ currentTarget, clientX, clientY }: React.MouseEvent<HTMLDivElement>) {
    const { left, top } = currentTarget.getBoundingClientRect();
    mouseX.set(clientX - left);
    mouseY.set(clientY - top);
  }

  return (
    <div
      onMouseMove={handleMouseMove}
      onClick={onClick}
      className={cn(
        'group/card relative rounded-2xl border border-gray-200 dark:border-white/10 bg-white dark:bg-zinc-950 transition-all duration-300 hover:border-black/30 dark:hover:border-white/30',
        onClick ? 'cursor-pointer' : '',
        containerClassName
      )}
    >
      {/* Light Mode Glow */}
      <motion.div
        className="pointer-events-none absolute -inset-px rounded-2xl opacity-0 group-hover/card:opacity-100 transition duration-300 dark:hidden"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              200px circle at ${mouseX}px ${mouseY}px,
              rgba(0, 0, 0, 0.03),
              transparent 80%
            )
          `,
          border: '1px solid rgba(0, 0, 0, 0.15)',
        }}
      />
      {/* Dark Mode Glow */}
      <motion.div
        className="pointer-events-none absolute -inset-px rounded-2xl opacity-0 group-hover/card:opacity-100 transition duration-300 hidden dark:block"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              200px circle at ${mouseX}px ${mouseY}px,
              rgba(255, 255, 255, 0.08),
              transparent 80%
            )
          `,
          border: '1px solid rgba(255, 255, 255, 0.25)',
        }}
      />
      <div className={cn('relative z-10 p-6', className)}>{children}</div>
    </div>
  );
}
