import React from 'react';
import { cn } from '@/lib/utils';

export function Tabs({ defaultValue, value, onValueChange, children, className }: {
  defaultValue?: string;
  value?: string;
  onValueChange?: (value: string) => void;
  children: React.ReactNode;
  className?: string;
}) {
  const [activeTab, setActiveTab] = React.useState(value || defaultValue || '');
  
  const handleTabChange = (tabValue: string) => {
    setActiveTab(tabValue);
    onValueChange?.(tabValue);
  };

  return (
    <div className={cn('w-full', className)}>
      {React.Children.map(children, (child) => {
        if (React.isValidElement(child) && child.type === TabList) {
          return React.cloneElement(child, {
            activeTab,
            onTabChange: handleTabChange,
          } as any);
        }
        return child;
      })}
      {React.Children.map(children, (child) => {
        if (React.isValidElement(child) && child.type === TabContent) {
          return React.isValidElement(activeTab) && child.props.value === activeTab
            ? child
            : null;
        }
        return child;
      })}
    </div>
  );
}

export function TabList({ children, activeTab, onTabChange, className }: {
  children: React.ReactNode;
  activeTab: string;
  onTabChange: (value: string) => void;
  className?: string;
}) {
  return (
    <div
      className={cn(
        'inline-flex h-10 items-center justify-center rounded-lg bg-muted/50 p-1 text-muted-foreground',
        className
      )}
    >
      {React.Children.map(children, (child) => {
        if (React.isValidElement(child) && child.type === TabTrigger) {
          return React.cloneElement(child, {
            isActive: activeTab === child.props.value,
            onClick: () => child.props.value && onTabChange(child.props.value),
          } as any);
        }
        return child;
      })}
    </div>
  );
}

export function TabTrigger({ children, value, isActive, onClick, className }: {
  children: React.ReactNode;
  value: string;
  isActive: boolean;
  onClick?: () => void;
  className?: string;
}) {
  return (
    <button
      className={cn(
        'inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
        isActive
          ? 'bg-background text-foreground shadow-sm'
          : 'hover:bg-background/50 hover:text-foreground',
        className
      )}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

export function TabContent({ children, value, className }: {
  children: React.ReactNode;
  value: string;
  className?: string;
}) {
  return (
    <div
      className={cn(
        'mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
        className
      )}
    >
      {children}
    </div>
  );
}
