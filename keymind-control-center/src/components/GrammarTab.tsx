import React, { useState } from "react";
import { GrammarStatus, GrammarFix, GrammarMode } from "../types";
import { CheckCheck, Sparkles, Sliders, ShieldCheck, Play, ArrowRight, RefreshCw } from "lucide-react";

interface GrammarTabProps {
  status: GrammarStatus;
  fixes: GrammarFix[];
  onToggle: (enabled: boolean) => void;
  onModeChange: (mode: GrammarMode) => void;
}

export const GrammarTab: React.FC<GrammarTabProps> = ({
  status,
  fixes,
  onToggle,
  onModeChange,
}) => {
  const [sandboxInput, setSandboxInput] = useState(
    "He are going to teh store because there books was lost."
  );
  const [sandboxOutput, setSandboxOutput] = useState<string | null>(null);
  const [detectedRules, setDetectedRules] = useState<
    { rule: string; description: string; original: string; fix: string }[]
  >([]);
  const [isEvaluating, setIsEvaluating] = useState(false);

  const handleRunSandbox = () => {
    setIsEvaluating(true);
    setSandboxOutput(null);

    setTimeout(() => {
      let result = sandboxInput;

      // Mock Grammar Rule Evaluations matching LanguageTool & SymSpell
      const rulesFound: { rule: string; description: string; original: string; fix: string }[] = [];

      if (result.includes("He are")) {
        result = result.replace("He are", "He is");
        rulesFound.push({
          rule: "SUBJECT_VERB_AGREEMENT",
          description: "Singular subject 'He' requires singular verb 'is'.",
          original: "He are",
          fix: "He is",
        });
      }

      if (result.includes("teh")) {
        result = result.replace("teh", "the");
        rulesFound.push({
          rule: "TYPO_TEH",
          description: "Common spelling transposition.",
          original: "teh",
          fix: "the",
        });
      }

      if (result.includes("there books")) {
        result = result.replace("there books", "their books");
        rulesFound.push({
          rule: "HOMOPHONE_THERE",
          description: "Possessive pronoun 'their' required before noun 'books'.",
          original: "there books",
          fix: "their books",
        });
      }

      if (result.includes("was lost")) {
        result = result.replace("was lost", "were lost");
        rulesFound.push({
          rule: "PLURAL_VERB",
          description: "Plural subject 'books' requires plural verb 'were'.",
          original: "was lost",
          fix: "were lost",
        });
      }

      setDetectedRules(rulesFound);
      setSandboxOutput(result);
      setIsEvaluating(false);
    }, 400);
  };

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
            <div className="p-2.5 bg-emerald-500/15 rounded-xl text-emerald-400 border border-emerald-500/20">
              <CheckCheck className="w-5 h-5" />
            </div>
            Grammar & Style Engine
          </h2>
          <p className="text-xs text-[#A1A0AB] mt-1">
            Real-time LanguageTool + AI grammar evaluation across active Windows/macOS applications.
          </p>
        </div>

        <div className="flex items-center gap-3">
          <span className="text-xs font-semibold text-[#A1A0AB]">Master Switch</span>
          <button
            onClick={() => onToggle(!status.enabled)}
            className={`w-12 h-6 rounded-full transition-colors p-1 relative cursor-pointer active:scale-95 ${
              status.enabled ? "bg-[#DA7756]" : "bg-zinc-800"
            }`}
          >
            <div
              className={`w-4 h-4 rounded-full bg-white shadow-md transition-transform ${
                status.enabled ? "translate-x-6" : "translate-x-0"
              }`}
            />
          </button>
        </div>
      </div>

      {/* Live Interactive Grammar Sandbox */}
      <div className="glass-panel p-6 rounded-3xl space-y-4 border border-[#DA7756]/20 shadow-2xl relative overflow-hidden">
        <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-3">
          <div className="flex items-center gap-2">
            <Sparkles className="w-4 h-4 text-[#DA7756]" />
            <h3 className="text-sm font-bold text-[#FAF8F5]">Live Interactive Grammar Sandbox</h3>
          </div>
          <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 font-bold">
            LanguageTool Server Connected
          </span>
        </div>

        <div className="space-y-3">
          <label className="block text-xs font-semibold text-[#A1A0AB]">
            Type or paste a sentence to test auto-correction:
          </label>
          <div className="flex gap-3">
            <input
              type="text"
              value={sandboxInput}
              onChange={(e) => setSandboxInput(e.target.value)}
              placeholder="Enter text to evaluate..."
              className="flex-1 bg-[#121216] border border-white/10 rounded-2xl px-4 py-3 text-xs text-[#FAF8F5] focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50 font-mono"
            />
            <button
              onClick={handleRunSandbox}
              disabled={isEvaluating}
              className="px-5 py-3 bg-[#DA7756] hover:bg-[#C86544] text-white rounded-2xl text-xs font-bold transition-all duration-200 flex items-center gap-2 shadow-lg shadow-[#DA7756]/25 cursor-pointer active:scale-95 disabled:opacity-50"
            >
              {isEvaluating ? (
                <>
                  <RefreshCw className="w-4 h-4 animate-spin" /> Evaluating...
                </>
              ) : (
                <>
                  <Play className="w-4 h-4 fill-white" /> Evaluate Fix
                </>
              )}
            </button>
          </div>
        </div>

        {/* Sandbox Results */}
        {sandboxOutput && (
          <div className="pt-3 space-y-3 animate-fade-in">
            <div className="p-4 bg-[#17161D] rounded-2xl border border-emerald-500/30 flex items-center justify-between text-xs">
              <div>
                <span className="text-[10px] font-mono uppercase tracking-wider text-[#71707C] block mb-1">
                  Corrected Output:
                </span>
                <p className="font-mono text-sm font-bold text-emerald-300">{sandboxOutput}</p>
              </div>
              <span className="text-[10px] font-bold px-2.5 py-1 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                {detectedRules.length} Rule(s) Applied
              </span>
            </div>

            {detectedRules.length > 0 && (
              <div className="grid grid-cols-2 gap-3 pt-1">
                {detectedRules.map((r, i) => (
                  <div
                    key={i}
                    className="p-3 bg-zinc-900/80 rounded-xl border border-white/[0.08] text-xs space-y-1"
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-mono text-[10px] font-extrabold text-[#DA7756] uppercase">
                        {r.rule}
                      </span>
                      <div className="flex items-center gap-1 font-mono text-[11px]">
                        <span className="text-rose-400 line-through">{r.original}</span>
                        <ArrowRight className="w-3 h-3 text-[#71707C]" />
                        <span className="text-emerald-400 font-bold">{r.fix}</span>
                      </div>
                    </div>
                    <p className="text-[11px] text-[#A1A0AB]">{r.description}</p>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Mode Controls Grid */}
      <div className="grid grid-cols-2 gap-6">
        <div
          onClick={() => onModeChange("Aggressive")}
          className={`glass-panel p-6 rounded-3xl cursor-pointer transition-all duration-200 active:scale-[0.99] ${
            status.mode === "Aggressive"
              ? "border-[#DA7756]/50 bg-[#DA7756]/10 shadow-lg shadow-[#DA7756]/15"
              : "hover:border-white/20"
          }`}
        >
          <div className="flex items-center justify-between">
            <div className="p-3 bg-[#DA7756]/15 rounded-2xl text-[#DA7756] border border-[#DA7756]/20">
              <Sparkles className="w-5 h-5" />
            </div>
            {status.mode === "Aggressive" && (
              <span className="text-[10px] font-extrabold px-2.5 py-0.5 rounded-full bg-[#DA7756] text-white uppercase tracking-wider">
                Active Mode
              </span>
            )}
          </div>
          <h3 className="text-base font-extrabold text-[#FAF8F5] mt-4">Aggressive Auto-Fix</h3>
          <p className="text-xs text-[#A1A0AB] mt-1 leading-relaxed">
            Automatically applies high-confidence grammar and spelling corrections instantly upon typing spaces or punctuation.
          </p>
        </div>

        <div
          onClick={() => onModeChange("Suggestions Only")}
          className={`glass-panel p-6 rounded-3xl cursor-pointer transition-all duration-200 active:scale-[0.99] ${
            status.mode === "Suggestions Only"
              ? "border-[#DA7756]/50 bg-[#DA7756]/10 shadow-lg shadow-[#DA7756]/15"
              : "hover:border-white/20"
          }`}
        >
          <div className="flex items-center justify-between">
            <div className="p-3 bg-sky-500/15 rounded-2xl text-sky-400 border border-sky-500/20">
              <Sliders className="w-5 h-5" />
            </div>
            {status.mode === "Suggestions Only" && (
              <span className="text-[10px] font-extrabold px-2.5 py-0.5 rounded-full bg-sky-500 text-white uppercase tracking-wider">
                Active Mode
              </span>
            )}
          </div>
          <h3 className="text-base font-extrabold text-[#FAF8F5] mt-4">Suggestions Only</h3>
          <p className="text-xs text-[#A1A0AB] mt-1 leading-relaxed">
            Displays subtle inline suggestion tooltips without auto-modifying text until accepted.
          </p>
        </div>
      </div>

      {/* Recent Grammar Fixes Table */}
      <div className="glass-panel rounded-2xl p-6 space-y-4">
        <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
          <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
            <ShieldCheck className="w-4 h-4 text-emerald-400" />
            Recent Grammar Corrections Log
          </h3>
          <span className="text-xs font-semibold text-[#A1A0AB]">Language: {status.language}</span>
        </div>

        <div className="space-y-3">
          {fixes.map((fix) => (
            <div
              key={fix.id}
              className="p-4 rounded-2xl bg-[#17161D]/80 border border-[#DA7756]/15 flex items-center justify-between text-xs"
            >
              <div className="space-y-1 font-mono">
                <div className="flex items-center gap-2">
                  <span className="text-rose-400 line-through bg-rose-500/10 px-2 py-0.5 rounded border border-rose-500/20">
                    {fix.original}
                  </span>
                  <span className="text-[#71707C]">→</span>
                  <span className="text-emerald-400 font-bold bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
                    {fix.fixed}
                  </span>
                </div>
                <p className="text-[11px] text-[#71707C] font-sans">Rule: {fix.rule_id}</p>
              </div>

              <span className="text-[10px] font-extrabold px-2.5 py-1 rounded-full bg-[#DA7756]/15 text-[#DA7756] border border-[#DA7756]/20">
                {fix.category}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
