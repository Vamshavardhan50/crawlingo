import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Mail, User, Building, MessageSquare, CheckCircle2, 
  Loader2, ArrowRight, Sparkles, ShieldCheck 
} from 'lucide-react';

export default function WaitlistPage() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [company, setCompany] = useState('');
  const [useCase, setUseCase] = useState('');

  const [isLoading, setIsLoading] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    // Simple client-side validation
    if (!name.trim()) {
      setError('Please enter your name.');
      return;
    }
    if (!email.trim() || !/\S+@\S+\.\S+/.test(email)) {
      setError('Please enter a valid email address.');
      return;
    }

    setIsLoading(true);

    try {
      const response = await fetch('/api/waitlist', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: name.trim(),
          email: email.trim(),
          company: company.trim(),
          useCase: useCase.trim(),
        }),
      });

      const result = await response.json();

      if (!response.ok) {
        throw new Error(result.error || 'Failed to sign up. Please try again.');
      }

      setIsSuccess(true);
    } catch (err: any) {
      setError(err.message || 'An unexpected error occurred.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="relative min-h-[calc(100vh-8rem)] flex flex-col items-center justify-center py-12 px-4 overflow-hidden">
      
      {/* Decorative Radial Background */}
      <div className="absolute inset-0 bg-[linear-gradient(to_right,#8080800a_1px,transparent_1px),linear-gradient(to_bottom,#8080800a_1px,transparent_1px)] bg-[size:24px_24px] pointer-events-none" />
      <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-zinc-900/50 dark:bg-white/5 rounded-full blur-[120px] pointer-events-none" />

      <div className="relative z-10 w-full max-w-lg space-y-8">
        
        {/* Page Header */}
        <div className="text-center space-y-4">
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.4 }}
            className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-zinc-100 dark:bg-white/5 border border-zinc-200 dark:border-white/10 text-xs font-mono font-semibold text-zinc-600 dark:text-zinc-400"
          >
            <Sparkles size={12} className="text-amber-500 animate-pulse" />
            Crawlingo Private Beta v1.2
          </motion.div>

          <motion.h1
            initial={{ opacity: 0, y: 15 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.1 }}
            className="text-display-md md:text-display-lg font-black tracking-tight text-black dark:text-white"
          >
            Build Scrapers That <span className="underline decoration-wavy decoration-1 decoration-zinc-400 dark:decoration-zinc-600">Never Break</span>
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 15 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.2 }}
            className="text-sm md:text-base text-zinc-500 dark:text-zinc-400 max-w-md mx-auto"
          >
            Join the waitlist to get early access to our Rust-powered self-healing DOM crawling SDKs and real-time webhook endpoints.
          </motion.p>
        </div>

        {/* Form Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.3 }}
          className="bg-white/75 dark:bg-zinc-950/80 backdrop-blur-xl border border-zinc-200 dark:border-white/10 rounded-2xl p-6 md:p-8 shadow-xl"
        >
          <AnimatePresence mode="wait">
            {!isSuccess ? (
              <motion.form
                key="waitlist-form"
                onSubmit={handleSubmit}
                className="space-y-5"
                exit={{ opacity: 0, scale: 0.95 }}
                transition={{ duration: 0.3 }}
              >
                {/* Error Banner */}
                {error && (
                  <motion.div
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="p-3 bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-400 text-xs rounded-lg font-medium"
                  >
                    {error}
                  </motion.div>
                )}

                {/* Name Field */}
                <div className="space-y-1.5">
                  <label htmlFor="name" className="text-xs font-semibold text-zinc-600 dark:text-zinc-400 flex items-center gap-1.5">
                    <User size={13} /> Full Name <span className="text-red-500">*</span>
                  </label>
                  <input
                    id="name"
                    type="text"
                    required
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    placeholder="John Doe"
                    disabled={isLoading}
                    className="w-full px-3 py-2 text-sm bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl text-black dark:text-white placeholder-zinc-400 dark:placeholder-zinc-600 outline-none focus:border-black dark:focus:border-white transition-all duration-200"
                  />
                </div>

                {/* Email Field */}
                <div className="space-y-1.5">
                  <label htmlFor="email" className="text-xs font-semibold text-zinc-600 dark:text-zinc-400 flex items-center gap-1.5">
                    <Mail size={13} /> Email Address <span className="text-red-500">*</span>
                  </label>
                  <input
                    id="email"
                    type="email"
                    required
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="john@company.com"
                    disabled={isLoading}
                    className="w-full px-3 py-2 text-sm bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl text-black dark:text-white placeholder-zinc-400 dark:placeholder-zinc-600 outline-none focus:border-black dark:focus:border-white transition-all duration-200"
                  />
                </div>

                {/* Company Field */}
                <div className="space-y-1.5">
                  <label htmlFor="company" className="text-xs font-semibold text-zinc-600 dark:text-zinc-400 flex items-center gap-1.5">
                    <Building size={13} /> Organization / Company <span className="text-zinc-400 dark:text-zinc-500 text-[10px] font-normal">(Optional)</span>
                  </label>
                  <input
                    id="company"
                    type="text"
                    value={company}
                    onChange={(e) => setCompany(e.target.value)}
                    placeholder="Acme Corp"
                    disabled={isLoading}
                    className="w-full px-3 py-2 text-sm bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl text-black dark:text-white placeholder-zinc-400 dark:placeholder-zinc-600 outline-none focus:border-black dark:focus:border-white transition-all duration-200"
                  />
                </div>

                {/* Use Case Field */}
                <div className="space-y-1.5">
                  <label htmlFor="useCase" className="text-xs font-semibold text-zinc-600 dark:text-zinc-400 flex items-center gap-1.5">
                    <MessageSquare size={13} /> What are you scraping? <span className="text-zinc-400 dark:text-zinc-500 text-[10px] font-normal">(Optional)</span>
                  </label>
                  <textarea
                    id="useCase"
                    value={useCase}
                    onChange={(e) => setUseCase(e.target.value)}
                    placeholder="e.g. E-commerce prices, stock updates, real estate listings..."
                    disabled={isLoading}
                    rows={3}
                    className="w-full px-3 py-2 text-sm bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-white/5 rounded-xl text-black dark:text-white placeholder-zinc-400 dark:placeholder-zinc-600 outline-none focus:border-black dark:focus:border-white transition-all duration-200 resize-none"
                  />
                </div>

                {/* Submit Button */}
                <button
                  type="submit"
                  disabled={isLoading}
                  className="w-full py-2.5 px-4 rounded-xl bg-black dark:bg-white text-white dark:text-black font-semibold text-sm hover:opacity-90 transition-all flex items-center justify-center gap-2 disabled:opacity-50 disabled:pointer-events-none mt-2"
                >
                  {isLoading ? (
                    <>
                      <Loader2 size={16} className="animate-spin" />
                      Securing your spot...
                    </>
                  ) : (
                    <>
                      Request Early Access
                      <ArrowRight size={16} />
                    </>
                  )}
                </button>
              </motion.form>
            ) : (
              <motion.div
                key="waitlist-success"
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.4, ease: [0.16, 1, 0.3, 1] }}
                className="text-center py-6 space-y-5"
              >
                <div className="mx-auto w-12 h-12 rounded-full bg-green-500/10 border border-green-500/20 flex items-center justify-center text-green-500">
                  <CheckCircle2 size={28} />
                </div>
                
                <div className="space-y-2">
                  <h3 className="text-lg font-bold text-black dark:text-white">You're on the list!</h3>
                  <p className="text-xs text-zinc-500 dark:text-zinc-400 max-w-sm mx-auto leading-relaxed">
                    Thank you for signing up, <span className="font-semibold text-black dark:text-white">{name}</span>. 
                    We've saved your spot. An invite link will be dispatched to <span className="font-semibold text-black dark:text-white">{email}</span> as soon as slots open up.
                  </p>
                </div>

                <div className="pt-4 border-t border-zinc-100 dark:border-white/5 flex items-center justify-center gap-2 text-[10px] font-mono text-zinc-400">
                  <ShieldCheck size={14} className="text-zinc-500" />
                  Your data is securely logged in waitlist.csv
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </motion.div>

        {/* Waitlist Badge Counter Illustration */}
        <div className="text-center">
          <span className="text-[10px] font-mono font-bold text-zinc-400 uppercase tracking-widest">
            Join 500+ developers already in line
          </span>
        </div>
        
      </div>
    </div>
  );
}
