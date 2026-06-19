import React from 'react';
import { cn } from '@/lib/utils';
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
        'grid grid-cols-1 md:grid-cols-3 gap-4 max-w-7xl mx-auto',
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
      transition={{ duration: 0.2 }}
      className={cn(
        'row-span-1 rounded-2xl group/bento hover:shadow-xl transition duration-200 shadow-input dark:shadow-none p-4 dark:bg-zinc-950 dark:border-white/10 bg-white border border-gray-200 justify-between flex flex-col space-y-4 hover:border-black/30 dark:hover:border-white/30',
        className
      )}
    >
      {header && (
        <div className="flex flex-1 w-full min-h-[6rem] rounded-xl overflow-hidden bg-gray-50 dark:bg-black border border-gray-100 dark:border-white/5">
          {header}
        </div>
      )}
      <div className={cn('transition duration-200 group-hover/bento:translate-x-1', classNameContent)}>
        <div className="w-8 h-8 rounded-lg bg-black dark:bg-white text-white dark:text-black flex items-center justify-center mb-3 group-hover/bento:scale-105 duration-200">
          {icon}
        </div>
        <div className="font-semibold text-black dark:text-white mb-1.5 text-sm">
          {title}
        </div>
        <div className="text-gray-500 dark:text-gray-400 text-xs leading-relaxed">
          {description}
        </div>
      </div>
    </motion.div>
  );
}
