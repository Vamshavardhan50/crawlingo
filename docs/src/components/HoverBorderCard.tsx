"use client";

import React from 'react';
import { motion, useMotionValue, useMotionTemplate } from 'framer-motion';
import { cn } from '@/lib/cn';

interface HoverBorderCardProps {
  children: React.ReactNode;
  className?: string;
  containerClassName?: string;
  onClick?: () => void;
  glowColor?: 'violet' | 'cyan' | 'emerald' | 'rose' | 'default';
}

const colorMaps = {
  violet: {
    light: 'rgba(168, 85, 247, 0.08)',
    dark: 'rgba(168, 85, 247, 0.15)',
    borderLight: 'rgba(168, 85, 247, 0.25)',
    borderDark: 'rgba(168, 85, 247, 0.4)',
  },
  cyan: {
    light: 'rgba(6, 182, 212, 0.08)',
    dark: 'rgba(6, 182, 212, 0.15)',
    borderLight: 'rgba(6, 182, 212, 0.25)',
    borderDark: 'rgba(6, 182, 212, 0.4)',
  },
  emerald: {
    light: 'rgba(16, 185, 129, 0.08)',
    dark: 'rgba(16, 185, 129, 0.15)',
    borderLight: 'rgba(16, 185, 129, 0.25)',
    borderDark: 'rgba(16, 185, 129, 0.4)',
  },
  rose: {
    light: 'rgba(244, 63, 94, 0.08)',
    dark: 'rgba(244, 63, 94, 0.15)',
    borderLight: 'rgba(244, 63, 94, 0.25)',
    borderDark: 'rgba(244, 63, 94, 0.4)',
  },
  default: {
    light: 'rgba(0, 0, 0, 0.04)',
    dark: 'rgba(255, 255, 255, 0.08)',
    borderLight: 'rgba(0, 0, 0, 0.12)',
    borderDark: 'rgba(255, 255, 255, 0.25)',
  }
};

export default function HoverBorderCard({
  children,
  className,
  containerClassName,
  onClick,
  glowColor = 'default',
}: HoverBorderCardProps) {
  const mouseX = useMotionValue(0);
  const mouseY = useMotionValue(0);

  const colors = colorMaps[glowColor];

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
        'group/card relative rounded-2xl transition-all duration-500 overflow-hidden',
        'bg-white/90 border border-gray-150 shadow-[0_2px_10px_rgba(0,0,0,0.03)] hover:shadow-[0_12px_32px_rgba(0,0,0,0.06)] hover:border-gray-300',
        'dark:bg-zinc-950/70 dark:backdrop-blur-xl dark:border-white/5 dark:hover:border-white/10 dark:shadow-none dark:hover:shadow-[0_12px_40px_rgba(0,0,0,0.4)]',
        onClick ? 'cursor-pointer' : '',
        containerClassName
      )}
    >
      {/* Light Mode Glow */}
      <motion.div
        className="pointer-events-none absolute -inset-px rounded-2xl opacity-0 group-hover/card:opacity-100 transition-opacity duration-500 dark:hidden"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              300px circle at ${mouseX}px ${mouseY}px,
              ${colors.light},
              transparent 80%
            )
          `,
          border: `1px solid ${colors.borderLight}`,
        }}
      />
      
      {/* Dark Mode Glow */}
      <motion.div
        className="pointer-events-none absolute -inset-px rounded-2xl opacity-0 group-hover/card:opacity-100 transition-opacity duration-500 hidden dark:block"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              300px circle at ${mouseX}px ${mouseY}px,
              ${colors.dark},
              transparent 80%
            )
          `,
          border: `1px solid ${colors.borderDark}`,
        }}
      />
      
      {/* Inner subtle glow ring */}
      <div className="absolute inset-[1px] rounded-2xl bg-transparent border border-white/40 dark:border-white/5 pointer-events-none z-0" />
      
      <div className={cn('relative z-10 p-6', className)}>{children}</div>
    </div>
  );
}
