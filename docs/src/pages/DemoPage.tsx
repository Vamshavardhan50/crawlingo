import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  ArrowLeft, ArrowRight, Play, Pause, Zap, Shield, Layers, 
  Database, Cpu, Clock, Webhook, Terminal, Check, X, 
  ChevronRight, RefreshCw, Server, Globe, Sparkles 
} from 'lucide-react';
import EagleLogo from '@/components/ui/EagleLogo';

export default function DemoPage() {
  const [currentSlide, setCurrentSlide] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [activeCodeTab, setActiveCodeTab] = useState<'python' | 'javascript'>('python');

  // Dom self-healing simulator states
  const [driftStep, setDriftStep] = useState<number>(0);
  // 0: Initial page state
  // 1: Developer updates website (selector drifts)
  // 2: Traditional scraper tries to run & crashes
  // 3: Crawlingo runs scanner / computes Jaro-Winkler
  // 4: Crawlingo auto-heals and extracts successfully
  
  // Stealth fetcher client simulation states
  const [stealthActive, setStealthActive] = useState<boolean>(false);
  const [stealthStatus, setStealthStatus] = useState<'idle' | 'fetching' | 'blocked' | 'success'>('idle');

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
      }, 5000);
      return () => clearInterval(interval);
    }
  }, [currentSlide]);

  // Scheduler ticks
  useEffect(() => {
    if (currentSlide === 3) {
      const interval = setInterval(() => {
        setSchedulerTick((prev) => (prev + 1) % 4);
        const timeStr = new Date().toLocaleTimeString();
        if (schedulerTick === 2) {
          setWebhookLog((prev) => [
            { time: timeStr, msg: `POST /webhook -> Dispatching Crawl dataset`, status: 'ok' },
            ...prev.slice(0, 3)
          ]);
        }
      }, 2000);
      return () => clearInterval(interval);
    } else {
      setWebhookLog([]);
    }
  }, [currentSlide, schedulerTick]);

  const slideTitles = [
    "Introducing Crawlingo",
    "Auto-Match & Self-Healing DOM",
    "Stealth Fetcher Engine",
    "Cron Scheduler & Webhooks",
    "Quickstart SDK & Output"
  ];

  return (
    <div className="min-h-[calc(100vh-8rem)] flex flex-col justify-between py-6 space-y-6">
      
      {/* Top Slide Header & Progress */}
      <div className="space-y-4">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-zinc-200 dark:border-white/10">
          <div>
            <span className="text-xs font-mono font-bold tracking-widest text-zinc-500 uppercase">
              Demo Slideshow • Slide {currentSlide + 1} of {totalSlides}
            </span>
            <h1 className="text-2xl md:text-3xl font-black text-black dark:text-white mt-1">
              {slideTitles[currentSlide]}
            </h1>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setIsPlaying(!isPlaying)}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-white/10 hover:bg-zinc-100 dark:hover:bg-white/5 transition-colors text-xs font-medium"
            >
              {isPlaying ? (
                <>
                  <Pause size={14} /> Pause Auto-play
                </>
              ) : (
                <>
                  <Play size={14} /> Auto-play
                </>
              )}
            </button>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="w-full h-1 bg-zinc-200 dark:bg-white/5 rounded-full overflow-hidden">
          <motion.div 
            className="h-full bg-black dark:bg-white"
            initial={{ width: "0%" }}
            animate={{ width: `${((currentSlide + 1) / totalSlides) * 100}%` }}
            transition={{ duration: 0.3 }}
          />
        </div>
      </div>

      {/* Main Slide Panel Area */}
      <div className="flex-1 bg-zinc-50 dark:bg-zinc-950 border border-zinc-200 dark:border-white/10 rounded-2xl p-6 md:p-8 flex flex-col justify-center min-h-[450px] relative overflow-hidden">
        
        {/* Background Decorative Grid */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#8080800a_1px,transparent_1px),linear-gradient(to_bottom,#8080800a_1px,transparent_1px)] bg-[size:24px_24px] pointer-events-none" />

        <AnimatePresence mode="wait">
          <motion.div
            key={currentSlide}
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            transition={{ duration: 0.4, ease: [0.16, 1, 0.3, 1] }}
            className="w-full h-full relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-8 items-center"
          >
            
            {/* SLIDE 1: INTRODUCING CRAWLINGO */}
            {currentSlide === 0 && (
              <>
                <div className="lg:col-span-7 space-y-6">
                  <div className="flex items-center gap-3">
                    <EagleLogo size="xl" className="animate-pulse" />
                    <div>
                      <h2 className="text-3xl font-black tracking-tight text-black dark:text-white">
                        Crawlingo
                      </h2>
                      <p className="text-sm font-mono text-zinc-500">
                        The Self-Healing Scraping Framework
                      </p>
                    </div>
                  </div>

                  <p className="text-base text-zinc-600 dark:text-zinc-400 leading-relaxed">
                    Crawlingo is a high-performance web extraction tool written in Rust. 
                    It solves the fragile selector problem by dynamically matching changed elements on webpage revisions. 
                    No more broken pipelines when websites update class names, ids, or structures.
                  </p>

                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div className="p-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl flex items-start gap-3">
                      <Zap className="text-black dark:text-white shrink-0 mt-0.5" size={18} />
                      <div>
                        <h4 className="text-sm font-bold text-black dark:text-white">Self-Healing Selectors</h4>
                        <p className="text-xs text-zinc-500 mt-0.5">Jaro-Winkler similarity matching repairs broken selectors instantly.</p>
                      </div>
                    </div>

                    <div className="p-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl flex items-start gap-3">
                      <Shield className="text-black dark:text-white shrink-0 mt-0.5" size={18} />
                      <div>
                        <h4 className="text-sm font-bold text-black dark:text-white">Stealth Engine</h4>
                        <p className="text-xs text-zinc-500 mt-0.5">TLS & header fingerprint spoofing without heavy browser headless overhead.</p>
                      </div>
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-5 flex justify-center">
                  <motion.div 
                    className="relative w-64 h-64 flex items-center justify-center bg-zinc-100 dark:bg-zinc-900 border border-zinc-200 dark:border-white/10 rounded-full shadow-2xl"
                    animate={{ rotate: 360 }}
                    transition={{ duration: 30, repeat: Infinity, ease: "linear" }}
                  >
                    <div className="absolute inset-4 rounded-full border border-dashed border-zinc-300 dark:border-white/20" />
                    <div className="absolute inset-12 rounded-full bg-zinc-200 dark:bg-zinc-800/80 flex items-center justify-center">
                      <Cpu size={40} className="text-black dark:text-white" />
                    </div>
                    {/* Orbiting particles */}
                    <div className="absolute top-0 w-4 h-4 bg-black dark:bg-white rounded-full" />
                    <div className="absolute bottom-0 w-4 h-4 bg-zinc-400 dark:bg-zinc-600 rounded-full" />
                    <div className="absolute left-0 w-3 h-3 bg-zinc-300 dark:bg-zinc-700 rounded-full" />
                    <div className="absolute right-0 w-3 h-3 bg-zinc-300 dark:bg-zinc-700 rounded-full" />
                  </motion.div>
                </div>
              </>
            )}

            {/* SLIDE 2: AUTO-MATCH SELF-HEALING */}
            {currentSlide === 1 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-2 px-2.5 py-1 rounded bg-yellow-500/10 text-yellow-600 dark:text-yellow-400 text-xs font-mono font-bold">
                    <Sparkles size={12} />
                    Feature Focus: Resilient Matching
                  </div>
                  <h3 className="text-2xl font-bold text-black dark:text-white">
                    The Healing Simulator
                  </h3>
                  <p className="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed">
                    Watch the drift simulator on the right. When the website layout shifts (classes change from <code className="font-mono bg-zinc-200 dark:bg-white/10 px-1 py-0.5 rounded text-black dark:text-white">.price</code> to <code className="font-mono bg-zinc-200 dark:bg-white/10 px-1 py-0.5 rounded text-black dark:text-white">.amount-tag</code>), Crawlingo scans alternative element tags, contents, and ancestors to automatically fix the broken reference.
                  </p>

                  <div className="space-y-2">
                    <div className="flex justify-between text-xs font-mono">
                      <span className="text-zinc-500">Current Phase:</span>
                      <span className="font-bold text-black dark:text-white uppercase">
                        {driftStep === 0 && "1. Original Layout"}
                        {driftStep === 1 && "2. Website Updates classes"}
                        {driftStep === 2 && "3. Traditional Scraper Crashes"}
                        {driftStep === 3 && "4. Crawlingo Runs Matching Scan"}
                        {driftStep === 4 && "5. Auto-Healed & Extracted!"}
                      </span>
                    </div>
                    <div className="grid grid-cols-5 gap-1.5 h-1.5 bg-zinc-200 dark:bg-white/5 rounded-full overflow-hidden">
                      {[0, 1, 2, 3, 4].map((step) => (
                        <div 
                          key={step} 
                          className={`h-full transition-colors duration-300 ${
                            step <= driftStep ? 'bg-black dark:bg-white' : 'bg-transparent'
                          }`} 
                        />
                      ))}
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-6">
                  <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4">
                    <div className="flex items-center justify-between border-b border-zinc-100 dark:border-white/5 pb-3">
                      <span className="text-xs font-mono font-semibold text-zinc-400">Selector Sim Log</span>
                      <span className="flex items-center gap-1.5 text-xs font-mono">
                        <span className={`w-2 h-2 rounded-full ${driftStep === 4 ? 'bg-green-500 animate-ping' : driftStep === 2 ? 'bg-red-500 animate-pulse' : 'bg-yellow-500'}`} />
                        {driftStep === 0 && "Active"}
                        {driftStep === 1 && "Layout changed"}
                        {driftStep === 2 && "Traditional Error"}
                        {driftStep === 3 && "Healing..."}
                        {driftStep === 4 && "Healed Successful"}
                      </span>
                    </div>

                    <div className="space-y-3 font-mono text-xs">
                      {/* Code representations */}
                      <div className="p-3 bg-zinc-50 dark:bg-zinc-950 rounded-lg space-y-1">
                        <span className="text-zinc-500 text-[10px]">DOM Element Tree</span>
                        <div className="text-zinc-600 dark:text-zinc-400 pl-2">
                          &lt;div class=&quot;{driftStep >= 1 ? 'product-card-new' : 'product-card'}&quot;&gt;
                          <div className="pl-4">
                            &lt;span class=&quot;{driftStep >= 1 ? 'amount-tag' : 'price'}&quot;&gt;
                            <span className="text-black dark:text-white font-bold">$19.99</span>
                            &lt;/span&gt;
                          </div>
                          &lt;/div&gt;
                        </div>
                      </div>

                      {/* Scraper comparison */}
                      <div className="space-y-1">
                        <div className="p-2 border border-dashed border-zinc-200 dark:border-white/5 rounded-lg flex items-center justify-between">
                          <span className="text-zinc-500">Query selector:</span>
                          <span className="bg-zinc-100 dark:bg-white/5 px-2 py-0.5 rounded font-bold">span.price</span>
                        </div>

                        {driftStep === 2 && (
                          <motion.div 
                            initial={{ scale: 0.95, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            className="p-3 bg-red-500/10 border border-red-500/20 text-red-500 rounded-lg flex items-center gap-2"
                          >
                            <X size={16} />
                            <span>Error: element not found (Traditional selector broke!)</span>
                          </motion.div>
                        )}

                        {driftStep === 3 && (
                          <motion.div 
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            className="p-3 bg-yellow-500/10 border border-yellow-500/20 text-yellow-600 dark:text-yellow-400 rounded-lg space-y-1.5"
                          >
                            <div className="flex items-center gap-2">
                              <RefreshCw size={14} className="animate-spin" />
                              <span>Scanning candidate matching tree...</span>
                            </div>
                            <div className="text-[10px] text-zinc-500 pl-6">
                              Match: span.amount-tag (Tag: 100%, Text: 100%, Parents: 85%) {"->"} JW Similarity score: 0.94
                            </div>
                          </motion.div>
                        )}

                        {driftStep === 4 && (
                          <motion.div 
                            initial={{ scale: 0.95, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            className="p-3 bg-green-500/10 border border-green-500/20 text-green-600 dark:text-green-400 rounded-lg space-y-1"
                          >
                            <div className="flex items-center gap-2 font-bold">
                              <Check size={16} />
                              <span>Healed Selector Auto-Mapped!</span>
                            </div>
                            <div className="text-xs pl-6">
                              Resolved data: <code className="font-semibold text-black dark:text-white">{"{ \"price\": \"$19.99\" }"}</code>
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
                  <div className="inline-flex items-center gap-2 px-2.5 py-1 rounded bg-green-500/10 text-green-600 dark:text-green-400 text-xs font-mono font-bold">
                    <Shield size={12} />
                    Core Architecture: Stealth Client
                  </div>
                  <h3 className="text-2xl font-bold text-black dark:text-white">
                    HTTP/2 & TLS Fingerprint Spoofing
                  </h3>
                  <p className="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed">
                    Most anti-bot firewalls block basic HTTP clients immediately because of missing HTTP/2 features or raw TLS signature mismatches. Crawlingo's Rust core handles TLS/ALPN handshake configuration and mimics real browsers without the performance bottleneck of opening Chrome.
                  </p>

                  <div className="p-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl space-y-2">
                    <span className="text-xs font-bold text-black dark:text-white block">Stealth Spoof Settings</span>
                    <ul className="text-xs text-zinc-500 space-y-1 list-disc pl-4">
                      <li>Chrome TLS JA3 signature mimicry</li>
                      <li>HTTP/2 multiplexing & connection headers alignment</li>
                      <li>User-Agent auto-rotation & headers spoofing</li>
                    </ul>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  {/* Visual animation representing client traffic */}
                  <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4">
                    <div className="flex items-center justify-between text-xs font-mono text-zinc-400 pb-2 border-b border-zinc-100 dark:border-white/5">
                      <span>Fetcher Comparison</span>
                      <button 
                        onClick={() => setStealthActive(!stealthActive)}
                        className="text-xs px-2 py-0.5 bg-black dark:bg-white text-white dark:text-black rounded"
                      >
                        Toggle Client Style
                      </button>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      {/* Left: Traditional Client */}
                      <div className={`p-3 rounded-xl border transition-all duration-300 ${!stealthActive ? 'border-red-500/30 bg-red-500/5' : 'border-zinc-200 dark:border-white/5 bg-transparent opacity-50'}`}>
                        <div className="flex items-center justify-between mb-2">
                          <span className="text-xs font-bold text-red-500">Standard Requests</span>
                          <X size={14} className="text-red-500" />
                        </div>
                        <ul className="text-[10px] font-mono text-zinc-500 space-y-1">
                          <li>User-Agent: python-requests</li>
                          <li>HTTP/1.1 Standard</li>
                          <li>OpenSSL Cipher Suite</li>
                        </ul>
                        <div className="mt-4 p-2 bg-red-500/10 text-red-600 rounded text-center text-[10px] font-mono font-bold">
                          Cloudflare Blocked (403)
                        </div>
                      </div>

                      {/* Right: Crawlingo Client */}
                      <div className={`p-3 rounded-xl border transition-all duration-300 ${stealthActive ? 'border-green-500/30 bg-green-500/5 shadow-[0_0_15px_rgba(34,197,94,0.1)]' : 'border-zinc-200 dark:border-white/5 bg-transparent opacity-50'}`}>
                        <div className="flex items-center justify-between mb-2">
                          <span className="text-xs font-bold text-green-500">Crawlingo Stealth</span>
                          <Check size={14} className="text-green-500" />
                        </div>
                        <ul className="text-[10px] font-mono text-zinc-500 space-y-1">
                          <li>User-Agent: Chrome/120.0</li>
                          <li>HTTP/2 Multiplexing</li>
                          <li>Chrome JA3 Fingerprint</li>
                        </ul>
                        <div className="mt-4 p-2 bg-green-500/10 text-green-600 rounded text-center text-[10px] font-mono font-bold">
                          Connection Success (200)
                        </div>
                      </div>
                    </div>

                    {/* Packet transmission illustration */}
                    <div className="h-12 bg-zinc-50 dark:bg-zinc-950 rounded-xl relative flex items-center justify-between px-6 overflow-hidden">
                      <Terminal size={16} className="text-zinc-400" />
                      
                      {/* Animating packet dot */}
                      <motion.div 
                        className={`w-3.5 h-3.5 rounded-full absolute ${stealthActive ? 'bg-green-500' : 'bg-red-500'}`}
                        animate={{ left: ["10%", "90%"] }}
                        transition={{ repeat: Infinity, duration: 2, ease: "easeInOut" }}
                      />

                      <Server size={16} className="text-zinc-400" />
                    </div>
                  </div>
                </div>
              </>
            )}

            {/* SLIDE 4: SCHEDULER & WEBHOOKS */}
            {currentSlide === 3 && (
              <>
                <div className="lg:col-span-6 space-y-6">
                  <div className="inline-flex items-center gap-2 px-2.5 py-1 rounded bg-purple-500/10 text-purple-600 dark:text-purple-400 text-xs font-mono font-bold">
                    <Clock size={12} />
                    Automation Pipeline
                  </div>
                  <h3 className="text-2xl font-bold text-black dark:text-white">
                    Scheduled Crawls & Real-time Webhooks
                  </h3>
                  <p className="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed">
                    Set up structured scrapers to execute on recurring intervals or standard cron parameters. Data results are immediately pushed directly to target Webhook HTTP endpoints, allowing you to build real-time monitoring workflows seamlessly.
                  </p>

                  <div className="flex gap-4">
                    <div className="flex-1 p-3 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl">
                      <div className="flex items-center gap-1.5 text-xs font-bold text-black dark:text-white mb-1">
                        <Clock size={14} /> Cron Scheduler
                      </div>
                      <p className="text-[11px] text-zinc-500">Run loops autonomously in secondary OS threads without blocking primary thread execution.</p>
                    </div>

                    <div className="flex-1 p-3 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl">
                      <div className="flex items-center gap-1.5 text-xs font-bold text-black dark:text-white mb-1">
                        <Webhook size={14} /> Webhook Delivery
                      </div>
                      <p className="text-[11px] text-zinc-500">Post dataset JSON payloads instantly to configured webhook endpoints with automatic retries.</p>
                    </div>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  {/* Cron and Webhook pipeline visualizer */}
                  <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/10 rounded-2xl p-5 shadow-lg space-y-4">
                    <span className="text-xs font-mono font-bold text-zinc-400 block border-b border-zinc-100 dark:border-white/5 pb-2">
                      Pipeline Activity Stream
                    </span>

                    <div className="flex items-center justify-around py-2">
                      {/* Scheduler node */}
                      <div className="flex flex-col items-center gap-1">
                        <div className={`p-3 rounded-full border ${schedulerTick === 0 ? 'bg-zinc-800 text-white dark:bg-white dark:text-black border-black' : 'bg-zinc-100 dark:bg-white/5 border-zinc-200 dark:border-white/5'}`}>
                          <Clock size={20} />
                        </div>
                        <span className="text-[10px] font-mono">1. Cron Tick</span>
                      </div>

                      <ChevronRight size={16} className="text-zinc-300" />

                      {/* Crawl Engine node */}
                      <div className="flex flex-col items-center gap-1">
                        <div className={`p-3 rounded-full border ${schedulerTick === 1 ? 'bg-zinc-800 text-white dark:bg-white dark:text-black border-black' : 'bg-zinc-100 dark:bg-white/5 border-zinc-200 dark:border-white/5'}`}>
                          <Cpu size={20} />
                        </div>
                        <span className="text-[10px] font-mono">2. Extracting</span>
                      </div>

                      <ChevronRight size={16} className="text-zinc-300" />

                      {/* Webhook Post node */}
                      <div className="flex flex-col items-center gap-1">
                        <div className={`p-3 rounded-full border ${schedulerTick === 2 ? 'bg-zinc-800 text-white dark:bg-white dark:text-black border-black animate-pulse' : 'bg-zinc-100 dark:bg-white/5 border-zinc-200 dark:border-white/5'}`}>
                          <Webhook size={20} />
                        </div>
                        <span className="text-[10px] font-mono">3. Webhook POST</span>
                      </div>
                    </div>

                    {/* Log of successful posts */}
                    <div className="p-3 bg-zinc-50 dark:bg-zinc-950 rounded-xl space-y-1.5 min-h-[90px]">
                      <span className="text-[10px] font-mono text-zinc-400 block mb-1">Live webhook responses:</span>
                      {webhookLog.length === 0 ? (
                        <div className="text-xs text-zinc-500 italic py-2 text-center">Waiting for next schedule cycle...</div>
                      ) : (
                        webhookLog.map((log, idx) => (
                          <div key={idx} className="flex items-center justify-between font-mono text-[10px] text-zinc-500">
                            <span>{log.time} - {log.msg}</span>
                            <span className="px-1.5 py-0.5 rounded bg-green-500/10 text-green-500 font-bold">200 OK</span>
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
                  <div className="inline-flex items-center gap-2 px-2.5 py-1 rounded bg-zinc-900 text-white dark:bg-white dark:text-black text-xs font-mono font-bold">
                    <Terminal size={12} />
                    Developer Interface
                  </div>
                  <h3 className="text-2xl font-bold text-black dark:text-white">
                    Clean, Simple API Bindings
                  </h3>
                  <p className="text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed">
                    Crawlingo is fully accessible via native SDK wrappers in Python and Node.js. 
                    Write a few lines of code to declare your scraping goals and configure parameters like custom weights, proxies, and scheduler parameters.
                  </p>

                  <div className="flex gap-2">
                    <button 
                      onClick={() => setActiveCodeTab('python')}
                      className={`px-3 py-1.5 rounded-lg text-xs font-mono transition-colors ${
                        activeCodeTab === 'python' 
                          ? 'bg-black text-white dark:bg-white dark:text-black font-semibold' 
                          : 'bg-zinc-100 dark:bg-white/5 text-zinc-500 hover:text-black dark:hover:text-white'
                      }`}
                    >
                      crawlingo.py (Python)
                    </button>
                    <button 
                      onClick={() => setActiveCodeTab('javascript')}
                      className={`px-3 py-1.5 rounded-lg text-xs font-mono transition-colors ${
                        activeCodeTab === 'javascript' 
                          ? 'bg-black text-white dark:bg-white dark:text-black font-semibold' 
                          : 'bg-zinc-100 dark:bg-white/5 text-zinc-500 hover:text-black dark:hover:text-white'
                      }`}
                    >
                      crawlingo.js (Node.js)
                    </button>
                  </div>
                </div>

                <div className="lg:col-span-6 space-y-4">
                  {/* Code editor simulation */}
                  <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-white/10 rounded-2xl overflow-hidden shadow-lg">
                    <div className="bg-zinc-100 dark:bg-zinc-950 px-4 py-2 border-b border-zinc-200 dark:border-white/10 flex items-center justify-between text-xs font-mono text-zinc-500">
                      <span>{activeCodeTab === 'python' ? 'crawlingo_app.py' : 'crawlingo_app.js'}</span>
                      <div className="flex gap-1.5">
                        <span className="w-2.5 h-2.5 rounded-full bg-red-500" />
                        <span className="w-2.5 h-2.5 rounded-full bg-yellow-500" />
                        <span className="w-2.5 h-2.5 rounded-full bg-green-500" />
                      </div>
                    </div>

                    <div className="p-4 font-mono text-xs overflow-x-auto bg-zinc-50 dark:bg-zinc-950 text-zinc-600 dark:text-zinc-300">
                      {activeCodeTab === 'python' ? (
                        <pre className="space-y-1">
                          <div><span className="text-purple-500">import</span> crawlingo</div>
                          <div></div>
                          <div><span className="text-zinc-500"># Setup Stealth session & similarity config</span></div>
                          <div>session = crawlingo.<span className="text-blue-500">Session</span>(</div>
                          <div>    proxies=[<span className="text-green-600">&quot;http://127.0.0.1:8000&quot;</span>],</div>
                          <div>    weights=<span className="text-blue-500">SimilarityWeights</span>(text=<span className="text-amber-500">1.0</span>, tag=<span className="text-amber-500">0.8</span>)</div>
                          <div>)</div>
                          <div></div>
                          <div><span className="text-zinc-500"># Crawl with auto-healing</span></div>
                          <div>results = session.<span className="text-blue-500">crawl</span>(</div>
                          <div>    url=<span className="text-green-600">&quot;https://site.com/products&quot;</span>,</div>
                          <div>    selector=<span className="text-green-600">&quot;span.price&quot;</span>,</div>
                          <div>    webhook=<span className="text-green-600">&quot;https://webhook.site/v1&quot;</span></div>
                          <div>)</div>
                          <div><span className="text-purple-500">print</span>(results)</div>
                        </pre>
                      ) : (
                        <pre className="space-y-1">
                          <div><span className="text-purple-500">const</span> crawlingo = <span className="text-purple-500">require</span>(<span className="text-green-600">&apos;crawlingo&apos;</span>);</div>
                          <div></div>
                          <div><span className="text-zinc-500">// Setup Stealth session & similarity config</span></div>
                          <div><span className="text-purple-500">const</span> session = <span className="text-purple-500">new</span> crawlingo.<span className="text-blue-500">Session</span>({"{"})</div>
                          <div>  proxies: [<span className="text-green-600">&quot;http://127.0.0.1:8000&quot;</span>],</div>
                          <div>  weights: {"{"} text: <span className="text-amber-500">1.0</span>, tag: <span className="text-amber-500">0.8</span> {"}"}</div>
                          <div>{"}"});</div>
                          <div></div>
                          <div><span className="text-zinc-500">// Crawl with auto-healing</span></div>
                          <div><span className="text-purple-500">const</span> results = <span className="text-purple-500">await</span> session.<span className="text-blue-500">crawl</span>({"{"})</div>
                          <div>  url: <span className="text-green-600">&quot;https://site.com/products&quot;</span>,</div>
                          <div>  selector: <span className="text-green-600">&quot;span.price&quot;</span>,</div>
                          <div>  webhook: <span className="text-green-600">&quot;https://webhook.site/v1&quot;</span></div>
                          <div>{"}"});</div>
                          <div>console.<span className="text-blue-500">log</span>(results);</div>
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
      <div className="flex items-center justify-between pt-4 border-t border-zinc-200 dark:border-white/10">
        <button
          onClick={() => setCurrentSlide((prev) => Math.max(0, prev - 1))}
          disabled={currentSlide === 0}
          className="flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border border-zinc-200 dark:border-white/10 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-white/5 disabled:opacity-50 disabled:pointer-events-none transition-colors"
        >
          <ArrowLeft size={16} />
          Back
        </button>

        {/* Bullet page Indicators */}
        <div className="flex items-center gap-2">
          {Array.from({ length: totalSlides }).map((_, idx) => (
            <button
              key={idx}
              onClick={() => setCurrentSlide(idx)}
              className={`w-2.5 h-2.5 rounded-full transition-all duration-300 ${
                currentSlide === idx 
                  ? 'bg-black dark:bg-white scale-125' 
                  : 'bg-zinc-300 dark:bg-white/20'
              }`}
            />
          ))}
        </div>

        {currentSlide === totalSlides - 1 ? (
          <button
            onClick={() => setCurrentSlide(0)}
            className="flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl bg-black text-white dark:bg-white dark:text-black hover:opacity-90 transition-opacity"
          >
            Restart Demo
            <RefreshCw size={16} />
          </button>
        ) : (
          <button
            onClick={() => setCurrentSlide((prev) => prev + 1)}
            className="flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl bg-black text-white dark:bg-white dark:text-black hover:opacity-90 transition-opacity"
          >
            Next Slide
            <ArrowRight size={16} />
          </button>
        )}
      </div>
      
    </div>
  );
}
