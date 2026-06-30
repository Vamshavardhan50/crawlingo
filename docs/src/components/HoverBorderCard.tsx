"use client";

import React from 'react';
import { motion, useMotionValue, useMotionTemplate } from 'framer-motion';
import { cn } from '@/lib/cn';

interface HoverBorderCardProps {
  children: React.ReactNode;
  className?: string;
  containerClassName?: string;
  onClick?: () => void;
  glowColor?: 'primary' | 'secondary' | 'accent' | 'default';
}

const colorMaps = {
  primary: {
    light: 'rgba(13, 148, 136, 0.06)',
    dark: 'rgba(13, 148, 136, 0.12)',
    borderLight: 'rgba(13, 148, 136, 0.2)',
    borderDark: 'rgba(13, 148, 136, 0.35)',
  },
  secondary: {
    light: 'rgba(100, 116, 139, 0.06)',
    dark: 'rgba(100, 116, 139, 0.12)',
    borderLight: 'rgba(100, 116, 139, 0.2)',
    borderDark: 'rgba(100, 116, 139, 0.35)',
  },
  accent: {
    light: 'rgba(45, 212, 191, 0.06)',
    dark: 'rgba(45, 212, 191, 0.12)',
    borderLight: 'rgba(45, 212, 191, 0.2)',
    borderDark: 'rgba(45, 212, 191, 0.35)',
  },
  default: {
    light: 'rgba(0, 0, 0, 0.03)',
    dark: 'rgba(255, 255, 255, 0.05)',
    borderLight: 'rgba(0, 0, 0, 0.08)',
    borderDark: 'rgba(255, 255, 255, 0.1)',
  },
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
        'group/card relative rounded-xl transition-all duration-300 overflow-hidden',
        'bg-card border border-border shadow-sm hover:shadow-md',
        'dark:hover:shadow-[0_8px_30px_rgba(0,0,0,0.3)]',
        onClick ? 'cursor-pointer' : '',
        containerClassName
      )}
    >
      <motion.div
        className="pointer-events-none absolute -inset-px rounded-xl opacity-0 group-hover/card:opacity-100 transition-opacity duration-300 dark:hidden"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              250px circle at ${mouseX}px ${mouseY}px,
              ${colors.light},
              transparent 80%
            )
          `,
          border: `1px solid ${colors.borderLight}`,
        }}
      />

      <motion.div
        className="pointer-events-none absolute -inset-px rounded-xl opacity-0 group-hover/card:opacity-100 transition-opacity duration-300 hidden dark:block"
        style={{
          background: useMotionTemplate`
            radial-gradient(
              250px circle at ${mouseX}px ${mouseY}px,
              ${colors.dark},
              transparent 80%
            )
          `,
          border: `1px solid ${colors.borderDark}`,
        }}
      />

      <div className={cn('relative z-10 p-5', className)}>{children}</div>
    </div>
  );
}
