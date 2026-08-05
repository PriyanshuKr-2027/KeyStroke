import React, { useState } from "react";
import { EngineStatus, DailyStats, AutocorrectFeedItem } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton, StatCardSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Activity } from "lucide-react";

import { invoke } from "@tauri-apps/api/tauri";

interface DashboardTabProps {
  status: EngineStatus;
  stats: DailyStats;
  feed: AutocorrectFeedItem[];
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
}

export const DashboardTab: React.FC<DashboardTabProps> = ({
  status,
  stats,
  feed,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [activityFeed, setActivityFeed] = useState<
    (AutocorrectFeedItem & { app?: string; timestamp?: string; time_ago?: string })[]
  >(feed);

  const getRelativeTime = (timestamp?: string, timeAgo?: string) => {
    if (timeAgo) return timeAgo;
    if (!timestamp) return "Just now";
    const date = new Date(timestamp);
    if (isNaN(date.getTime())) return timestamp;
    const now = new Date();
    const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);
    
    if (diffInSeconds < 60) return `${Math.max(0, diffInSeconds)}s ago`;
    if (diffInSeconds < 3600) return `${Math.floor(diffInSeconds / 60)}m ago`;
    if (diffInSeconds < 86400) return `${Math.floor(diffInSeconds / 3600)}h ago`;
    return `${Math.floor(diffInSeconds / 86400)}d ago`;
  };

  React.useEffect(() => {
    setActivityFeed(feed);
  }, [feed]);

  const [sandboxText, setSandboxText] = useState("");
  const [showCallout, setShowCallout] = useState(true);

  // Live sandbox detection state
  const [sandboxCorrection, setSandboxCorrection] = useState<{
    original: string;
    corrected: string;
  } | null>(null);
  const [sandboxNextWord, setSandboxNextWord] = useState<string | null>(null);
  const [sandboxGrammarFix, setSandboxGrammarFix] = useState<{
    original: string;
    fixed: string;
    issue: string;
  } | null>(null);

  const handleUndo = (id: string) => {
    setActivityFeed((prev) => prev.filter((item) => item.id !== id));
    invoke("undo_correction", { id }).catch(console.error);
  };

  // Run dynamic backend checks whenever sandbox text changes
  React.useEffect(() => {
    let isCancelled = false;

    if (!sandboxText.trim()) {
      setSandboxCorrection(null);
      setSandboxNextWord(null);
      setSandboxGrammarFix(null);
      return;
    }

    const timer = setTimeout(async () => {
      const words = sandboxText.trim().split(/\s+/);
      const lastWord = words[words.length - 1];

      // 1. Check Autocorrect for last word
      if (lastWord.length >= 2) {
        invoke<{ original: string; corrected: string } | null>("check_autocorrect_word", {
          word: lastWord,
        })
          .then((res) => {
            if (isCancelled) return;
            if (res && res.corrected.toLowerCase() !== res.original.toLowerCase()) {
              setSandboxCorrection(res);
            } else {
              setSandboxCorrection(null);
            }
          })
          .catch(() => { if (!isCancelled) setSandboxCorrection(null); });
      } else {
        if (!isCancelled) setSandboxCorrection(null);
      }

      // 2. Predict next word
      invoke<{ candidate_word: string } | null>("predict_next_word", {
        context: sandboxText,
      })
        .then((res) => {
          if (isCancelled) return;
          if (res && res.candidate_word) {
            setSandboxNextWord(res.candidate_word);
          } else {
            setSandboxNextWord(null);
          }
        })
        .catch(() => { if (!isCancelled) setSandboxNextWord(null); });

      // 3. Check Grammar
      invoke<{ original: string; fixed: string; issues: string[] }>("check_grammar_text", {
        text: sandboxText,
      })
        .then((res) => {
          if (isCancelled) return;
          if (res && res.issues.length > 0 && res.fixed !== res.original) {
            setSandboxGrammarFix({
              original: res.original,
              fixed: res.fixed,
              issue: res.issues[0],
            });
          } else {
            setSandboxGrammarFix(null);
          }
        })
        .catch(() => { if (!isCancelled) setSandboxGrammarFix(null); });
    }, 300);

    return () => {
      isCancelled = true;
      clearTimeout(timer);
    };
  }, [sandboxText]);

  // Handle Tab key to accept predicted next word
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Tab" && sandboxNextWord) {
      e.preventDefault();
      setSandboxText((prev) => prev.trim() + " " + sandboxNextWord + " ");
      setSandboxNextWord(null);
    }
  };

  const getRenderedSandboxContent = () => {
    if (!sandboxText) return null;

    return (
      <div className="mt-3 space-y-2" aria-live="polite">
        {/* Autocorrect Pill */}
        {sandboxCorrection && (
          <div className="p-3 bg-[#FFFFFF] border border-[#EBEBEB] rounded-[8px] text-[13px] font-mono text-[#111111] flex items-center justify-between shadow-sm">
            <div>
              <span className="text-[#EF4444] line-through bg-[#FEE2E2] px-1.5 py-0.5 rounded mr-2">
                {sandboxCorrection.original}
              </span>
              <span className="text-[#22C55E] bg-[#DCFCE7] px-1.5 py-0.5 rounded font-bold">
                {sandboxCorrection.corrected}
              </span>
            </div>
            <span className="text-[11px] font-sans font-medium text-[#6B6B6B]">
              Autocorrect Suggestion
            </span>
          </div>
        )}

        {/* Grammar Fix Pill */}
        {sandboxGrammarFix && (
          <div className="p-3 bg-[#EFF6FF] border border-[#BFDBFE] rounded-[8px] text-[13px] text-[#1E40AF] flex items-center justify-between shadow-sm">
            <div>
              <span className="font-semibold mr-2">Grammar Fix:</span>
              <span className="font-mono">{sandboxGrammarFix.fixed}</span>
              <span className="text-[11px] block text-[#3B82F6] mt-0.5">
                {sandboxGrammarFix.issue}
              </span>
            </div>
            <span className="text-[11px] font-sans font-medium text-[#2563EB]">
              Grammar AI
            </span>
          </div>
        )}

        {/* Next-Word Prediction Ghost Pill */}
        {sandboxNextWord && (
          <div className="p-2.5 bg-[#F9FAFB] border border-[#E5E7EB] rounded-[8px] text-[13px] font-sans text-[#374151] flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-[11px] bg-[#E5E7EB] text-[#4B5563] px-1.5 py-0.5 rounded font-mono font-medium">
                Tab ↹
              </span>
              <span>Next word suggestion:</span>
              <span className="font-semibold text-[#111827] bg-[#FEF3C7] px-1.5 py-0.5 rounded">
                {sandboxNextWord}
              </span>
            </div>
            <span className="text-[11px] text-[#9CA3AF]">Press Tab to insert</span>
          </div>
        )}
      </div>
    );
  };

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="Engine Status Unreachable"
          message={errorMessage || "Unable to retrieve dashboard stats and engine status."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  const todayStr = new Date().toLocaleDateString("en-US", { month: "long", day: "numeric", year: "numeric" });

  return (
    <div className="space-y-7 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Top Greeting & Streak Stats Row */}
      <div className="flex items-baseline justify-between border-b border-[#EBEBEB] pb-4">
        <h1 className="font-sans text-[22px] font-semibold text-[#111111] tracking-tight">
          Welcome back
        </h1>
        {isLoading ? (
          <StatCardSkeleton />
        ) : (
          <div className="flex items-center gap-3 text-[13px] font-sans text-[#6B6B6B]">
            <span>✦ {stats.words_typed} words</span>
            <span className="text-[#AAAAAA]">|</span>
            <span>⚡ {stats.corrections_made} corrections</span>
            <span className="text-[#AAAAAA]">|</span>
            <span>🤖 {stats.ai_requests} AI requests</span>
          </div>
        )}
      </div>

      {/* Callout Card — First Week Introduction */}
      {showCallout && (
        <CalloutCard
          headline="KeyStroke types the way you think."
          body="Works in every app. Type /date, /email, or any trigger and KeyStroke expands it instantly — with grammar fixes happening silently in the background."
          chips={[
            { trigger: "/date", arrow: "→", label: todayStr },
            { trigger: "teh", arrow: "→", label: "the" },
            { trigger: "there", arrow: "→", label: "their" },
          ]}
          ctaLabel="See how it works"
          onCtaClick={() => window.open("#", "_blank")}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* Engine Status Bar */}
      <div>
        <div className="text-[11px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase mb-2.5">
          ENGINE STATUS
        </div>
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2">
            <span
              className={`w-2 h-2 rounded-full ${
                status.engine === "running" ? "bg-[#22C55E]" : "bg-[#EF4444]"
              }`}
            />
            <span className="text-[13px] font-sans text-[#6B6B6B]">
              Keyboard interceptor {status.engine}
            </span>
          </div>

          <div className="flex items-center gap-2">
            <span
              className={`w-2 h-2 rounded-full ${
                status.grammar === "ready" ? "bg-[#22C55E]" : "bg-[#F59E0B]"
              }`}
            />
            <span className="text-[13px] font-sans text-[#6B6B6B]">
              Grammar server connected
            </span>
          </div>

          <div className="flex items-center gap-2">
            <span
              className={`w-2 h-2 rounded-full ${
                status.ai === "connected" ? "bg-[#22C55E]" : "bg-[#EF4444]"
              }`}
            />
            <span className="text-[13px] font-sans text-[#6B6B6B]">
              Groq API ready
            </span>
          </div>
        </div>
      </div>

      <div className="border-t border-[#EBEBEB] my-4" />

      {/* Today Activity Log */}
      <div>
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase mb-3">
          {"TODAY — " + todayStr.toUpperCase()}
        </div>

        {isLoading ? (
          <TableRowSkeleton count={3} />
        ) : activityFeed.length > 0 ? (
          <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
            {activityFeed.map((item) => (
              <div
                key={item.id}
                className="h-[48px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors group"
              >
                <div className="flex items-center gap-4 text-[14px]">
                  <span className="font-sans text-[12px] text-[#6B6B6B] w-[54px]">
                    {getRelativeTime(item.timestamp, item.time_ago)}
                  </span>

                  <div className="flex items-center gap-2">
                    <span className="font-mono text-[#111111]">{item.original}</span>
                    <span className="text-[#AAAAAA]">→</span>
                    <span className="font-sans text-[#6B6B6B]">{item.corrected}</span>
                  </div>

                  {item.app && (
                    <span className="text-[13px] text-[#AAAAAA]">({item.app})</span>
                  )}
                </div>

                <button
                  onClick={() => handleUndo(item.id)}
                  className="text-[13px] font-sans text-[#6B6B6B] hover:text-[#111111] hover:underline cursor-pointer"
                >
                  Undo
                </button>
              </div>
            ))}
          </div>
        ) : (
          <EmptyState
            icon={Activity}
            title="No corrections made today"
            description="KeyStroke is running silently in the background. Start typing in any app to see live autocorrect and grammar fixes recorded here."
            actionLabel="Try Live Sandbox below"
            onAction={() => {
              const sandboxInput = document.querySelector<HTMLInputElement>("input[placeholder*='Try it:']");
              sandboxInput?.focus();
            }}
          />
        )}
      </div>

      {/* Interactive Engine Sandbox */}
      <div className="pt-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase mb-2">
          LIVE ENGINE SANDBOX
        </div>

        <input
          type="text"
          value={sandboxText}
          onChange={(e) => setSandboxText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Try it: type something here (e.g. 'recieve', 'there books', 'thank you')..."
          className="w-full h-[44px] px-4 bg-[#F5F5F5] text-[#111111] placeholder-[#AAAAAA] text-[14px] rounded-[12px] focus:outline-none focus:ring-1 focus:ring-[#111111] transition"
        />

        {getRenderedSandboxContent()}
      </div>
    </div>
  );
};
