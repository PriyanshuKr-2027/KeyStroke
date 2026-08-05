import React, { useState } from "react";
import { EngineStatus, DailyStats, AutocorrectFeedItem } from "../types";
import {
  Activity,
  Cpu,
  Bot,
  CheckCheck,
  Type,
  Wand2,
  FileCode,
  Sparkles,
  AlertTriangle,
  X,
  TrendingUp,
  ShieldCheck,
  Zap,
  Play,
  RotateCcw,
} from "lucide-react";

interface DashboardTabProps {
  status: EngineStatus;
  stats: DailyStats;
  feed: AutocorrectFeedItem[];
}

export const DashboardTab: React.FC<DashboardTabProps> = ({
  status,
  stats,
  feed,
}) => {
  const [rateLimitToast, setRateLimitToast] = useState<string | null>(null);
  const [testInput, setTestInput] = useState("thank you for your");
  const [testResult, setTestResult] = useState<string | null>(null);
  const [isSimulating, setIsSimulating] = useState(false);

  const handleRunQuickTest = (type: "prediction" | "autocorrect" | "grammar") => {
    setIsSimulating(true);
    setTestResult(null);

    setTimeout(() => {
      if (type === "prediction") {
        setTestResult('Next-word prediction match: "support" (Confidence: 98%)');
      } else if (type === "autocorrect") {
        setTestResult('SymSpell typo fix: "recieve" → "receive" (Confidence: 99%)');
      } else {
        setTestResult('Grammar fix: "He are going" → "He is going" (Rule: SUBJECT_VERB_AGREEMENT)');
      }
      setIsSimulating(false);
    }, 300);
  };

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Toast Alert */}
      {rateLimitToast && (
        <div className="bg-amber-500/10 border border-amber-500/30 rounded-2xl p-4 flex items-center justify-between text-amber-300 text-xs font-semibold shadow-xl backdrop-blur-xl animate-fade-in">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-amber-500/20 rounded-xl text-amber-400">
              <AlertTriangle className="w-4 h-4" />
            </div>
            <span>{rateLimitToast}</span>
          </div>
          <button
            onClick={() => setRateLimitToast(null)}
            className="text-amber-400/60 hover:text-amber-300 p-1 hover:bg-amber-500/20 rounded-lg transition"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      )}

      {/* Header Banner */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-2.5">
            System Overview
            <span className="text-xs px-2.5 py-0.5 rounded-full font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 flex items-center gap-1">
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
              Optimal
            </span>
          </h2>
          <p className="text-xs text-[#A1A0AB] mt-1">
            Real-time keyboard event processing and local AI assistant status.
          </p>
        </div>

        <div className="flex items-center gap-2 px-3.5 py-2 bg-[#1B1A22]/80 border border-[#DA7756]/20 rounded-2xl text-xs font-semibold text-[#FAF8F5] shadow-sm">
          <ShieldCheck className="w-4 h-4 text-[#DA7756]" />
          <span>Local Engine Verified</span>
        </div>
      </div>

      {/* Top Status Cards Grid */}
      <div className="grid grid-cols-3 gap-5">
        {/* Core Interceptor Card */}
        <div className="glass-panel glass-panel-hover p-5 rounded-3xl flex items-center justify-between relative overflow-hidden group">
          <div className="flex items-center gap-3.5">
            <div className="p-3 bg-[#DA7756]/15 rounded-2xl text-[#DA7756] border border-[#DA7756]/20 group-hover:scale-105 transition-transform">
              <Cpu className="w-5 h-5" />
            </div>
            <div>
              <p className="text-xs font-semibold text-[#A1A0AB]">Core Interceptor</p>
              <h4 className="text-base font-extrabold text-[#FAF8F5] capitalize mt-0.5">
                {status.engine}
              </h4>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[11px] font-mono font-bold text-[#A1A0AB]">P99 &lt;1ms</span>
            <span
              className={`w-3 h-3 rounded-full ${
                status.engine === "running"
                  ? "bg-emerald-400 shadow-md shadow-emerald-400/50 animate-pulse"
                  : "bg-rose-500"
              }`}
            />
          </div>
        </div>

        {/* Dual AI Engine Card */}
        <div className="glass-panel glass-panel-hover p-5 rounded-3xl flex items-center justify-between relative overflow-hidden group">
          <div className="flex items-center gap-3.5">
            <div className="p-3 bg-amber-500/15 rounded-2xl text-amber-400 border border-amber-500/20 group-hover:scale-105 transition-transform">
              <Bot className="w-5 h-5" />
            </div>
            <div>
              <p className="text-xs font-semibold text-[#A1A0AB]">AI Copilot Engine</p>
              <h4 className="text-base font-extrabold text-[#FAF8F5] capitalize mt-0.5">
                {status.ai}
              </h4>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[10px] font-extrabold px-2 py-0.5 rounded-full bg-[#DA7756]/20 text-[#DA7756] border border-[#DA7756]/30">
              Groq + Cerebras
            </span>
            <span
              className={`w-3 h-3 rounded-full ${
                status.ai === "connected"
                  ? "bg-emerald-400 shadow-md shadow-emerald-400/50"
                  : "bg-rose-500"
              }`}
            />
          </div>
        </div>

        {/* Grammar Engine Card */}
        <div className="glass-panel glass-panel-hover p-5 rounded-3xl flex items-center justify-between relative overflow-hidden group">
          <div className="flex items-center gap-3.5">
            <div className="p-3 bg-emerald-500/15 rounded-2xl text-emerald-400 border border-emerald-500/20 group-hover:scale-105 transition-transform">
              <CheckCheck className="w-5 h-5" />
            </div>
            <div>
              <p className="text-xs font-semibold text-[#A1A0AB]">LanguageTool Grammar</p>
              <h4 className="text-base font-extrabold text-[#FAF8F5] capitalize mt-0.5">
                {status.grammar}
              </h4>
            </div>
          </div>
          <span
            className={`w-3 h-3 rounded-full ${
              status.grammar === "ready"
                ? "bg-emerald-400 shadow-md shadow-emerald-400/50"
                : status.grammar === "starting"
                ? "bg-amber-400 animate-bounce"
                : "bg-rose-500"
            }`}
          />
        </div>
      </div>

      {/* Quick Interactive Engine Test Bar */}
      <div className="glass-panel p-5 rounded-3xl space-y-3 border border-[#DA7756]/20">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs font-bold text-[#FAF8F5]">
            <Zap className="w-4 h-4 text-[#DA7756]" />
            <span>Interactive Engine Tester</span>
          </div>
          <span className="text-[10px] text-[#A1A0AB]">Test interception live</span>
        </div>

        <div className="flex items-center gap-3">
          <input
            type="text"
            value={testInput}
            onChange={(e) => setTestInput(e.target.value)}
            placeholder="Type sample text to simulate..."
            className="flex-1 bg-[#121216] border border-white/10 rounded-2xl px-4 py-2.5 text-xs text-[#FAF8F5] font-mono focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
          />

          <button
            onClick={() => handleRunQuickTest("prediction")}
            disabled={isSimulating}
            className="px-3.5 py-2.5 bg-[#DA7756] hover:bg-[#C86544] text-white rounded-2xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer active:scale-95 shadow-md shadow-[#DA7756]/20"
          >
            <Play className="w-3.5 h-3.5 fill-white" /> Predict Next
          </button>

          <button
            onClick={() => handleRunQuickTest("autocorrect")}
            disabled={isSimulating}
            className="px-3.5 py-2.5 bg-amber-500/15 text-amber-300 hover:bg-amber-500/25 border border-amber-500/30 rounded-2xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer active:scale-95"
          >
            <Wand2 className="w-3.5 h-3.5 text-amber-400" /> SymSpell Typo
          </button>

          <button
            onClick={() => handleRunQuickTest("grammar")}
            disabled={isSimulating}
            className="px-3.5 py-2.5 bg-emerald-500/15 text-emerald-300 hover:bg-emerald-500/25 border border-emerald-500/30 rounded-2xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer active:scale-95"
          >
            <CheckCheck className="w-3.5 h-3.5 text-emerald-400" /> Grammar
          </button>
        </div>

        {testResult && (
          <div className="p-3 bg-[#17161D] rounded-2xl border border-emerald-500/30 text-xs font-mono text-emerald-300 flex items-center justify-between animate-fade-in">
            <span>{testResult}</span>
            <button onClick={() => setTestResult(null)} className="text-[#A1A0AB] hover:text-white p-1">
              <X className="w-3.5 h-3.5" />
            </button>
          </div>
        )}
      </div>

      {/* Metrics Row */}
      <div className="grid grid-cols-4 gap-5">
        <div className="glass-panel glass-panel-hover p-5 rounded-3xl space-y-2">
          <div className="flex items-center justify-between text-[#A1A0AB]">
            <span className="text-xs font-bold uppercase tracking-wider">Words Typed</span>
            <Type className="w-4 h-4 text-[#DA7756]" />
          </div>
          <div className="flex items-baseline justify-between">
            <p className="text-3xl font-black tracking-tight text-[#FAF8F5] font-mono">
              {stats.words_typed.toLocaleString()}
            </p>
            <span className="text-xs font-semibold text-emerald-400 flex items-center gap-0.5">
              <TrendingUp className="w-3 h-3" /> +12%
            </span>
          </div>
        </div>

        <div className="glass-panel glass-panel-hover p-5 rounded-3xl space-y-2">
          <div className="flex items-center justify-between text-[#A1A0AB]">
            <span className="text-xs font-bold uppercase tracking-wider">Autocorrects</span>
            <Wand2 className="w-4 h-4 text-amber-400" />
          </div>
          <div className="flex items-baseline justify-between">
            <p className="text-3xl font-black tracking-tight text-[#FAF8F5] font-mono">
              {stats.corrections_made.toLocaleString()}
            </p>
            <span className="text-xs font-semibold text-emerald-400 flex items-center gap-0.5">
              <TrendingUp className="w-3 h-3" /> +8%
            </span>
          </div>
        </div>

        <div className="glass-panel glass-panel-hover p-5 rounded-3xl space-y-2">
          <div className="flex items-center justify-between text-[#A1A0AB]">
            <span className="text-xs font-bold uppercase tracking-wider">Variables Used</span>
            <FileCode className="w-4 h-4 text-sky-400" />
          </div>
          <div className="flex items-baseline justify-between">
            <p className="text-3xl font-black tracking-tight text-[#FAF8F5] font-mono">
              {stats.variables_used.toLocaleString()}
            </p>
            <span className="text-xs font-semibold text-[#71707C]">Today</span>
          </div>
        </div>

        <div className="glass-panel glass-panel-hover p-5 rounded-3xl space-y-2">
          <div className="flex items-center justify-between text-[#A1A0AB]">
            <span className="text-xs font-bold uppercase tracking-wider">AI Prompts</span>
            <Sparkles className="w-4 h-4 text-[#DA7756]" />
          </div>
          <div className="flex items-baseline justify-between">
            <p className="text-3xl font-black tracking-tight text-[#FAF8F5] font-mono">
              {stats.ai_requests.toLocaleString()}
            </p>
            <span className="text-xs font-semibold text-amber-400 flex items-center gap-1">
              <Zap className="w-3 h-3" /> Fast
            </span>
          </div>
        </div>
      </div>

      {/* Bottom Row: Activity Feed & Grammar Summary */}
      <div className="grid grid-cols-2 gap-6">
        {/* Recent Autocorrect Feed */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2.5">
              <Activity className="w-4 h-4 text-[#DA7756]" />
              Live Autocorrect Feed
            </h3>
            <span className="text-xs font-semibold text-[#A1A0AB]">Last 10 events</span>
          </div>

          <div className="space-y-2.5 max-h-[300px] overflow-y-auto pr-1">
            {feed.map((item) => (
              <div
                key={item.id}
                className="flex items-center justify-between p-3 rounded-2xl bg-[#17161D]/80 border border-[#DA7756]/15 hover:border-[#DA7756]/40 transition-all text-xs"
              >
                <div className="flex items-center gap-2.5 font-mono">
                  <span className="text-rose-400 bg-rose-500/10 px-2 py-0.5 rounded border border-rose-500/20 line-through">
                    {item.original}
                  </span>
                  <span className="text-[#71707C]">→</span>
                  <span className="text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 font-bold">
                    {item.corrected}
                  </span>
                </div>
                <span className="text-[11px] font-medium text-[#71707C]">{item.time_ago}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Grammar Issue Summary */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2.5">
              <CheckCheck className="w-4 h-4 text-emerald-400" />
              Grammar & Style Breakdown
            </h3>
            <span className="text-xs font-extrabold text-[#DA7756] px-2.5 py-1 rounded-full bg-[#DA7756]/20 border border-[#DA7756]/30">
              18 Issues Resolved
            </span>
          </div>

          <div className="grid grid-cols-2 gap-4 pt-1">
            <div className="p-4 bg-[#17161D]/80 border border-[#DA7756]/15 rounded-2xl hover:border-amber-500/30 transition">
              <span className="text-xs font-semibold text-[#A1A0AB]">Typos Corrected</span>
              <p className="text-2xl font-black text-amber-400 mt-1 font-mono">9</p>
            </div>
            <div className="p-4 bg-[#17161D]/80 border border-[#DA7756]/15 rounded-2xl hover:border-[#DA7756]/30 transition">
              <span className="text-xs font-semibold text-[#A1A0AB]">Grammar Fixes</span>
              <p className="text-2xl font-black text-[#DA7756] mt-1 font-mono">5</p>
            </div>
            <div className="p-4 bg-[#17161D]/80 border border-[#DA7756]/15 rounded-2xl hover:border-sky-500/30 transition">
              <span className="text-xs font-semibold text-[#A1A0AB]">Punctuation</span>
              <p className="text-2xl font-black text-sky-400 mt-1 font-mono">3</p>
            </div>
            <div className="p-4 bg-[#17161D]/80 border border-[#DA7756]/15 rounded-2xl hover:border-emerald-500/30 transition">
              <span className="text-xs font-semibold text-[#A1A0AB]">Style Improvements</span>
              <p className="text-2xl font-black text-emerald-400 mt-1 font-mono">1</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
