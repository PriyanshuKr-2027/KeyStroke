import React, { useState, useEffect } from "react";
import { EngineStatus, DailyStats, AutocorrectFeedItem } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton, StatCardSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Activity, Undo2 } from "lucide-react";
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
    if (!sandboxText.trim()) {
      setSandboxCorrection(null);
      setSandboxNextWord(null);
      return;
    }

    const timer = setTimeout(async () => {
      const words = sandboxText.trim().split(/\s+/);
      const lastWord = words[words.length - 1];

      try {
        const corrRes = await invoke<{ original: string; corrected: string } | null>("check_autocorrect_word", { word: lastWord });
        setSandboxCorrection(corrRes);
      } catch (e) {}

      try {
        const predRes = await invoke<{ candidate_word: string } | null>("predict_next_word", { context: sandboxText });
        setSandboxNextWord(predRes ? predRes.candidate_word : null);
      } catch (e) {}
    }, 200);

    return () => clearTimeout(timer);
  }, [sandboxText]);

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10 font-sans">
        <ErrorState
          title="Engine Disconnected"
          message={errorMessage || "Could not communicate with local KeyStroke interceptor daemon."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  const statCards = [
    {
      label: "Words Typed Today",
      value: stats.words_typed.toLocaleString(),
      subtext: "Words Processed",
      sparkline: "M0,18 L15,14 L30,16 L45,10 L60,12 L75,6 L90,4",
      color: "#DA7756",
    },
    {
      label: "Typos Auto-Fixed",
      value: stats.corrections_made.toLocaleString(),
      subtext: "Corrections Made",
      sparkline: "M0,18 L15,12 L30,15 L45,8 L60,10 L75,4 L90,2",
      color: "#22C55E",
    },
    {
      label: "Text Shortcuts Used",
      value: stats.variables_used.toLocaleString(),
      subtext: "Expansions",
      sparkline: "M0,18 L15,15 L30,14 L45,10 L60,8 L75,6 L90,4",
      color: "#F59E0B",
    },
  ];

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10 font-sans select-none text-[var(--text-primary)]">
      {/* Header Bar */}
      <div className="flex items-center justify-between">
        <h1 className="text-[22px] font-semibold tracking-tight text-[var(--text-primary)]">
          Dashboard
        </h1>

        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-[#22C55E]" />
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">Smart Writing Active</span>
        </div>
      </div>

      {showCallout && (
        <CalloutCard
          headline="KeyStroke is running smoothly in the background."
          body="Type anywhere in your daily apps — typos fix automatically, and next-word suggestions appear directly beside your cursor."
          chips={[{ label: "Smart Autocorrect" }, { label: "Predictive Typing" }]}
          ctaLabel="Test sandbox"
          onCtaClick={() => {
            const el = document.getElementById("sandbox-textarea");
            if (el) {
              el.focus();
              el.scrollIntoView({ behavior: "smooth", block: "center" });
            }
          }}
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
              className="bg-[var(--surface)] border border-[var(--border)] p-4 rounded-[12px] shadow-sm flex flex-col justify-between transition-colors"
            >
              <div>
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">{card.label}</span>
                <p className="text-[24px] font-semibold text-[var(--text-primary)] tracking-tight mt-1">
                  {card.value}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Live Interactive Sandbox */}
      <div className="bg-[var(--surface)] border border-[var(--border)] rounded-[12px] p-4 space-y-3 shadow-sm transition-colors">
        <div className="flex items-center justify-between">
          <span className="text-[12px] font-mono font-semibold text-[var(--accent)] uppercase tracking-wider">
            INTERACTIVE TYPING SANDBOX
          </span>
          <span className="text-[11px] text-[var(--text-secondary)]">Type below to test live engines</span>
        </div>

        <textarea
          id="sandbox-textarea"
          rows={2}
          value={sandboxText}
          onChange={(e) => setSandboxText(e.target.value)}
          placeholder="Type here to test autocorrect (e.g. 'teh', 'recieve') or next-word prediction..."
          className="w-full bg-[var(--bg-app)] border border-[var(--border)] rounded-[8px] p-3 text-[13px] text-[var(--text-primary)] placeholder-[var(--text-tertiary)] focus:outline-none focus:border-[var(--accent)] resize-none transition-colors"
        />

        {(sandboxCorrection || sandboxNextWord) && (
          <div className="flex items-center gap-3 pt-1 text-[12px]">
            {sandboxCorrection && (
              <span className="px-2.5 py-1 bg-[#22C55E]/10 border border-[#22C55E]/20 text-[#22C55E] rounded-[6px] font-mono">
                Autocorrect: {sandboxCorrection.original} → <strong>{sandboxCorrection.corrected}</strong>
              </span>
            )}
            {sandboxNextWord && (
              <span className="px-2.5 py-1 bg-[var(--accent)]/10 border border-[var(--accent)]/20 text-[var(--accent)] rounded-[6px] font-mono">
                Next-word: <strong>{sandboxNextWord}</strong>
              </span>
            )}
          </div>
        )}
      </div>

      {/* Live Activity Feed */}
      <div className="space-y-3 pt-2">
        <div className="text-[11px] font-mono font-semibold tracking-wider text-[var(--text-secondary)] uppercase">
          RECENT AUTOMATIC CORRECTIONS
        </div>

        {isLoading ? (
          <TableRowSkeleton count={3} />
        ) : activityFeed.length > 0 ? (
          <div className="bg-[var(--surface)] border border-[var(--border)] rounded-[12px] divide-y divide-[var(--border)] shadow-sm overflow-hidden transition-colors">
            {activityFeed.map((item) => (
              <div
                key={item.id}
                className="h-[48px] px-4 flex items-center justify-between hover:bg-[var(--surface-hover)] transition group"
              >
                <div className="flex items-center gap-2 text-[13px]">
                  <span className="line-through text-[var(--text-tertiary)]">{item.original}</span>
                  <span className="text-[var(--text-secondary)]">→</span>
                  <span className="font-semibold text-[#22C55E]">{item.corrected}</span>
                </div>

                <div className="flex items-center gap-3">
                  <span className="text-[11px] font-mono text-[var(--text-tertiary)]">
                    {item.time_ago || "Just now"}
                  </span>
                  <button
                    onClick={() => handleUndo(item.id)}
                    className="text-[var(--text-secondary)] hover:text-[var(--text-primary)] opacity-0 group-hover:opacity-100 transition p-1 cursor-pointer"
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
