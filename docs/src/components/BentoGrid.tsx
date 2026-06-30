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
        'grid grid-cols-1 md:grid-cols-3 gap-6 max-w-7xl mx-auto my-8',
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
      whileHover={{ y: -6, scale: 1.01 }}
      transition={{ type: "spring", stiffness: 300, damping: 20 }}
      className={cn(
        'row-span-1 rounded-2xl group/bento transition duration-300 p-6 flex flex-col justify-between space-y-4 relative overflow-hidden',
        // Light Mode styling
        'bg-white/80 border border-gray-100 hover:border-brand-violet/20 hover:shadow-[0_12px_30px_-10px_rgba(168,85,247,0.15)] shadow-[0_2px_8px_rgba(0,0,0,0.04)]',
        // Dark Mode styling
        'dark:bg-zinc-950/70 dark:backdrop-blur-xl dark:border-white/5 dark:hover:border-brand-violet/30 dark:hover:shadow-[0_12px_40px_-15px_rgba(168,85,247,0.3)]',
        className
      )}
    >
      {/* Background glowing layer on hover */}
      <div className="absolute inset-0 bg-radial from-brand-violet/5 via-transparent to-transparent opacity-0 group-hover/bento:opacity-100 transition-opacity duration-500 pointer-events-none" />
      
      {header && (
        <div className="flex flex-1 w-full min-h-[8rem] rounded-xl overflow-hidden bg-gray-50/50 dark:bg-black/40 border border-gray-100/80 dark:border-white/5 relative z-10 transition-colors duration-300 group-hover/bento:border-brand-violet/10">
          {header}
        </div>
      )}
      
      <div className={cn('relative z-10 flex flex-col justify-end transition duration-300 group-hover/bento:translate-x-0.5', classNameContent)}>
        {icon && (
          <div className="w-10 h-10 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/10 text-black dark:text-white flex items-center justify-center mb-4 transition-all duration-300 group-hover/bento:scale-110 group-hover/bento:border-brand-violet/30 group-hover/bento:bg-brand-violet/5">
            <span className="text-brand-violet dark:text-brand-violet transition-colors duration-300 group-hover/bento:text-brand-violet">
              {icon}
            </span>
          </div>
        )}
        
        <div className="font-title font-bold text-gray-900 dark:text-white mb-2 text-base tracking-tight transition-colors duration-300 group-hover/bento:text-brand-violet">
          {title}
        </div>
        
        <div className="text-gray-500 dark:text-gray-400 text-xs leading-relaxed font-normal">
          {description}
        </div>
      </div>
    </motion.div>
  );
}
