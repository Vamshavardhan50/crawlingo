"use client";

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  ArrowLeft, ArrowRight, Play, Pause, Zap, Shield, 
  Cpu, Clock, Webhook, Terminal, Check, X, 
  ChevronRight, RefreshCw, Server, Code, Eye
} from 'lucide-react';
import EagleLogo from './EagleLogo';
import { cn } from '@/lib/cn';

export default function DemoSlideshow() {
  const [currentSlide, setCurrentSlide] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [activeCodeTab, setActiveCodeTab] = useState<'python' | 'javascript'>('python');

  // Dom self-healing simulator states
  const [driftStep, setDriftStep] = useState<number>(0);
  
  // Stealth fetcher client simulation states
  const [stealthActive, setStealthActive] = useState<boolean>(true);

  // Scheduler / Webhook timeline
  const [schedulerTick, setSchedulerTick] = useState<number>(0);
  const [webhookLog, setWebhookLog] = useState<Array<{ time: string; msg: string; status: 'ok' | 'err' }>>([]);

  const totalSlides = 5;

  // Auto-play timer loop
  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isPlaying) {
      interval = setInterval(() => {
        setCurrentSlide((prev) => (prev === totalSlides - 1 ? 0 : prev + 1));
      }, 8000);
    }
    return () => clearInterval(interval);
  }, [isPlaying]);

  // Self-healing simulator loop
  useEffect(() => {
    if (currentSlide === 1) {
      const timer = setInterval(() => {
        setDriftStep((prev) => (prev === 4 ? 0 : prev + 1));
      }, 3500);
      return () => clearInterval(timer);
    } else {
      setDriftStep(0);
    }
  }, [currentSlide]);

  // Stealth fetcher loop
  useEffect(() => {
    if (currentSlide === 2) {
      const interval = setInterval(() => {
        setStealthActive((prev) => !prev);
      }, 4000);
      return () => clearInterval(interval);
    }
  }, [currentSlide]);

  // Scheduler ticks
  useEffect(() => {
    if (currentSlide === 3) {
      const interval = setInterval(() => {
        setSchedulerTick((prev) => (prev + 1) % 3);
      }, 2000);
      return () => clearInterval(interval);
    }
  }, [currentSlide]);

  useEffect(() => {
    if (currentSlide === 3 && schedulerTick === 2) {
      const timeStr = new Date().toLocaleTimeString();
      setWebhookLog((prev) => [
        { time: timeStr, msg: `POST /webhook -> Dispatching Crawl dataset`, status: 'ok' },
        ...prev.slice(0, 3)
      ]);
    }
  }, [schedulerTick, currentSlide]);

  const slideTitles = [
    "Introducing Crawlingo",
    "Auto-Match & Self-Healing DOM",
    "Stealth Fetcher Engine",
    "Cron Scheduler & Webhooks",
    "Quickstart SDK & Output"
  ];

  return (
    <div className="flex flex-col justify-between py-6 space-y-6 my-6">
      
      {/* Top Slide Header & Progress */}
      <div className="space-y-4">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-gray-200 dark:border-white/5">
          <div>
            <span className="text-[10px] font-mono font-bold tracking-widest text-zinc-400 dark:text-zinc-500 uppercase flex items-center gap-1.5">
              <Eye size={12} /> Slide {currentSlide + 1} of {totalSlides}
            </span>
            <h2 className="text-xl md:text-2xl font-title font-bold text-gray-900 dark:text-white mt-1.5 tracking-tight">
              {slideTitles[currentSlide]}
            </h2>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setIsPlaying(!isPlaying)}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl border border-gray-200 dark:border-white/10 hover:bg-gray-50 dark:hover:bg-white/5 transition-all text-xs font-semibold text-gray-700 dark:text-zinc-300"
            >
              {isPlaying ? (
                <>
                  <Pause size={13} className="text-brand-violet" /> Pause
                </>
              ) : (
                <>
                  <Play size={13} /> Auto-play
                </>
              )}
            </button>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="w-full h-1 bg-gray-100 dark:bg-white/5 rounded-full overflow-hidden">
          <motion.div 
            className="h-full bg-brand-violet"
            initial={{ width: "0%" }}
            animate={{ width: `${((currentSlide + 1) / totalSlides) * 100}%` }}
            transition={{ duration: 0.3 }}
          />
        </div>
      </div>

      {/* Main Slide Panel Area */}
      <div className="flex-1 bg-white dark:bg-zinc-950/40 border border-gray-200 dark:border-white/5 rounded-3xl p-6 md:p-8 flex flex-col justify-center min-h-[460px] relative overflow-hidden shadow-inner">
        
        {/* Background Decorative Grid */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808007_1px,transparent_1px),linear-gradient(to_bottom,#80808007_1px,transparent_1px)] bg-[size:30px_30px] pointer-events-none" />
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[350px] h-[350px] bg-brand-violet/5 rounded-full filter blur-[80px] pointer-events-none" />

        <AnimatePresence mode="wait">
          <motion.div
            key={currentSlide}
            initial={{ opacity: 0, x: 15 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -15 }}
            transition={{ duration: 0.35, ease: "easeOut" }}
            className="w-full h-full relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-8 items-center"
          >
            
            {/* SLIDE 1: INTRODUCING CRAWLINGO */}
            {currentSlide === 0 && (
              <>
                <div className="lg:col-span-7 space-y-6">
                  <div className="flex items-center gap-3.5">
                    <div className="p-1 rounded-2xl bg-gray-50 dark:bg-zinc-900 border border-gray-200/50 dark:border-white/10 shadow-sm">
                      <EagleLogo size="lg" />
                    </div>
                    <div>
                      <h3 className="text-2xl font-title font-bold tracking-tight text-gray-900 dark:text-white">
                        Crawlingo
                      </h3>
                      <p className="text-xs font-mono text-brand-violet font-semibold tracking-wider uppercase">
                        Self-Healing Scraping Framework
                      </p>
                    </div>
                  </div>

                  <p className="text-sm text-gray-600 dark:text-zinc-400 leading-relaxed font-normal">
                    Crawlingo is an ultra-fast web scraper built in Rust. It eliminates pipeline failures by dynamically repairing selectors when layouts update. No more broken integrations when classes, tags, or DOM trees drift.
                  </p>

                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-2">
                    <div className="p-4 bg-gray-50/50 dark:bg-zinc-900/30 border border-gray-200 dark:border-white/5 rounded-2xl flex items-start gap-3 hover:border-brand-violet/20 transition-all duration-300">
                      <Zap className="text-brand-violet shrink-0 mt-0.5" size={18} />
                      <div>
                        <h4 className="text-xs font-bold text-gray-900 dark:text-white">Self-Healing Selectors</h4>
                        <p className="text-[11px] text-gray-500 dark:text-zinc-500 mt-1 leading-normal">Jaro-Winkler + attribute matching recovers drift variables in milliseconds.</p>
                      </div>
                    </div>

                    <div className="p-4 bg-gray-50/50 dark:bg-zinc-900/30 border border-gray-200 dark:border-white/5 rounded-2xl flex items-start gap-3 hover:border-brand-cyan/20 transition-all duration-300">
                      <Shield className="text-brand-cyan shrink-0 mt-0.5" size={18} />
                      <div>
                        <h4 className="text-xs font-bold text-gray-900 dark:text-white">Stealth Engine</h4>
                        <p className="text-[11px] text-gray-500 dark:text-zinc-500 mt-1 leading-normal">Bypasses Cloudflare & anti-bot handshakes natively without browser overhead.</p>
                      </div>
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-5 flex justify-center">
                  <div className="relative w-64 h-64 flex items-center justify-center">
                    <motion.div 
                      className="absolute inset-0 rounded-full border border-dashed border-gray-200 dark:border-white/10"
                      animate={{ rotate: 360 }}
                      transition={{ duration: 40, repeat: Infinity, ease: "linear" }}
                    />
                    <motion.div 
                      className="absolute inset-6 rounded-full border border-dashed border-brand-violet/20"
                      animate={{ rotate: -360 }}
                      transition={{ duration: 25, repeat: Infinity, ease: "linear" }}
                    />
                    
                    {/* Inner glowing core */}
                    <div className="absolute inset-16 rounded-full bg-gray-50 dark:bg-zinc-900/80 border border-gray-200 dark:border-white/10 flex items-center justify-center shadow-lg dark:shadow-[0_0_50px_rgba(168,85,247,0.15)]">
                      <Cpu size={36} className="text-brand-violet animate-pulse" />
                    </div>

                    {/* Orbiting particles */}
                    <motion.div 
                      className="absolute top-0 w-3 h-3 bg-brand-violet rounded-full shadow-[0_0_10px_#a855f7]"
                      animate={{ y: [0, 4, 0] }}
                      transition={{ duration: 2, repeat: Infinity }}
                    />
                    <div className="absolute bottom-6 left-6 w-2.5 h-2.5 bg-brand-cyan rounded-full shadow-[0_0_10px_#06b6d4]" />
                    <div className="absolute right-4 bottom-12 w-2 h-2 bg-brand-emerald rounded-full" />
                  </div>
                </div>
              </>
            )}

            {/* SLIDE 2: AUTO-MATCH SELF-HEALING */}
            {currentSlide === 1 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-brand-violet/10 text-brand-violet text-xs font-mono font-bold">
                    <Zap size={12} className="animate-bounce" />
                    Feature: Self-Healing System
                  </div>
                  <h3 className="text-xl font-title font-bold text-gray-900 dark:text-white">
                    Auto-Recover Drifted Selectors
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-zinc-400 leading-relaxed font-normal">
                    When a website update changes class names, tags, or paths, traditional scrapers instantly break. Crawlingo evaluates candidate nodes and compares text, tags, weights, and ancestors to automatically update selectors.
                  </p>

                  <div className="space-y-3 pt-2">
                    <div className="flex justify-between text-xs font-mono">
                      <span className="text-zinc-500">Healing Phase:</span>
                      <span className="font-bold text-gray-900 dark:text-white tracking-wide">
                        {driftStep === 0 && "1. Base Page State"}
                        {driftStep === 1 && "2. Class names Update"}
                        {driftStep === 2 && "3. Traditional Scraper Crashes"}
                        {driftStep === 3 && "4. Crawlingo Evaluates Candidates"}
                        {driftStep === 4 && "5. Auto-Healed & Restored"}
                      </span>
                    </div>
                    <div className="grid grid-cols-5 gap-1.5 h-1.5 bg-gray-100 dark:bg-white/5 rounded-full overflow-hidden">
                      {[0, 1, 2, 3, 4].map((step) => (
                        <div 
                          key={step} 
                          className={cn(
                            "h-full transition-all duration-300",
                            step <= driftStep ? 'bg-brand-violet' : 'bg-transparent'
                          )} 
                        />
                      ))}
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-6">
                  <div className="bg-white dark:bg-zinc-950/70 border border-gray-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4 relative">
                    <div className="flex items-center justify-between border-b border-gray-200 dark:border-white/5 pb-3">
                      <span className="text-[10px] font-mono font-semibold text-gray-400 dark:text-zinc-500">Auto-Healer Sandbox</span>
                      <span className="flex items-center gap-1.5 text-[10px] font-mono font-semibold">
                        <span className={cn(
                          "w-2 h-2 rounded-full",
                          driftStep === 4 ? 'bg-brand-emerald animate-ping' : 
                          driftStep === 2 ? 'bg-brand-rose animate-pulse' : 'bg-brand-violet'
                        )} />
                        {driftStep === 0 && "Active"}
                        {driftStep === 1 && "DOM Drifted"}
                        {driftStep === 2 && "Traditional Error"}
                        {driftStep === 3 && "Healing..."}
                        {driftStep === 4 && "Successfully Healed"}
                      </span>
                    </div>

                    <div className="space-y-3 font-mono text-[11px]">
                      <div className="p-3 bg-gray-50 dark:bg-black/60 rounded-xl space-y-1.5 border border-gray-200 dark:border-white/5">
                        <span className="text-gray-400 dark:text-zinc-600 text-[9px] font-bold">LIVE DOM TREE</span>
                        <div className="text-zinc-600 dark:text-zinc-400 pl-2 leading-relaxed">
                          &lt;div class=&quot;<span className={cn("transition-colors", driftStep >= 1 ? "text-yellow-500 font-bold" : "text-green-500")}>{driftStep >= 1 ? 'product-card-new' : 'product-card'}</span>&quot;&gt;
                          <div className="pl-4">
                            &lt;span class=&quot;<span className={cn("transition-colors", driftStep >= 1 ? "text-yellow-500 font-bold" : "text-green-500")}>{driftStep >= 1 ? 'amount-tag' : 'price'}</span>&quot;&gt;
                            <span className="text-gray-900 dark:text-white font-bold">$19.99</span>
                            &lt;/span&gt;
                          </div>
                          &lt;/div&gt;
                        </div>
                      </div>

                      <div className="space-y-1.5">
                        <div className="p-2.5 border border-dashed border-gray-200 dark:border-white/5 rounded-xl flex items-center justify-between text-zinc-500 dark:text-zinc-400">
                          <span>Target Selector:</span>
                          <span className="bg-gray-100 dark:bg-white/5 px-2 py-0.5 rounded font-bold text-gray-800 dark:text-zinc-300">span.price</span>
                        </div>

                        {driftStep === 2 && (
                          <motion.div 
                            initial={{ scale: 0.95, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            className="p-3 bg-brand-rose/5 border border-brand-rose/20 text-brand-rose rounded-xl flex items-center gap-2"
                          >
                            <X size={15} />
                            <span>Error: element not found (Pipeline blocked)</span>
                          </motion.div>
                        )}

                        {driftStep === 3 && (
                          <motion.div 
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            className="p-3 bg-brand-violet/5 border border-brand-violet/20 text-brand-violet rounded-xl space-y-1.5"
                          >
                            <div className="flex items-center gap-2">
                              <RefreshCw size={12} className="animate-spin" />
                              <span>Scanning candidate tree...</span>
                            </div>
                            <div className="text-[9px] text-zinc-500 pl-5">
                              Match: span.amount-tag (JW Score: 0.94) {"->"} Verified.
                            </div>
                          </motion.div>
                        )}

                        {driftStep === 4 && (
                          <motion.div 
                            initial={{ scale: 0.95, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            className="p-3 bg-brand-emerald/5 border border-brand-emerald/20 text-brand-emerald rounded-xl space-y-1"
                          >
                            <div className="flex items-center gap-2 font-bold">
                              <Check size={15} />
                              <span>Auto-Healing Complete!</span>
                            </div>
                            <div className="text-[10px] pl-5 text-gray-500 dark:text-zinc-400">
                              Payload resolved: <code className="text-gray-900 dark:text-white">{"{ \"price\": \"$19.99\" }"}</code>
                            </div>
                          </motion.div>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              </>
            )}

            {/* SLIDE 3: STEALTH FETCHING */}
            {currentSlide === 2 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-brand-cyan/10 text-brand-cyan text-xs font-mono font-bold">
                    <Shield size={12} />
                    Feature: Stealth Engine
                  </div>
                  <h3 className="text-xl font-title font-bold text-gray-900 dark:text-white">
                    HTTP/2 & TLS Handshake Spoofing
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-zinc-400 leading-relaxed font-normal">
                    Most firewalls block requests instantly if they exhibit TLS fingerprint mismatches or missing HTTP/2 parameters. Crawlingo mimics chrome JA3 ciphers, rotations, and multiplex ciphers to return success codes cleanly.
                  </p>

                  <div className="p-4 bg-gray-50 dark:bg-zinc-900/30 border border-gray-200 dark:border-white/5 rounded-2xl space-y-2">
                    <span className="text-xs font-bold text-gray-900 dark:text-white block">Stealth Configuration</span>
                    <ul className="text-xs text-gray-500 dark:text-zinc-400 space-y-1.5 list-disc pl-4 leading-normal">
                      <li>Chrome TLS JA3 signature spoofing</li>
                      <li>ALPN connection negotiation details</li>
                      <li>User-Agent headers & cookie synchronization</li>
                    </ul>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  <div className="bg-white dark:bg-zinc-950/70 border border-gray-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4">
                    <div className="flex items-center justify-between text-[10px] font-mono text-gray-400 dark:text-zinc-500 pb-2 border-b border-gray-200 dark:border-white/5">
                      <span>CLIENT SIMULATION</span>
                      <button 
                        onClick={() => setStealthActive(!stealthActive)}
                        className="text-[9px] px-2 py-0.5 bg-black dark:bg-white text-white dark:text-black rounded-md font-semibold"
                      >
                        Toggle Client
                      </button>
                    </div>

                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                      {/* Left: Traditional Client */}
                      <div className={cn(
                        "p-3.5 rounded-xl border transition-all duration-300 flex flex-col justify-between min-h-[120px]",
                        !stealthActive ? 'border-brand-rose/20 bg-brand-rose/5 shadow-sm' : 'border-gray-100 dark:border-white/5 bg-transparent opacity-40'
                      )}>
                        <div>
                          <div className="flex items-center justify-between mb-2">
                            <span className="text-xs font-bold text-brand-rose">Standard GET</span>
                            <X size={12} className="text-brand-rose" />
                          </div>
                          <ul className="text-[9px] font-mono text-gray-500 dark:text-zinc-500 space-y-1">
                            <li>python-requests</li>
                            <li>HTTP/1.1 Standard</li>
                            <li>OpenSSL cipher</li>
                          </ul>
                        </div>
                        <div className="mt-3 p-1.5 bg-brand-rose/10 text-brand-rose rounded-lg text-center text-[9px] font-bold">
                          Blocked (403 Forbidden)
                        </div>
                      </div>

                      {/* Right: Crawlingo Client */}
                      <div className={cn(
                        "p-3.5 rounded-xl border transition-all duration-300 flex flex-col justify-between min-h-[120px]",
                        stealthActive ? 'border-brand-cyan/20 bg-brand-cyan/5 shadow-[0_0_15px_rgba(6,182,212,0.1)]' : 'border-gray-100 dark:border-white/5 bg-transparent opacity-40'
                      )}>
                        <div>
                          <div className="flex items-center justify-between mb-2">
                            <span className="text-xs font-bold text-brand-cyan">Crawlingo Stealth</span>
                            <Check size={12} className="text-brand-cyan" />
                          </div>
                          <ul className="text-[9px] font-mono text-gray-500 dark:text-zinc-500 space-y-1">
                            <li>Chrome 120 client</li>
                            <li>HTTP/2 Multiplexing</li>
                            <li>Chrome JA3 cipher</li>
                          </ul>
                        </div>
                        <div className="mt-3 p-1.5 bg-brand-cyan/10 text-brand-cyan rounded-lg text-center text-[9px] font-bold">
                          Approved (200 OK)
                        </div>
                      </div>
                    </div>

                    <div className="h-11 bg-gray-50 dark:bg-black rounded-xl border border-gray-200 dark:border-white/5 relative flex items-center justify-between px-5 overflow-hidden">
                      <Terminal size={14} className="text-zinc-400" />
                      
                      <motion.div 
                        className={cn("w-3 h-3 rounded-full absolute", stealthActive ? 'bg-brand-cyan shadow-[0_0_10px_#06b6d4]' : 'bg-brand-rose')}
                        animate={{ left: ["10%", "85%"] }}
                        transition={{ repeat: Infinity, duration: 2.5, ease: "easeInOut" }}
                      />

                      <Server size={14} className="text-zinc-400" />
                    </div>
                  </div>
                </div>
              </>
            )}

            {/* SLIDE 4: SCHEDULER & WEBHOOKS */}
            {currentSlide === 3 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-brand-emerald/10 text-brand-emerald text-xs font-mono font-bold">
                    <Clock size={12} />
                    Feature: Pipeline Scheduler
                  </div>
                  <h3 className="text-xl font-title font-bold text-gray-900 dark:text-white">
                    Cron Scrapes & Payload Dispatch
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-zinc-400 leading-relaxed font-normal">
                    Schedule extractions using custom intervals or cron rules. Crawlingo parses targets recursively and forwards parsed data payloads directly to target webhook endpoints.
                  </p>

                  <div className="flex gap-4">
                    <div className="flex-1 p-3 bg-gray-50 dark:bg-zinc-900/30 border border-gray-200 dark:border-white/5 rounded-xl">
                      <div className="flex items-center gap-1.5 text-xs font-bold text-gray-900 dark:text-white mb-1.5">
                        <Clock size={13} className="text-brand-emerald" /> Cron Scheduler
                      </div>
                      <p className="text-[10px] text-gray-500 dark:text-zinc-400 leading-normal">Scrapers run in separate system threads without blocking primary thread activities.</p>
                    </div>

                    <div className="flex-1 p-3 bg-gray-50 dark:bg-zinc-900/30 border border-gray-200 dark:border-white/5 rounded-xl">
                      <div className="flex items-center gap-1.5 text-xs font-bold text-gray-900 dark:text-white mb-1.5">
                        <Webhook size={13} className="text-brand-cyan" /> Webhooks
                      </div>
                      <p className="text-[10px] text-gray-500 dark:text-zinc-400 leading-normal">Posts structured data payloads cleanly with customized retry variables.</p>
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  <div className="bg-white dark:bg-zinc-950/70 border border-gray-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4">
                    <span className="text-[10px] font-mono font-semibold text-gray-400 dark:text-zinc-500 block border-b border-gray-200 dark:border-white/5 pb-2">
                      PIPELINE STREAM
                    </span>

                    <div className="flex items-center justify-around py-1">
                      <div className="flex flex-col items-center gap-1.5">
                        <div className={cn(
                          "p-3 rounded-full border transition-all duration-300",
                          schedulerTick === 0 ? 'bg-brand-emerald text-white border-brand-emerald shadow-[0_0_10px_rgba(16,185,129,0.2)]' : 'bg-gray-50 dark:bg-white/5 border-gray-200 dark:border-white/5'
                        )}>
                          <Clock size={16} />
                        </div>
                        <span className="text-[9px] font-mono font-semibold">1. Cron Tick</span>
                      </div>

                      <ChevronRight size={14} className="text-zinc-300 dark:text-zinc-700" />

                      <div className="flex flex-col items-center gap-1.5">
                        <div className={cn(
                          "p-3 rounded-full border transition-all duration-300",
                          schedulerTick === 1 ? 'bg-brand-emerald text-white border-brand-emerald shadow-[0_0_10px_rgba(16,185,129,0.2)]' : 'bg-gray-50 dark:bg-white/5 border-gray-200 dark:border-white/5'
                        )}>
                          <Cpu size={16} />
                        </div>
                        <span className="text-[9px] font-mono font-semibold">2. Parsing</span>
                      </div>

                      <ChevronRight size={14} className="text-zinc-300 dark:text-zinc-700" />

                      <div className="flex flex-col items-center gap-1.5">
                        <div className={cn(
                          "p-3 rounded-full border transition-all duration-300",
                          schedulerTick === 2 ? 'bg-brand-emerald text-white border-brand-emerald shadow-[0_0_10px_rgba(16,185,129,0.2)]' : 'bg-gray-50 dark:bg-white/5 border-gray-200 dark:border-white/5'
                        )}>
                          <Webhook size={16} />
                        </div>
                        <span className="text-[9px] font-mono font-semibold">3. Webhook</span>
                      </div>
                    </div>

                    <div className="p-3 bg-gray-50 dark:bg-black rounded-xl border border-gray-200 dark:border-white/5 space-y-1.5 min-h-[90px] overflow-y-auto max-h-[110px]">
                      <span className="text-[9px] font-mono font-bold text-gray-400 dark:text-zinc-600 block">WEBHOOK RESPONSE</span>
                      {webhookLog.length === 0 ? (
                        <div className="text-xs text-zinc-500 py-3 text-center">Waiting for pipeline run cycle...</div>
                      ) : (
                        webhookLog.map((log, idx) => (
                          <div key={idx} className="flex items-center justify-between font-mono text-[9px] text-zinc-600 dark:text-zinc-400">
                            <span>{log.time} - {log.msg.substring(0, 32)}...</span>
                            <span className="px-1.5 py-0.5 rounded bg-brand-emerald/10 text-brand-emerald font-bold">200 OK</span>
                          </div>
                        ))
                      )}
                    </div>
                  </div>
                </div>
              </>
            )}

            {/* SLIDE 5: SDK CODE & OUTPUT */}
            {currentSlide === 4 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-gray-900 text-white dark:bg-white dark:text-black text-xs font-mono font-bold">
                    <Code size={12} />
                    Developer Bindings
                  </div>
                  <h3 className="text-xl font-title font-bold text-gray-900 dark:text-white">
                    Clean, Simple API Integration
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-zinc-400 leading-relaxed font-normal">
                    Crawlingo is fully accessible via native language wraps in Python and Node.js. Setup crawler details, threshold parameters, proxies, and webhooks in just a few lines.
                  </p>

                  <div className="flex gap-2">
                    <button 
                      onClick={() => setActiveCodeTab('python')}
                      className={cn(
                        "px-3.5 py-1.5 rounded-lg text-xs font-mono transition-all duration-300 font-semibold",
                        activeCodeTab === 'python' 
                          ? 'bg-black text-white dark:bg-white dark:text-black shadow-md' 
                          : 'bg-gray-100 dark:bg-white/5 text-zinc-500 hover:text-gray-900 dark:hover:text-white'
                      )}
                    >
                      Python
                    </button>
                    <button 
                      onClick={() => setActiveCodeTab('javascript')}
                      className={cn(
                        "px-3.5 py-1.5 rounded-lg text-xs font-mono transition-all duration-300 font-semibold",
                        activeCodeTab === 'javascript' 
                          ? 'bg-black text-white dark:bg-white dark:text-black shadow-md' 
                          : 'bg-gray-100 dark:bg-white/5 text-zinc-500 hover:text-gray-900 dark:hover:text-white'
                      )}
                    >
                      Node.js
                    </button>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  <div className="bg-zinc-950 dark:bg-black border border-zinc-900 dark:border-white/5 rounded-2xl overflow-hidden shadow-2xl">
                    <div className="bg-zinc-900 dark:bg-zinc-950 px-4 py-2 border-b border-zinc-800 dark:border-white/5 flex items-center justify-between text-[10px] font-mono text-zinc-500">
                      <span>{activeCodeTab === 'python' ? 'crawlingo_app.py' : 'crawlingo_app.js'}</span>
                      <div className="flex gap-1">
                        <span className="w-2.5 h-2.5 rounded-full bg-red-400/60" />
                        <span className="w-2.5 h-2.5 rounded-full bg-yellow-400/60" />
                        <span className="w-2.5 h-2.5 rounded-full bg-green-400/60" />
                      </div>
                    </div>

                    <div className="p-4 font-mono text-[11px] overflow-x-auto bg-zinc-950 dark:bg-black/90 text-zinc-300 min-h-[220px]">
                      {activeCodeTab === 'python' ? (
                        <pre className="space-y-1 text-zinc-400">
                          <div><span className="text-pink-500 font-semibold">import</span> crawlingo</div>
                          <div></div>
                          <div><span className="text-zinc-600"># Setup session config</span></div>
                          <div>session = crawlingo.<span className="text-brand-cyan">Session</span>(</div>
                          <div>    proxies=[<span className="text-brand-emerald">&quot;http://127.0.0.1:8000&quot;</span>],</div>
                          <div>    weights=<span className="text-brand-cyan">SimilarityWeights</span>(text=<span className="text-yellow-500">1.0</span>, tag=<span className="text-yellow-500">0.8</span>)</div>
                          <div>)</div>
                          <div></div>
                          <div><span className="text-zinc-600"># Run auto-healing crawl</span></div>
                          <div>results = session.<span className="text-brand-cyan">crawl</span>(</div>
                          <div>    url=<span className="text-brand-emerald">&quot;https://site.com/products&quot;</span>,</div>
                          <div>    selector=<span className="text-brand-emerald">&quot;span.price&quot;</span>,</div>
                          <div>    webhook=<span className="text-brand-emerald">&quot;https://webhook.site/v1&quot;</span></div>
                          <div>)</div>
                          <div><span className="text-pink-500 font-semibold">print</span>(results)</div>
                        </pre>
                      ) : (
                        <pre className="space-y-1 text-zinc-400">
                          <div><span className="text-pink-500 font-semibold">const</span> crawlingo = <span className="text-pink-500 font-semibold">require</span>(<span className="text-brand-emerald">&apos;crawlingo&apos;</span>);</div>
                          <div></div>
                          <div><span className="text-zinc-600">// Setup session config</span></div>
                          <div><span className="text-pink-500 font-semibold">const</span> session = <span className="text-pink-500 font-semibold">new</span> crawlingo.<span className="text-brand-cyan">Session</span>({"{"}</div>
                          <div>  proxies: [<span className="text-brand-emerald">&quot;http://127.0.0.1:8000&quot;</span>],</div>
                          <div>  weights: {"{"} text: <span className="text-yellow-500">1.0</span>, tag: <span className="text-yellow-500">0.8</span> {"}"}</div>
                          <div>{"}"});</div>
                          <div></div>
                          <div><span className="text-zinc-600">// Run auto-healing crawl</span></div>
                          <div><span className="text-pink-500 font-semibold">const</span> results = <span className="text-pink-500 font-semibold">await</span> session.<span className="text-brand-cyan">crawl</span>({"{"})</div>
                          <div>  url: <span className="text-brand-emerald">&quot;https://site.com/products&quot;</span>,</div>
                          <div>  selector: <span className="text-brand-emerald">&quot;span.price&quot;</span>,</div>
                          <div>  webhook: <span className="text-brand-emerald">&quot;https://webhook.site/v1&quot;</span></div>
                          <div>{"}"});</div>
                          <div>console.<span className="text-brand-cyan">log</span>(results);</div>
                        </pre>
                      )}
                    </div>
                  </div>
                </div>
              </>
            )}

          </motion.div>
        </AnimatePresence>
      </div>

      {/* Bottom Controls / Slide Navigation */}
      <div className="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-white/5">
        <button
          onClick={() => setCurrentSlide((prev) => Math.max(0, prev - 1))}
          disabled={currentSlide === 0}
          className="flex items-center gap-1.5 px-4 py-2 text-xs font-semibold rounded-xl border border-gray-200 dark:border-white/10 text-gray-700 dark:text-zinc-300 hover:bg-gray-50 dark:hover:bg-white/5 disabled:opacity-40 disabled:pointer-events-none transition-colors"
        >
          <ArrowLeft size={14} />
          Back
        </button>

        {/* Bullet page Indicators */}
        <div className="flex items-center gap-2">
          {Array.from({ length: totalSlides }).map((_, idx) => (
            <button
              key={idx}
              onClick={() => setCurrentSlide(idx)}
              className={cn(
                "w-2 h-2 rounded-full transition-all duration-300",
                currentSlide === idx 
                  ? 'bg-brand-violet scale-125 shadow-[0_0_8px_#a855f7]' 
                  : 'bg-gray-300 dark:bg-white/20'
              )}
            />
          ))}
        </div>

        {currentSlide === totalSlides - 1 ? (
          <button
            onClick={() => setCurrentSlide(0)}
            className="flex items-center gap-1.5 px-4 py-2 text-xs font-semibold rounded-xl bg-black text-white dark:bg-white dark:text-black hover:opacity-90 transition-all shadow-sm"
          >
            Restart
            <RefreshCw size={13} />
          </button>
        ) : (
          <button
            onClick={() => setCurrentSlide((prev) => prev + 1)}
            className="flex items-center gap-1.5 px-4 py-2 text-xs font-semibold rounded-xl bg-black text-white dark:bg-white dark:text-black hover:opacity-90 transition-all shadow-sm"
          >
            Next
            <ArrowRight size={13} />
          </button>
        )}
      </div>
      
    </div>
  );
}
