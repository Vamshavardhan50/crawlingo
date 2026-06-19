import { useState } from 'react';
import { Copy, Check } from 'lucide-react';
import { cn } from '@/lib/utils';

interface CodeBlockProps {
  code: string;
  language?: string;
  filename?: string;
  showLineNumbers?: boolean;
  className?: string;
}

export default function CodeBlock({
  code,
  language = 'bash',
  filename,
  showLineNumbers = false,
  className,
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(code);
      } else {
        // Fallback for sandboxed frames or non-secure contexts
        const textarea = document.createElement('textarea');
        textarea.value = code;
        textarea.style.position = 'fixed';
        textarea.style.opacity = '0';
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
      }
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy code: ', err);
    }
  };

  const lines = code.split('\n');

  return (
    <div className={cn('code-block my-6 border border-gray-200 dark:border-white/10 overflow-hidden rounded-2xl bg-gray-50/50 dark:bg-black group relative', className)}>
      {/* Code Header Bar */}
      <div className="flex items-center justify-between px-4 py-2.5 bg-gray-50 dark:bg-zinc-900/50 border-b border-gray-200 dark:border-white/10">
        <div className="flex items-center gap-1.5">
          <div className="w-2.5 h-2.5 rounded-full bg-gray-200 dark:bg-zinc-800" />
          <div className="w-2.5 h-2.5 rounded-full bg-gray-200 dark:bg-zinc-800" />
          <div className="w-2.5 h-2.5 rounded-full bg-gray-200 dark:bg-zinc-800" />
          {filename && (
            <span className="text-[11px] font-mono text-gray-500 dark:text-gray-400 ml-2 font-medium">
              {filename}
            </span>
          )}
        </div>

        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 text-[11px] font-medium text-gray-500 dark:text-gray-400 hover:text-black dark:hover:text-white transition-colors"
          title="Copy code to clipboard"
        >
          {copied ? (
            <>
              <Check size={12} className="text-green-500" />
              <span className="text-green-500 font-semibold">Copied!</span>
            </>
          ) : (
            <>
              <Copy size={12} />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>

      <div className="code-content p-4 overflow-x-auto text-[13px] leading-relaxed font-mono bg-white dark:bg-zinc-950 text-gray-800 dark:text-zinc-200">
        {showLineNumbers ? (
          <table className="w-full border-collapse">
            <tbody>
              {lines.map((line, i) => (
                <tr key={i} className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                  <td className="pr-4 text-gray-400 dark:text-zinc-600 select-none text-right w-8 text-[12px] font-mono border-r border-gray-100 dark:border-zinc-900">
                    {i + 1}
                  </td>
                  <td className="pl-4 font-mono text-[13px] whitespace-pre">
                    {line || ' '}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <pre className="whitespace-pre overflow-x-auto">{code}</pre>
        )}
      </div>
    </div>
  );
}
