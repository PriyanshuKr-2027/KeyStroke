import React, { useState } from "react";
import { GrammarStatus, GrammarFix, GrammarMode } from "../types";
import { SettingsRowGroup } from "./SettingsRowGroup";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { CheckCircle2 } from "lucide-react";

interface GrammarTabProps {
  status: GrammarStatus;
  fixes: GrammarFix[];
  onToggle: (enabled: boolean) => void;
  onModeChange: (mode: GrammarMode) => void;
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
}

export const GrammarTab: React.FC<GrammarTabProps> = ({
  status,
  fixes,
  onToggle,
  onModeChange,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [symSpell, setSymSpell] = useState(status.enabled);
  const [homophone, setHomophone] = useState(true);
  const [languageTool, setLanguageTool] = useState(true);
  const [nextWord, setNextWord] = useState(true);

  const [sensitivity, setSensitivity] = useState(90);
  const [language, setLanguage] = useState(status.language || "English (US)");

  const [sandboxText, setSandboxText] = useState("");

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="Grammar Engine Offline"
          message={errorMessage || "Could not connect to the local LanguageTool / SymSpell grammar daemon."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-8 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Header */}
      <h1 className="font-sans text-[22px] font-semibold text-[#111111]">
        Grammar & Autocorrect
      </h1>

      {/* Section 1 — Operating Mode */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          OPERATING MODE
        </div>
        <div className="bg-[#F5F5F5] rounded-[10px] p-1 space-y-1">
          <button
            onClick={() => onModeChange("Aggressive")}
            className={`w-full px-4 h-[48px] rounded-[8px] flex items-center justify-between text-[14px] cursor-pointer transition ${
              status.mode === "Aggressive"
                ? "bg-[#FFFFFF] text-[#111111] shadow-sm font-medium"
                : "text-[#6B6B6B] hover:text-[#111111]"
            }`}
          >
            <span>Aggressive — fix on Space / punctuation</span>
            <div
              className={`w-4 h-4 rounded-full border flex items-center justify-center ${
                status.mode === "Aggressive"
                  ? "border-[#111111] bg-[#111111]"
                  : "border-[#D1D5DB]"
              }`}
            >
              {status.mode === "Aggressive" && (
                <div className="w-1.5 h-1.5 rounded-full bg-[#FFFFFF]" />
              )}
            </div>
          </button>

          <button
            onClick={() => onModeChange("Suggestions Only")}
            className={`w-full px-4 h-[48px] rounded-[8px] flex items-center justify-between text-[14px] cursor-pointer transition ${
              status.mode === "Suggestions Only"
                ? "bg-[#FFFFFF] text-[#111111] shadow-sm font-medium"
                : "text-[#6B6B6B] hover:text-[#111111]"
            }`}
          >
            <span>Suggestions only — show tooltip, wait for accept</span>
            <div
              className={`w-4 h-4 rounded-full border flex items-center justify-center ${
                status.mode === "Suggestions Only"
                  ? "border-[#111111] bg-[#111111]"
                  : "border-[#D1D5DB]"
              }`}
            >
              {status.mode === "Suggestions Only" && (
                <div className="w-1.5 h-1.5 rounded-full bg-[#FFFFFF]" />
              )}
            </div>
          </button>
        </div>
      </div>

      {/* Section 2 — Engine Controls */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          ENGINE CONTROLS
        </div>
        <SettingsRowGroup
          items={[
            {
              id: "symspell",
              label: "SymSpell Autocorrect",
              subtitle: "Sub-millisecond dictionary typo replacement",
              type: "toggle",
              checked: symSpell,
              onToggle: (v) => {
                setSymSpell(v);
                onToggle(v);
              },
            },
            {
              id: "homophone",
              label: "Homophone Resolution",
              subtitle: "Contextual disambiguation (there / their / they're)",
              type: "toggle",
              checked: homophone,
              onToggle: setHomophone,
            },
            {
              id: "languagetool",
              label: "LanguageTool Grammar Engine",
              subtitle: "Local rule evaluation engine",
              type: "toggle",
              checked: languageTool,
              onToggle: setLanguageTool,
            },
            {
              id: "nextword",
              label: "Next-Word Prediction",
              subtitle: "Gboard-style inline suggestion chip",
              type: "toggle",
              checked: nextWord,
              onToggle: setNextWord,
            },
          ]}
        />
      </div>

      {/* Section 3 — Sensitivity Slider */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          SENSITIVITY
        </div>
        <div className="bg-[#F5F5F5] rounded-[10px] p-4 space-y-3">
          <div className="flex items-center justify-between text-[14px]">
            <span className="text-[#111111]">Correction confidence threshold</span>
            <span className="font-mono font-medium text-[#111111]">{sensitivity}%</span>
          </div>

          <input
            type="range"
            min="70"
            max="99"
            value={sensitivity}
            onChange={(e) => setSensitivity(Number(e.target.value))}
            className="w-full h-2 bg-[#D1D5DB] rounded-lg appearance-none cursor-pointer accent-[#111111]"
          />
        </div>
      </div>

      {/* Section 4 — Language */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          LANGUAGE
        </div>
        <SettingsRowGroup
          items={[
            {
              id: "language",
              label: "Primary language",
              subtitle: language,
              type: "button",
              buttonLabel: "Change",
              onButtonClick: () =>
                setLanguage(language === "English (US)" ? "English (UK)" : "English (US)"),
            },
          ]}
        />
      </div>

      {/* Section 5 — Interactive Sandbox */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          TEST YOUR GRAMMAR ENGINE LIVE
        </div>
        <input
          type="text"
          value={sandboxText}
          onChange={(e) => setSandboxText(e.target.value)}
          placeholder="Type something here — corrections appear as you type..."
          className="w-full h-[44px] px-4 bg-[#F5F5F5] text-[#111111] placeholder-[#AAAAAA] text-[14px] rounded-[12px] focus:outline-none focus:ring-1 focus:ring-[#111111] transition"
        />
      </div>

      {/* Section 6 — Recent Corrections Log */}
      <div className="space-y-2">
        <div className="text-[12px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase">
          RECENT CORRECTIONS LOG
        </div>
        {isLoading ? (
          <TableRowSkeleton count={3} />
        ) : fixes.length > 0 ? (
          <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
            {fixes.map((row) => (
              <div
                key={row.id}
                className="h-[48px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors"
              >
                <div className="flex items-center gap-4 text-[14px]">
                  <span className="font-sans text-[12px] text-[#6B6B6B] w-[54px]">
                    {row.timestamp}
                  </span>

                  <div className="flex items-center gap-2">
                    <span className="font-mono text-[#111111]">{row.original}</span>
                    <span className="text-[#AAAAAA]">→</span>
                    <span className="font-sans text-[#6B6B6B]">{row.fixed}</span>
                  </div>

                  <span className="px-2 py-0.5 bg-[#FFFFFF] border border-[#EBEBEB] text-[#6B6B6B] text-[11px] rounded-[4px] font-sans uppercase">
                    {row.category}
                  </span>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <EmptyState
            icon={CheckCircle2}
            title="No recent corrections"
            description="When KeyStroke automatically fixes spelling mistakes, homophones, or grammar errors in other apps, they will be logged here."
          />
        )}
      </div>
    </div>
  );
};
