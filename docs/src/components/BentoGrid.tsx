"use client";

import React from 'react';
import { cn } from '@/lib/cn';
import { motion } from 'framer-motion';

export function BentoGrid({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}) {
  return (
    <div
      className={cn(
        'grid grid-cols-1 md:grid-cols-3 gap-4 max-w-6xl mx-auto my-8',
        className
      )}
    >
      {children}
    </div>
  );
}

export function BentoGridItem({
  className,
  title,
  description,
  header,
  icon,
  classNameContent,
}: {
  className?: string;
  title?: string | React.ReactNode;
  description?: string | React.ReactNode;
  header?: React.ReactNode;
  icon?: React.ReactNode;
  classNameContent?: string;
}) {
  return (
    <motion.div
      whileHover={{ y: -4 }}
      transition={{ type: "spring", stiffness: 400, damping: 25 }}
      className={cn(
        'row-span-1 rounded-xl group/bento transition duration-200 p-5 flex flex-col justify-between space-y-3 relative overflow-hidden',
        'bg-card border border-border hover:border-primary/20 shadow-sm hover:shadow-md',
        'dark:hover:shadow-[0_8px_30px_-10px_rgba(13,148,136,0.15)]',
        className
      )}
    >
      {header && (
        <div className="flex flex-1 w-full min-h-[7rem] rounded-lg overflow-hidden bg-secondary/50 dark:bg-surface-900/50 border border-border relative z-10 transition-colors duration-200 group-hover/bento:border-primary/15">
          {header}
        </div>
      )}

      <div className={cn('relative z-10 flex flex-col justify-end transition duration-200', classNameContent)}>
        {icon && (
          <div className="w-8 h-8 rounded-lg bg-secondary dark:bg-surface-800 border border-border text-primary flex items-center justify-center mb-3 transition-all duration-200 group-hover/bento:scale-105 group-hover/bento:border-primary/30">
            {icon}
          </div>
        )}

        <div className="font-title font-semibold text-foreground mb-1 text-sm tracking-tight transition-colors duration-200 group-hover/bento:text-primary">
          {title}
        </div>

        <div className="text-muted-foreground text-xs leading-relaxed">
          {description}
        </div>
      </div>
    </motion.div>
  );
}
