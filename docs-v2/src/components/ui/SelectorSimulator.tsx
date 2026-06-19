import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Check, X, ShieldAlert, Zap, RefreshCw, Layers } from 'lucide-react';
import { cn } from '@/lib/utils';

export default function SelectorSimulator() {
  const [websiteState, setWebsiteState] = useState<'original' | 'redesigned'>('original');
  const [currentStep, setCurrentStep] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    setCurrentStep(0);
    setIsRunning(false);
    setLogs([]);
  }, [websiteState]);

  const runSelector = () => {
    if (isRunning) return;
    setIsRunning(true);
    setCurrentStep(1);
    setLogs([]);

    const steps = [
      {
        delay: 600,
        log: "Initializing Crawlingo Session with Auto-Healing Enabled...",
      },
      {
        delay: 1500,
        log: websiteState === 'original' 
          ? "Querying selector: 'button#submit.btn-primary' -> Found match! [Score: 100%]"
          : "Querying selector: 'button#submit.btn-primary' -> ERROR: Element not found (Selector drifted). Initiating self-healing scan...",
      },
      {
        delay: 2800,
        log: websiteState === 'original'
          ? "Extraction complete. No healing needed."
          : "Scanning DOM candidates: Found 3 button elements. Fetching DOM fingerprints...",
      },
      {
        delay: 4200,
        log: websiteState === 'original'
          ? "Finished."
          : "Computing similarity (Jaro-Winkler + Attributes + DOM depth):\n- Candidate 1 (div#submit-wrapper) -> Score: 32%\n- Candidate 2 (button.btn-secondary) -> Score: 48%\n- Candidate 3 (button#send-btn.btn-primary-new) -> Score: 92% (Threshold: 80%)",
      },
      {
        delay: 5500,
        log: websiteState === 'original'
          ? "Execution successful."
          : "Match found: button#send-btn.btn-primary-new selected. Auto-match self-healing complete! (Saved to selector cache)",
      }
    ];

    steps.forEach((step, idx) => {
      setTimeout(() => {
        setCurrentStep(idx + 1);
        setLogs(prev => [...prev, step.log]);
        if (idx === steps.length - 1) {
          setIsRunning(false);
        }
      }, step.delay);
    });
  };

  return (
    <div className="border border-gray-200 dark:border-white/10 rounded-2xl overflow-hidden bg-white dark:bg-zinc-950 my-8">
      {/* Simulator Header */}
      <div className="px-6 py-4 bg-gray-50 dark:bg-zinc-900/50 border-b border-gray-200 dark:border-white/10 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h3 className="text-sm font-semibold text-black dark:text-white flex items-center gap-2">
            <Zap size={16} className="text-black dark:text-white" />
            Auto-Match Self-Healing Simulator
          </h3>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
            Toggle website state and see how selector drift gets resolved.
          </p>
        </div>

        {/* Website State Switcher */}
        <div className="flex bg-gray-100 dark:bg-black p-1 rounded-xl border border-gray-200 dark:border-white/10 self-start sm:self-auto">
          <button
            onClick={() => setWebsiteState('original')}
            className={cn(
              "px-3 py-1.5 text-xs font-medium rounded-lg transition-all",
              websiteState === 'original'
                ? "bg-white dark:bg-zinc-900 text-black dark:text-white shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:text-black dark:hover:text-white"
            )}
          >
            Original DOM
          </button>
          <button
            onClick={() => setWebsiteState('redesigned')}
            className={cn(
              "px-3 py-1.5 text-xs font-medium rounded-lg transition-all",
              websiteState === 'redesigned'
                ? "bg-white dark:bg-zinc-900 text-black dark:text-white shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:text-black dark:hover:text-white"
            )}
          >
            Redesigned DOM
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 divide-y lg:divide-y-0 lg:divide-x divide-gray-200 dark:divide-white/10">
        {/* Left Side: DOM Visualizations */}
        <div className="p-6 space-y-6">
          <div>
            <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">Target Selector</h4>
            <div className="bg-gray-50 dark:bg-black border border-gray-200 dark:border-white/10 rounded-xl p-3 font-mono text-xs text-gray-800 dark:text-gray-200">
              <code>button#submit.btn-primary</code>
            </div>
          </div>

          <div>
            <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">Live Page HTML structure</h4>
            <div className="bg-gray-50 dark:bg-black border border-gray-200 dark:border-white/10 rounded-xl p-4 font-mono text-xs leading-relaxed text-gray-400">
              {websiteState === 'original' ? (
                <pre>
{`<body>
  <div id="wrapper">
    <div id="submit-wrapper" class="p-4">
      <!-- Target Element -->
      <button 
        id="submit" 
        class="btn btn-primary" 
        data-analytics="save-item"
      >
        Submit Form
      </button>
    </div>
    <button class="btn btn-secondary">Cancel</button>
  </div>
</body>`}
                </pre>
              ) : (
                <pre>
{`<body>
  <div id="wrapper">
    <div id="submit-wrapper" class="p-4">
      <!-- Redesigned Element (ID, Class name changed) -->
      <button 
        id="send-btn" 
        class="button btn-primary-new" 
        data-analytics="save-item"
      >
        Submit Form
      </button>
    </div>
    <button class="btn btn-secondary">Cancel</button>
  </div>
</body>`}
                </pre>
              )}
            </div>
          </div>
        </div>

        {/* Right Side: Execution Console */}
        <div className="p-6 flex flex-col justify-between bg-gray-50/50 dark:bg-black/20">
          <div>
            <div className="flex items-center justify-between mb-4">
              <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Scraper Console Logs</h4>
              <button
                disabled={isRunning}
                onClick={runSelector}
                className="btn-primary py-1.5 px-3 rounded-lg text-xs flex items-center gap-1.5 hover:opacity-90 disabled:opacity-50"
              >
                <RefreshCw size={12} className={cn(isRunning ? "animate-spin" : "")} />
                Run Scraper
              </button>
            </div>

            <div className="bg-black text-zinc-300 rounded-xl p-4 font-mono text-xs h-[240px] overflow-y-auto space-y-2 border border-zinc-800 shadow-inner">
              {logs.length === 0 && (
                <div className="text-zinc-500 italic h-full flex items-center justify-center">
                  Click 'Run Scraper' to test extraction
                </div>
              )}
              {logs.map((log, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, x: -5 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.3 }}
                  className={cn(
                    "whitespace-pre-wrap leading-relaxed",
                    log.includes("ERROR") || log.includes("drifted") ? "text-red-400" : "",
                    log.includes("HEALED") || log.includes("healed") || log.includes("Found match") ? "text-green-400 font-semibold" : ""
                  )}
                >
                  <span className="text-zinc-600 select-none mr-2">&gt;</span>
                  {log}
                </motion.div>
              ))}
            </div>
          </div>

          {/* Visual Indicator of Healing Stage */}
          <div className="mt-6 border-t border-gray-200 dark:border-white/10 pt-4">
            <div className="flex items-center justify-between">
              <span className="text-xs font-medium text-gray-500">Healing Status</span>
              <AnimatePresence mode="wait">
                {currentStep === 0 && (
                  <motion.span
                    key="idle"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="text-xs text-gray-400 bg-gray-100 dark:bg-white/5 px-2 py-0.5 rounded-full"
                  >
                    Idle
                  </motion.span>
                )}
                {currentStep > 0 && currentStep < 5 && (
                  <motion.span
                    key="running"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="text-xs text-yellow-500 bg-yellow-500/10 px-2 py-0.5 rounded-full flex items-center gap-1"
                  >
                    <Layers size={10} className="animate-pulse" /> Running Check
                  </motion.span>
                )}
                {currentStep === 5 && (
                  <motion.span
                    key="success"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className={cn(
                      "text-xs px-2 py-0.5 rounded-full flex items-center gap-1 font-semibold",
                      websiteState === 'original' 
                        ? "text-green-500 bg-green-500/10" 
                        : "text-green-500 bg-green-500/10 border border-green-500/20"
                    )}
                  >
                    {websiteState === 'original' ? (
                      <><Check size={10} /> Match Found</>
                    ) : (
                      <><Zap size={10} /> Healed [Score: 92%]</>
                    )}
                  </motion.span>
                )}
              </AnimatePresence>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
