import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { GrammarStatus, GrammarFix, GrammarMode } from "../types";
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
  onShowToast?: (title: string, message?: string, type?: "success" | "error" | "info") => void;
}

export const GrammarTab: React.FC<GrammarTabProps> = ({
  status,
  onModeChange,
  isError = false,
  errorMessage = "",
  onRetry,
  onShowToast,
}) => {
  const [symSpell, setSymSpell] = useState(true);
  const [homophone, setHomophone] = useState(true);
  const [nlprule, setNlprule] = useState(true);
  const [nextWord, setNextWord] = useState(true);

  const [sandboxText, setSandboxText] = useState("");
  const [sandboxResult, setSandboxResult] = useState<{ fixed: string; issuesCount: number } | null>(null);

  useEffect(() => {
    invoke<{ autocorrect_enabled: boolean; prediction_enabled: boolean; grammar_enabled: boolean; homophone_enabled: boolean }>("get_feature_toggles")
      .then((t) => {
        if (t) {
          setSymSpell(t.autocorrect_enabled);
          setNextWord(t.prediction_enabled);
          setNlprule(t.grammar_enabled);
          setHomophone(t.homophone_enabled);
        }
      })
      .catch(() => {});
  }, []);

  const handleToggleFeature = (feature: string, current: boolean, setter: (val: boolean) => void, label: string) => {
    const next = !current;
    setter(next);
    invoke("update_feature_toggle", { feature, enabled: next })
      .then(() => {
        if (onShowToast) onShowToast(label, next ? "Feature Enabled" : "Feature Disabled", next ? "success" : "info");
      })
      .catch(() => {});
  };

  useEffect(() => {
    if (!sandboxText.trim()) {
      setSandboxResult(null);
      return;
    }
    const timer = setTimeout(async () => {
      try {
        const res = await invoke<{ original: string; fixed: string; issues: string[] }>("check_grammar_text", { text: sandboxText });
        if (res) {
          setSandboxResult({ fixed: res.fixed, issuesCount: res.issues ? res.issues.length : 0 });
        }
      } catch (e) {}
    }, 300);
    return () => clearTimeout(timer);
  }, [sandboxText]);

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10 font-sans">
        <ErrorState
          title="Grammar Engine Offline"
          message={errorMessage || "Could not connect to the local nlprule grammar daemon."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10 font-sans select-none text-[#EDEDED]">
      <h1 className="text-[22px] font-semibold text-[#EDEDED] tracking-tight">
        Grammar & Correction Engines
      </h1>

      {/* Operating Mode */}
      <div className="space-y-2">
        <div className="text-[11px] font-mono font-semibold tracking-wider text-[#8F8F96] uppercase">
          CORRECTION MODE
        </div>
        <div className="bg-[#161618] border border-[rgba(255,255,255,0.08)] rounded-[10px] p-1.5 space-y-1">
          {[
            { id: "Aggressive", label: "Aggressive — Fix immediately on Space or punctuation" },
            { id: "Passive", label: "Passive — Underline only, fix on manual hotkey" },
            { id: "Off", label: "Off — Pause all automatic grammar changes" },
          ].map((modeItem) => {
            const isSelected = status.mode === modeItem.id;
            return (
              <button
                key={modeItem.id}
                onClick={() => {
                  onModeChange(modeItem.id as GrammarMode);
                  if (onShowToast) onShowToast("Grammar Mode", `Switched to ${modeItem.id}`);
                }}
                className={`w-full px-4 h-[44px] rounded-[8px] flex items-center justify-between text-[13px] cursor-pointer transition ${
                  isSelected
                    ? "bg-[#28282D] text-[#EDEDED] font-medium shadow-sm border border-[rgba(255,255,255,0.08)]"
                    : "text-[#8F8F96] hover:text-[#EDEDED]"
                }`}
              >
                <span>{modeItem.label}</span>
                <div
                  className={`w-4 h-4 rounded-full border flex items-center justify-center ${
                    isSelected ? "border-[#6366F1] bg-[#6366F1]" : "border-[rgba(255,255,255,0.2)]"
                  }`}
                >
                  {isSelected && <CheckCircle2 className="w-3.5 h-3.5 text-white" />}
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* Toggles Group */}
      <div className="space-y-2 pt-2">
        <div className="text-[11px] font-mono font-semibold tracking-wider text-[#8F8F96] uppercase">
          WRITING ASSISTANT FEATURES
        </div>
        <div className="bg-[#161618] border border-[rgba(255,255,255,0.08)] rounded-[10px] divide-y divide-[rgba(255,255,255,0.08)]">
          {[
            { key: "grammar", label: "Grammar & Punctuation Assistant", desc: "Catches sentence errors and missing punctuation", state: nlprule, set: setNlprule },
            { key: "autocorrect", label: "Smart Autocorrect", desc: "Fixes typos automatically as you type", state: symSpell, set: setSymSpell },
            { key: "homophone", label: "Homophone Fixer", desc: "Corrects words like 'their', 'there', and 'they're'", state: homophone, set: setHomophone },
            { key: "prediction", label: "Predictive Typing Suggestions", desc: "Shows next-word suggestions right next to your cursor", state: nextWord, set: setNextWord },
          ].map((row) => (
            <div key={row.key} className="px-4 py-3 flex items-center justify-between">
              <div>
                <p className="text-[13px] font-medium text-[#EDEDED]">{row.label}</p>
                <p className="text-[11px] text-[#8F8F96]">{row.desc}</p>
              </div>
              <button
                onClick={() => handleToggleFeature(row.key, row.state, row.set, row.label)}
                className={`w-10 h-5 rounded-full transition relative cursor-pointer ${
                  row.state ? "bg-[#6366F1]" : "bg-[#28282D]"
                }`}
              >
                <span
                  className={`w-3.5 h-3.5 bg-white rounded-full absolute top-0.75 transition-all ${
                    row.state ? "left-5.5" : "left-1"
                  }`}
                />
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Sandbox Tester */}
      <div className="bg-[#161618] border border-[rgba(255,255,255,0.08)] rounded-[10px] p-4 space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-[11px] font-mono font-semibold text-[#6366F1] uppercase">
            SENTENCE GRAMMAR TESTER
          </span>
          <span className="text-[11px] text-[#8F8F96]">Powered by nlprule</span>
        </div>

        <input
          type="text"
          value={sandboxText}
          onChange={(e) => setSandboxText(e.target.value)}
          placeholder="Type sentence (e.g. 'He are going to teh store.')"
          className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] px-3.5 py-2 text-[13px] text-[#EDEDED] placeholder-[#5C5C62] focus:outline-none focus:border-[#6366F1]"
        />

        {sandboxResult && (
          <div className="p-3 bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] flex items-center justify-between text-[13px]">
            <span className="text-[#8F8F96]">Corrected Output:</span>
            <span className="font-semibold text-[#22C55E]">{sandboxResult.fixed}</span>
          </div>
        )}
      </div>
    </div>
  );
};
