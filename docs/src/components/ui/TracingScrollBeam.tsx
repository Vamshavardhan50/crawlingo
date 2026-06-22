import { useEffect, useRef, useState } from 'react';
import { motion, useScroll, useSpring, useTransform } from 'framer-motion';
import { cn } from '@/lib/utils';

export default function TracingScrollBeam({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ['start start', 'end end'],
  });

  const [svgHeight, setSvgHeight] = useState(0);

  useEffect(() => {
    const handleResize = () => {
      if (ref.current) {
        setSvgHeight(ref.current.offsetHeight);
      }
    };

    handleResize();
    window.addEventListener('resize', handleResize);

    // Also run on a short delay to account for dynamic content loading
    const timer = setTimeout(handleResize, 500);

    return () => {
      window.removeEventListener('resize', handleResize);
      clearTimeout(timer);
    };
  }, [children]);

  const ySpring = useSpring(scrollYProgress, {
    stiffness: 150,
    damping: 35,
    restDelta: 0.001,
  });

  const yCoordinate = useTransform(ySpring, [0, 1], [0, svgHeight]);

  return (
    <div ref={ref} className={cn('relative w-full', className)}>
      <div className="absolute -left-6 md:-left-10 top-0 bottom-0 hidden lg:block select-none w-6">
        <svg
          width="20"
          height={svgHeight}
          viewBox={`0 0 20 ${svgHeight}`}
          className="ml-4 block"
          style={{ height: '100%' }}
          aria-hidden="true"
        >
          {/* Main vertical guide line */}
          <path
            d={`M 10 0 L 10 ${svgHeight}`}
            fill="none"
            stroke="rgba(0,0,0,0.06)"
            strokeWidth="1.5"
            className="dark:stroke-white/10"
          />
          {/* Scrolling beam indicator path */}
          <motion.path
            d={`M 10 0 L 10 ${svgHeight}`}
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
            className="text-black dark:text-white"
            strokeDasharray={svgHeight}
            strokeDashoffset={useTransform(ySpring, [0, 1], [svgHeight, 0])}
            transition={{
              duration: 0.1,
            }}
          />
          {/* Glowing dot moving with scroll */}
          <motion.circle
            cx="10"
            cy={yCoordinate}
            r="3.5"
            fill="currentColor"
            className="text-black dark:text-white"
            stroke="currentColor"
            strokeWidth="1"
          />
        </svg>
      </div>
      <div className="w-full">{children}</div>
    </div>
  );
}
