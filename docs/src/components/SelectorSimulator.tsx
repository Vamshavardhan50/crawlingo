"use client";

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Check, Zap, Play, RotateCcw, Layers, Terminal, Globe, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/cn';

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
        delay: 1600,
        log: websiteState === 'original' 
          ? "Querying selector: 'button#submit.btn-primary' -> Found exact match! [Confidence Score: 100%]"
          : "Querying selector: 'button#submit.btn-primary' -> WARNING: Selector drift detected. Target node not found. Running self-healing...",
      },
      {
        delay: 3000,
        log: websiteState === 'original'
          ? "Extraction complete. Attributes matches cached signature perfectly."
          : "Crawlingo DOM Parser: Scanning active DOM tree for similar nodes. Discovered 3 candidate buttons...",
      },
      {
        delay: 4400,
        log: websiteState === 'original'
          ? "Process finished successfully."
          : "Comparing fingerprint weights:\n  - Candidate 1 <div id=\"submit-wrapper\"> (Score: 32%)\n  - Candidate 2 <button class=\"btn-secondary\"> (Score: 48%)\n  - Candidate 3 <button id=\"send-btn\" class=\"btn-primary-new\"> (Score: 94%)\nComparing attributes (data-analytics: match, tag: match, text: match) -> Target identified (94% confidence, Threshold 80%).",
      },
      {
        delay: 5800,
        log: websiteState === 'original'
          ? "Execution completed. Result output: { \"status\": \"success\", \"text\": \"Submit Form\" }"
          : "Auto-Match healing complete! Mapped selector cache to 'button#send-btn.btn-primary-new'.\nResult output: { \"price\": \"Submit Form\", \"healed\": true }",
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
    <div className="border border-gray-150 dark:border-white/10 rounded-2xl overflow-hidden bg-white dark:bg-zinc-950/70 dark:backdrop-blur-xl my-10 shadow-[0_10px_35px_-10px_rgba(0,0,0,0.06)] dark:shadow-[0_20px_50px_rgba(0,0,0,0.3)]">
      {/* Simulator Header */}
      <div className="px-6 py-4 bg-gray-50/50 dark:bg-zinc-900/30 border-b border-gray-150 dark:border-white/5 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-xl bg-brand-violet/10 dark:bg-brand-violet/20 flex items-center justify-center text-brand-violet shadow-sm">
            <Zap size={18} className="animate-pulse" />
          </div>
          <div>
            <h3 className="text-sm font-bold text-gray-900 dark:text-white flex items-center gap-2 tracking-tight">
              Auto-Healing Element Simulator
            </h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 font-medium">
              Simulate dynamic element drifts and see Crawlingo's Jaro-Winkler parser auto-recover nodes.
            </p>
          </div>
        </div>

        {/* Website State Switcher */}
        <div className="flex bg-gray-100/80 dark:bg-black/60 p-1 rounded-xl border border-gray-200/50 dark:border-white/5 self-start sm:self-auto shrink-0 shadow-inner">
          <button
            onClick={() => setWebsiteState('original')}
            className={cn(
              "px-3.5 py-1.5 text-xs font-semibold rounded-lg transition-all duration-300",
              websiteState === 'original'
                ? "bg-white dark:bg-zinc-900 text-black dark:text-white shadow-md border border-gray-200/20"
                : "text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white"
            )}
          >
            Original DOM
          </button>
          <button
            onClick={() => setWebsiteState('redesigned')}
            className={cn(
              "px-3.5 py-1.5 text-xs font-semibold rounded-lg transition-all duration-300",
              websiteState === 'redesigned'
                ? "bg-white dark:bg-zinc-900 text-black dark:text-white shadow-md border border-gray-200/20"
                : "text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white"
            )}
          >
            Redesigned DOM
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-gray-150 dark:divide-white/5">
        
        {/* Left Side: DOM Visualizations - Mock Editor (Lg: col-span-5) */}
        <div className="lg:col-span-5 p-6 space-y-6 bg-transparent">
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <h4 className="text-[10px] font-bold text-gray-400 dark:text-zinc-500 uppercase tracking-widest flex items-center gap-1.5">
                <Globe size={11} /> Target Selector
              </h4>
              <span className="text-[10px] font-mono font-semibold text-brand-violet bg-brand-violet/5 px-2 py-0.5 rounded-md border border-brand-violet/10">
                CSS Selector
              </span>
            </div>
            <div className="bg-gray-50/50 dark:bg-zinc-900/30 border border-gray-150 dark:border-white/5 rounded-xl p-3 font-mono text-xs text-gray-900 dark:text-brand-cyan font-semibold flex items-center justify-between">
              <code>button#submit.btn-primary</code>
              <span className="w-1.5 h-1.5 rounded-full bg-brand-cyan animate-ping" />
            </div>
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <h4 className="text-[10px] font-bold text-gray-400 dark:text-zinc-500 uppercase tracking-widest flex items-center gap-1.5">
                <Layers size={11} /> Mock Webpage DOM
              </h4>
              <div className="flex gap-1">
                <span className="w-2.5 h-2.5 rounded-full bg-red-400/40" />
                <span className="w-2.5 h-2.5 rounded-full bg-yellow-400/40" />
                <span className="w-2.5 h-2.5 rounded-full bg-green-400/40" />
              </div>
            </div>
            
            {/* Syntax Highlighted DOM Editor Box */}
            <div className="bg-zinc-950 dark:bg-black/60 border border-zinc-900 dark:border-white/5 rounded-xl p-5 font-mono text-[11px] leading-relaxed text-zinc-400 shadow-inner overflow-x-auto min-h-[220px] relative">
              <div className="absolute top-2 right-3 text-[9px] text-zinc-600 select-none">HTML SOURCE</div>
              {websiteState === 'original' ? (
                <pre className="space-y-1">
                  <div><span className="text-zinc-600">&lt;</span><span className="text-pink-500">body</span><span className="text-zinc-600">&gt;</span></div>
                  <div>  <span className="text-zinc-600">&lt;</span><span className="text-pink-500">div</span> <span className="text-yellow-600">id</span>=<span className="text-green-500">"wrapper"</span><span className="text-zinc-600">&gt;</span></div>
                  <div>    <span className="text-zinc-600">&lt;</span><span className="text-pink-500">div</span> <span className="text-yellow-600">id</span>=<span className="text-green-500">"submit-wrapper"</span> <span className="text-yellow-600">class</span>=<span className="text-green-500">"p-4"</span><span className="text-zinc-600">&gt;</span></div>
                  <div className="bg-brand-violet/5 border-l-2 border-brand-violet px-2 py-0.5 my-1">
                    <div className="text-zinc-500 text-[9px] select-none italic">// Exact selector match</div>
                    <div>      <span className="text-zinc-600">&lt;</span><span className="text-pink-500">button</span></div>
                    <div>        <span className="text-yellow-600">id</span>=<span className="text-green-500">"submit"</span></div>
                    <div>        <span className="text-yellow-600">class</span>=<span className="text-green-500">"btn btn-primary"</span></div>
                    <div>        <span className="text-yellow-600">data-analytics</span>=<span className="text-green-500">"save-item"</span></div>
                    <div>      <span className="text-zinc-600">&gt;</span>Submit Form<span className="text-zinc-600">&lt;/</span><span className="text-pink-500">button</span><span className="text-zinc-600">&gt;</span></div>
                  </div>
                  <div>    <span className="text-zinc-600">&lt;/</span><span className="text-pink-500">div</span><span className="text-zinc-600">&gt;</span></div>
                  <div>    <span className="text-zinc-600">&lt;</span><span className="text-pink-500">button</span> <span className="text-yellow-600">class</span>=<span className="text-green-500">"btn-secondary"</span><span className="text-zinc-600">&gt;</span>Cancel<span className="text-zinc-600">&lt;/</span><span className="text-pink-500">button</span><span className="text-zinc-600">&gt;</span></div>
                  <div>  <span className="text-zinc-600">&lt;/</span><span className="text-pink-500">div</span><span className="text-zinc-600">&gt;</span></div>
                  <div><span className="text-zinc-600">&lt;/</span><span className="text-pink-500">body</span><span className="text-zinc-600">&gt;</span></div>
                </pre>
              ) : (
                <pre className="space-y-1">
                  <div><span className="text-zinc-600">&lt;</span><span className="text-pink-500">body</span><span className="text-zinc-600">&gt;</span></div>
                  <div>  <span className="text-zinc-600">&lt;</span><span className="text-pink-500">div</span> <span className="text-yellow-600">id</span>=<span className="text-green-500">"wrapper"</span><span className="text-zinc-600">&gt;</span></div>
                  <div>    <span className="text-zinc-600">&lt;</span><span className="text-pink-500">div</span> <span className="text-yellow-600">id</span>=<span className="text-green-500">"submit-wrapper"</span> <span className="text-yellow-600">class</span>=<span className="text-green-500">"p-4"</span><span className="text-zinc-600">&gt;</span></div>
                  <div className="bg-red-500/5 border-l-2 border-red-500 px-2 py-0.5 my-1">
                    <div className="text-zinc-500 text-[9px] select-none italic">// Drifted element (id & class renamed)</div>
                    <div>      <span className="text-zinc-600">&lt;</span><span className="text-pink-500">button</span></div>
                    <div>        <span className="text-yellow-600">id</span>=<span className="text-green-500">"send-btn"</span> <span className="text-zinc-500 italic">(was "submit")</span></div>
                    <div>        <span className="text-yellow-600">class</span>=<span className="text-green-500">"button btn-primary-new"</span></div>
                    <div>        <span className="text-yellow-600">data-analytics</span>=<span className="text-green-500">"save-item"</span></div>
                    <div>      <span className="text-zinc-600">&gt;</span>Submit Form<span className="text-zinc-600">&lt;/</span><span className="text-pink-500">button</span><span className="text-zinc-600">&gt;</span></div>
                  </div>
                  <div>    <span className="text-zinc-600">&lt;/</span><span className="text-pink-500">div</span><span className="text-zinc-600">&gt;</span></div>
                  <div>    <span className="text-zinc-600">&lt;</span><span className="text-pink-500">button</span> <span className="text-yellow-600">class</span>=<span className="text-green-500">"btn-secondary"</span><span className="text-zinc-600">&gt;</span>Cancel<span className="text-zinc-600">&lt;/</span><span className="text-pink-500">button</span><span className="text-zinc-600">&gt;</span></div>
                  <div>  <span className="text-zinc-600">&lt;/</span><span className="text-pink-500">div</span><span className="text-zinc-600">&gt;</span></div>
                  <div><span className="text-zinc-600">&lt;/</span><span className="text-pink-500">body</span><span className="text-zinc-600">&gt;</span></div>
                </pre>
              )}
            </div>
          </div>
        </div>

        {/* Right Side: Execution Console (Lg: col-span-7) */}
        <div className="lg:col-span-7 p-6 flex flex-col justify-between bg-gray-50/20 dark:bg-zinc-950/20">
          <div>
            <div className="flex items-center justify-between mb-4">
              <h4 className="text-[10px] font-bold text-gray-400 dark:text-zinc-500 uppercase tracking-widest flex items-center gap-1.5">
                <Terminal size={11} /> Extraction Terminal
              </h4>
              <button
                disabled={isRunning}
                onClick={runSelector}
                className={cn(
                  "py-1.5 px-4 rounded-xl text-xs font-semibold flex items-center gap-2 transition-all duration-300",
                  "bg-black dark:bg-white text-white dark:text-black hover:opacity-90 shadow-md",
                  "disabled:opacity-50 disabled:cursor-not-allowed"
                )}
              >
                {isRunning ? (
                  <>
                    <RotateCcw size={12} className="animate-spin" />
                    Running...
                  </>
                ) : (
                  <>
                    <Play size={12} />
                    Run Scraper
                  </>
                )}
              </button>
            </div>

            {/* Terminal Window Box */}
            <div className="bg-zinc-950 dark:bg-black rounded-xl p-4 font-mono text-xs h-[230px] overflow-y-auto space-y-3 border border-zinc-900 dark:border-white/5 shadow-inner">
              {logs.length === 0 && (
                <div className="text-zinc-500 italic h-full flex flex-col items-center justify-center gap-2">
                  <Terminal size={24} className="text-zinc-700" />
                  <span>Click 'Run Scraper' to test extraction</span>
                </div>
              )}
              {logs.map((log, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, x: -5 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.3 }}
                  className={cn(
                    "whitespace-pre-wrap leading-relaxed flex items-start gap-2",
                    log.includes("WARNING") || log.includes("drift") ? "text-yellow-400/90" : 
                    log.includes("complete") || log.includes("confidence") || log.includes("success") ? "text-brand-emerald" : "text-zinc-300"
                  )}
                >
                  <span className="text-zinc-650 select-none shrink-0">&gt;</span>
                  <span className="flex-1">{log}</span>
                </motion.div>
              ))}
            </div>
          </div>

          {/* Visual Indicator of Healing Stage */}
          <div className="mt-5 border-t border-gray-150 dark:border-white/5 pt-4">
            <div className="flex items-center justify-between">
              <span className="text-xs font-semibold text-gray-500 dark:text-zinc-400">Analysis status:</span>
              <AnimatePresence mode="wait">
                {currentStep === 0 && (
                  <motion.span
                    key="idle"
                    initial={{ opacity: 0, y: 5 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -5 }}
                    className="text-[10px] font-bold text-gray-500 bg-gray-100 dark:bg-white/5 px-2.5 py-1 rounded-md border border-gray-200/50 dark:border-white/5"
                  >
                    READY
                  </motion.span>
                )}
                {currentStep > 0 && currentStep < 5 && (
                  <motion.span
                    key="running"
                    initial={{ opacity: 0, y: 5 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -5 }}
                    className="text-[10px] font-bold text-yellow-600 dark:text-yellow-400 bg-yellow-500/10 px-2.5 py-1 rounded-md border border-yellow-500/20 flex items-center gap-1.5"
                  >
                    <Layers size={10} className="animate-spin" /> ANALYZING DOM
                  </motion.span>
                )}
                {currentStep === 5 && (
                  <motion.span
                    key="success"
                    initial={{ opacity: 0, y: 5 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -5 }}
                    className={cn(
                      "text-[10px] font-bold px-2.5 py-1 rounded-md flex items-center gap-1.5",
                      websiteState === 'original' 
                        ? "text-brand-emerald bg-brand-emerald/10 border border-brand-emerald/20" 
                        : "text-brand-violet bg-brand-violet/10 border border-brand-violet/20 shadow-[0_0_10px_rgba(168,85,247,0.15)]"
                    )}
                  >
                    {websiteState === 'original' ? (
                      <><Check size={11} /> MATCH VERIFIED</>
                    ) : (
                      <><Zap size={11} className="animate-bounce" /> HEALED [94%]</>
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
