import React, { useState, useEffect } from "react";
import { GrammarIssueInline } from "../types";
import { CheckCheck, AlertCircle, X, Sparkles, Check } from "lucide-react";

interface GrammarOverlayProps {
  issues?: GrammarIssueInline[];
  onFixInline: (offset: number, length: number, replacement: string) => void;
  onDismiss: () => void;
}

export const GrammarOverlay: React.FC<GrammarOverlayProps> = ({
  issues = [
    {
      offset: 3,
      length: 3,
      message: "Did you mean 'is'?",
      replacements: ["is"],
      rule_id: "HE_ARE",
      category: "GRAMMAR",
    },
    {
      offset: 16,
      length: 3,
      message: "Did you mean 'the'?",
      replacements: ["the"],
      rule_id: "TEH_TYPO",
      category: "TYPOS",
    },
  ],
  onFixInline,
  onDismiss,
}) => {
  const [activeHoverIndex, setActiveHoverIndex] = useState<number | null>(null);
  const [visible, setVisible] = useState(true);

  // Auto-dismiss on keydown typing
  useEffect(() => {
    const handleTyping = () => {
      setVisible(false);
      onDismiss();
    };

    window.addEventListener("keydown", handleTyping);
    return () => window.removeEventListener("keydown", handleTyping);
  }, [onDismiss]);

  if (!visible || !issues || issues.length === 0) {
    return null;
  }

  const categoryColor = (cat: string) => {
    switch (cat.toUpperCase()) {
      case "TYPOS":
        return "bg-rose-500 text-rose-400 border-rose-500/30";
      case "GRAMMAR":
        return "bg-amber-500 text-amber-400 border-amber-500/30";
      default:
        return "bg-slate-400 text-slate-300 border-slate-400/30";
    }
  };

  const categoryUnderlineClass = (cat: string) => {
    switch (cat.toUpperCase()) {
      case "TYPOS":
        return "border-b-2 border-dashed border-rose-500 decoration-rose-500";
      case "GRAMMAR":
        return "border-b-2 border-dashed border-amber-500 decoration-amber-500";
      default:
        return "border-b-2 border-dashed border-slate-400 decoration-slate-400";
    }
  };

  return (
    <div className="fixed bottom-6 right-6 z-50 font-['Inter',sans-serif] select-none">
      <div className="bg-[#141414]/95 backdrop-blur-xl border border-[#2A2A2A] rounded-2xl p-4 shadow-2xl w-80 space-y-3">
        {/* Floating Badge Header */}
        <div className="flex items-center justify-between border-b border-[#2A2A2A] pb-2.5">
          <div className="flex items-center gap-2">
            <CheckCheck className="w-4 h-4 text-[#8B5CF6]" />
            <span className="text-xs font-bold text-white">Grammar Assistant</span>
            <span className="text-[10px] font-bold px-1.5 py-0.5 rounded bg-[#8B5CF6]/20 text-[#8B5CF6]">
              {issues.length} Issues
            </span>
          </div>
          <button
            onClick={() => {
              setVisible(false);
              onDismiss();
            }}
            className="text-[#888888] hover:text-white"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        </div>

        {/* List of Grammar Underline Issues */}
        <div className="space-y-2 max-h-56 overflow-y-auto pr-1">
          {issues.map((iss, idx) => {
            const isHovered = activeHoverIndex === idx;
            return (
              <div
                key={idx}
                onMouseEnter={() => setActiveHoverIndex(idx)}
                onMouseLeave={() => setActiveHoverIndex(null)}
                className={`p-2.5 rounded-xl border transition cursor-pointer ${
                  isHovered
                    ? "bg-[#1A1A1A] border-[#8B5CF6]/40"
                    : "bg-[#0F0F0F] border-[#2A2A2A]"
                }`}
              >
                <div className="flex items-center justify-between mb-1">
                  <span
                    className={`text-[10px] font-bold px-1.5 py-0.2 rounded border ${categoryColor(
                      iss.category
                    )}`}
                  >
                    {iss.category}
                  </span>
                  <span className="text-[10px] font-mono text-[#888888]">
                    Rule: {iss.rule_id}
                  </span>
                </div>

                <p className={`text-xs text-white ${categoryUnderlineClass(iss.category)}`}>
                  {iss.message}
                </p>

                {/* Replacement Options Chips */}
                {iss.replacements && iss.replacements.length > 0 && (
                  <div className="flex flex-wrap items-center gap-1.5 pt-2">
                    <span className="text-[10px] text-[#888888]">Fix:</span>
                    {iss.replacements.map((rep, rIdx) => (
                      <button
                        key={rIdx}
                        onClick={(e) => {
                          e.stopPropagation();
                          onFixInline(iss.offset, iss.length, rep);
                        }}
                        className="px-2 py-0.5 rounded bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 text-xs font-mono font-bold flex items-center gap-1 transition"
                      >
                        <Check className="w-3 h-3" />
                        {rep}
                      </button>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
