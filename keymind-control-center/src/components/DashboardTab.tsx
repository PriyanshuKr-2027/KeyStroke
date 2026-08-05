import React, { useState } from "react";
import { EngineStatus, DailyStats, AutocorrectFeedItem } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton, StatCardSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Activity } from "lucide-react";

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
    (AutocorrectFeedItem & { app?: string; timestamp?: string })[]
  >(feed);

  React.useEffect(() => {
    setActivityFeed(feed);
  }, [feed]);

  const [sandboxText, setSandboxText] = useState("");
  const [showCallout, setShowCallout] = useState(true);

  const handleUndo = (id: string) => {
    setActivityFeed((prev) => prev.filter((item) => item.id !== id));
  };

  // Helper to render live corrections inline in the sandbox
  const getRenderedSandboxContent = () => {
    if (!sandboxText) return null;
    let rendered = sandboxText;

    const replacements: { target: string; replacement: string }[] = [
      { target: "teh", replacement: "the" },
      { target: "recieve", replacement: "receive" },
      { target: "there books", replacement: "their books" },
    ];

    let foundMatch: { original: string; fix: string } | null = null;
    for (const r of replacements) {
      if (rendered.includes(r.target)) {
        foundMatch = { original: r.target, fix: r.replacement };
        break;
      }
    }

    if (foundMatch) {
      const parts = sandboxText.split(foundMatch.original);
      return (
        <div className="mt-2.5 p-3 bg-[#FFFFFF] border border-[#EBEBEB] rounded-[8px] text-[13px] font-mono text-[#111111] flex items-center justify-between">
          <div>
            <span>{parts[0]}</span>
            <span className="text-[#EF4444] line-through bg-[#FEE2E2] px-1 py-0.5 rounded mr-1">
              {foundMatch.original}
            </span>
            <span className="text-[#22C55E] bg-[#DCFCE7] px-1 py-0.5 rounded font-bold">
              {foundMatch.fix}
            </span>
            <span>{parts.slice(1).join(foundMatch.original)}</span>
          </div>
          <span className="text-[11px] font-sans text-[#6B6B6B]">Auto-corrected live</span>
        </div>
      );
    }

    return null;
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
          headline="KeyMind types the way you think."
          body="Works in every app. Type /date, /email, or any trigger and KeyMind expands it instantly — with grammar fixes happening silently in the background."
          chips={[
            { trigger: "/date", arrow: "→", label: "August 5, 2026" },
            { trigger: "teh", arrow: "→", label: "the" },
            { trigger: "there", arrow: "→", label: "their" },
          ]}
          ctaLabel="See how it works"
          onCtaClick={() => window.open("https://github.com", "_blank")}
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
          TODAY — AUGUST 5, 2026
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
                    {item.timestamp || item.time_ago}
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
            description="KeyMind is running silently in the background. Start typing in any app to see live autocorrect and grammar fixes recorded here."
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
          placeholder="Try it: type something here to test KeyMind live..."
          className="w-full h-[44px] px-4 bg-[#F5F5F5] text-[#111111] placeholder-[#AAAAAA] text-[14px] rounded-[12px] focus:outline-none focus:ring-1 focus:ring-[#111111] transition"
        />

        {getRenderedSandboxContent()}
      </div>
    </div>
  );
};
