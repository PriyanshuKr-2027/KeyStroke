import React, { useState, useEffect } from "react";
import { EngineStatus, DailyStats, AutocorrectFeedItem } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton, StatCardSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Activity, Sparkles, CheckCheck, Undo2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/tauri";

interface DashboardTabProps {
  status: EngineStatus;
  stats: DailyStats;
  feed: AutocorrectFeedItem[];
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
  onShowToast?: (title: string, message?: string, type?: "success" | "error" | "info") => void;
}

export const DashboardTab: React.FC<DashboardTabProps> = ({
  status,
  stats,
  feed,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
  onShowToast,
}) => {
  const [activityFeed, setActivityFeed] = useState<
    (AutocorrectFeedItem & { app?: string; timestamp?: string; time_ago?: string })[]
  >(feed);

  useEffect(() => {
    setActivityFeed(feed);
  }, [feed]);

  const [sandboxText, setSandboxText] = useState("");
  const [showCallout, setShowCallout] = useState(true);

  const [sandboxCorrection, setSandboxCorrection] = useState<{ original: string; corrected: string } | null>(null);
  const [sandboxNextWord, setSandboxNextWord] = useState<string | null>(null);

  const handleUndo = (id: string) => {
    setActivityFeed((prev) => prev.filter((item) => item.id !== id));
    if (onShowToast) onShowToast("Correction Undone", "Reverted typo correction", "info");
  };

  useEffect(() => {
    let isCancelled = false;
    if (!sandboxText.trim()) {
      setSandboxCorrection(null);
      setSandboxNextWord(null);
      return;
    }

    const timer = setTimeout(async () => {
      const words = sandboxText.trim().split(/\s+/);
      const lastWord = words[words.length - 1];

      if (lastWord.length >= 2) {
        invoke<{ original: string; corrected: string } | null>("check_autocorrect_word", { word: lastWord })
          .then((res) => {
            if (!isCancelled && res && res.corrected.toLowerCase() !== res.original.toLowerCase()) {
              setSandboxCorrection(res);
            } else if (!isCancelled) {
              setSandboxCorrection(null);
            }
          })
          .catch(() => {});
      }

      invoke<{ candidate_word: string } | null>("predict_next_word", { context: sandboxText })
        .then((res) => {
          if (!isCancelled && res) {
            setSandboxNextWord(res.candidate_word);
          } else if (!isCancelled) {
            setSandboxNextWord(null);
          }
        })
        .catch(() => {});
    }, 200);

    return () => {
      isCancelled = true;
      clearTimeout(timer);
    };
  }, [sandboxText]);

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10 font-sans">
        <ErrorState
          title="Engine Disconnected"
          message={errorMessage || "Unable to communicate with the KeyStroke background engine."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  const statCards = [
    {
      label: "Words Typed",
      value: stats.words_typed.toLocaleString(),
      subtext: "Today",
      sparkline: "M0,20 L15,18 L30,12 L45,15 L60,8 L75,10 L90,2",
      color: "#22C55E",
    },
    {
      label: "Corrections Made",
      value: stats.corrections_made.toLocaleString(),
      subtext: "Auto-fixed",
      sparkline: "M0,15 L15,10 L30,18 L45,8 L60,12 L75,5 L90,3",
      color: "#6366F1",
    },
    {
      label: "Snippets Used",
      value: stats.variables_used.toLocaleString(),
      subtext: "Expansions",
      sparkline: "M0,18 L15,15 L30,14 L45,10 L60,8 L75,6 L90,4",
      color: "#F59E0B",
    },
  ];

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10 font-sans select-none text-[#EDEDED]">
      {/* Top Header Bar */}
      <div className="flex items-center justify-between">
        <h1 className="text-[22px] font-semibold text-[#EDEDED] tracking-tight">
          Dashboard
        </h1>

        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-[#22C55E] animate-pulse" />
          <span className="text-[12px] font-medium text-[#8F8F96]">Smart Writing Active</span>
        </div>
      </div>

      {showCallout && (
        <CalloutCard
          headline="KeyStroke is running smoothly in the background."
          body="Type anywhere in your daily apps — typos fix automatically, and next-word suggestions appear directly beside your cursor."
          chips={[{ label: "Smart Autocorrect" }, { label: "Predictive Typing" }]}
          ctaLabel="Test sandbox"
          onCtaClick={() => {}}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* Metric Cards Grid */}
      {isLoading ? (
        <StatCardSkeleton />
      ) : (
        <div className="grid grid-cols-3 gap-4">
          {statCards.map((card, i) => (
            <div
              key={i}
              className="bg-[#161618] border border-[rgba(255,255,255,0.08)] p-4 rounded-[12px] shadow-sm flex flex-col justify-between h-[104px] relative overflow-hidden"
            >
              <div>
                <span className="text-[12px] font-medium text-[#8F8F96]">{card.label}</span>
                <p className="text-[24px] font-semibold text-[#EDEDED] tracking-tight mt-0.5">
                  {card.value}
                </p>
              </div>

              {/* Sparkline SVG */}
              <div className="absolute right-3 bottom-3 opacity-60">
                <svg width="90" height="24" viewBox="0 0 90 24" fill="none">
                  <path d={card.sparkline} stroke={card.color} strokeWidth="2" strokeLinecap="round" />
                </svg>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Live Interactive Sandbox */}
      <div className="bg-[#161618] border border-[rgba(255,255,255,0.08)] rounded-[12px] p-4 space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-[12px] font-mono font-semibold text-[#6366F1] uppercase tracking-wider">
            INTERACTIVE TYPING SANDBOX
          </span>
          <span className="text-[11px] text-[#8F8F96]">Type below to test live engines</span>
        </div>

        <textarea
          rows={2}
          value={sandboxText}
          onChange={(e) => setSandboxText(e.target.value)}
          placeholder="Type here to test autocorrect (e.g. 'teh', 'recieve') or next-word prediction..."
          className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] p-3 text-[13px] text-[#EDEDED] placeholder-[#5C5C62] focus:outline-none focus:border-[#6366F1] resize-none"
        />

        {(sandboxCorrection || sandboxNextWord) && (
          <div className="flex items-center gap-3 pt-1 text-[12px]">
            {sandboxCorrection && (
              <span className="px-2.5 py-1 bg-[#22C55E]/10 border border-[#22C55E]/20 text-[#22C55E] rounded-[6px] font-mono">
                Autocorrect: {sandboxCorrection.original} → <strong>{sandboxCorrection.corrected}</strong>
              </span>
            )}
            {sandboxNextWord && (
              <span className="px-2.5 py-1 bg-[#6366F1]/10 border border-[#6366F1]/20 text-[#6366F1] rounded-[6px] font-mono">
                Next-word: <strong>{sandboxNextWord}</strong>
              </span>
            )}
          </div>
        )}
      </div>

      {/* Live Activity Feed */}
      <div className="space-y-3 pt-2">
        <div className="text-[11px] font-mono font-semibold tracking-wider text-[#8F8F96] uppercase">
          RECENT AUTOMATIC CORRECTIONS
        </div>

        {isLoading ? (
          <TableRowSkeleton count={3} />
        ) : activityFeed.length > 0 ? (
          <div className="divide-y divide-[rgba(255,255,255,0.08)] border-t border-b border-[rgba(255,255,255,0.08)]">
            {activityFeed.map((item) => (
              <div
                key={item.id}
                className="h-[48px] px-3 flex items-center justify-between hover:bg-[rgba(255,255,255,0.03)] transition rounded-[8px] group"
              >
                <div className="flex items-center gap-2 text-[13px]">
                  <span className="line-through text-[#8F8F96]">{item.original}</span>
                  <span className="text-[#8F8F96]">→</span>
                  <span className="font-semibold text-[#22C55E]">{item.corrected}</span>
                </div>

                <div className="flex items-center gap-3">
                  <span className="text-[11px] font-mono text-[#5C5C62]">
                    {item.time_ago || "Just now"}
                  </span>
                  <button
                    onClick={() => handleUndo(item.id)}
                    className="text-[#8F8F96] hover:text-[#EDEDED] opacity-0 group-hover:opacity-100 transition p-1"
                    title="Undo correction"
                  >
                    <Undo2 className="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <EmptyState
            icon={Activity}
            title="No recent corrections"
            description="As you type in any application, KeyStroke will auto-correct typos and log them here."
          />
        )}
      </div>
    </div>
  );
};
