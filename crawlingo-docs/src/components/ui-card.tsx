import React from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import { CheckCircle, Clock, AlertCircle, Terminal, ExternalLink } from 'lucide-react';
import { CopyButton } from '@/components/copy-button';

export function CodeBlock({
  code,
  language = 'bash',
  title,
  fileName,
}: {
  code: string;
  language?: string;
  title?: string;
  fileName?: string;
}) {
  return (
    <Card className="overflow-hidden border-border/50 bg-slate-950/50 backdrop-blur-sm">
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-3">
        <div className="flex items-center gap-2">
          <Terminal className="h-4 w-4 text-indigo-400" />
          {title && <CardTitle className="text-sm font-mono">{title}</CardTitle>}
          {fileName && (
            <Badge variant="secondary" className="text-xs">
              {fileName}
            </Badge>
          )}
        </div>
        <CopyButton text={code} />
      </CardHeader>
      <CardContent className="p-0">
        <pre className="overflow-x-auto p-4 font-mono text-sm text-slate-200">
          <code className={`language-${language}`}>{code}</code>
        </pre>
      </CardContent>
    </Card>
  );
}

export function TerminalExample({
  commands,
  title = "Command",
  description,
}: {
  commands: string[];
  title?: string;
  description?: string;
}) {
  return (
    <Card className="border-border/50 bg-slate-950/50 backdrop-blur-sm">
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-sm">
          <Terminal className="h-4 w-4 text-indigo-400" />
          {title}
        </CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <div className="space-y-2 font-mono text-sm">
          {commands.map((cmd, idx) => (
            <div key={idx} className="flex items-start gap-3">
              <span className="text-slate-500">$&gt;</span>
              <code className="text-slate-200">{cmd}</code>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

export function FeatureCard({
  icon,
  title,
  description,
  gradient = false,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  gradient?: boolean;
}) {
  return (
    <Card
      className={cn(
        'group relative border-border/50 bg-card/50 backdrop-blur-sm transition-all duration-300',
        'hover:scale-[1.02] hover:shadow-lg hover:border-indigo-500/20'
      )}
    >
      <CardHeader>
        <div
          className={cn(
            'flex h-12 w-12 items-center justify-center rounded-lg',
            gradient
              ? 'bg-gradient-to-br from-indigo-500 to-purple-600'
              : 'bg-muted/50'
          )}
        >
          {icon}
        </div>
        <CardTitle className="text-lg">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-muted-foreground">{description}</p>
      </CardContent>
    </Card>
  );
}

export function StatusBadge({
  status,
  text,
}: {
  status: 'success' | 'warning' | 'error' | 'info';
  text: string;
}) {
  const variants = {
    success: 'bg-green-500/10 text-green-400 border-green-500/30',
    warning: 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30',
    error: 'bg-red-500/10 text-red-400 border-red-500/30',
    info: 'bg-blue-500/10 text-blue-400 border-blue-500/30',
  };

  const icons = {
    success: <CheckCircle className="h-3 w-3" />,
    warning: <AlertCircle className="h-3 w-3" />,
    error: <AlertCircle className="h-3 w-3" />,
    info: <Clock className="h-3 w-3" />,
  };

  return (
    <Badge className={cn('border', variants[status])}>
      <span className="flex items-center gap-1">
        {icons[status]}
        {text}
      </span>
    </Badge>
  );
}
