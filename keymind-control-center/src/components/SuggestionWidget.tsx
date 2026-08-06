import React from "react";
import { ActivePrediction } from "../types";
import { X, Sparkles } from "lucide-react";

interface SuggestionWidgetProps {
  prediction: ActivePrediction | null;
  onAccept: (word: string) => void;
  onDismiss: () => void;
}

export const SuggestionWidget: React.FC<SuggestionWidgetProps> = ({
  prediction,
  onAccept,
  onDismiss,
}) => {
  if (!prediction || !prediction.candidate_word) return null;

  return (
    <div className="fixed bottom-6 right-6 z-50 flex items-center gap-3 px-3.5 py-2 bg-[#161618] border border-[rgba(255,255,255,0.12)] rounded-[10px] shadow-2xl animate-slide-up text-[13px] font-sans select-none text-[#EDEDED]">
      <div className="flex items-center gap-2">
        <Sparkles className="w-3.5 h-3.5 text-[#6366F1]" />
        <span className="text-[#8F8F96]">Suggestion:</span>
        <span className="font-mono font-medium text-[#EDEDED] bg-[#28282D] px-2 py-0.5 rounded border border-[rgba(255,255,255,0.08)]">
          {prediction.candidate_word}
        </span>
      </div>

      <div className="flex items-center gap-2 border-l border-[rgba(255,255,255,0.08)] pl-2.5">
        <button
          onClick={() => onAccept(prediction.candidate_word)}
          className="flex items-center gap-1.5 px-2.5 py-1 bg-[#6366F1] hover:bg-[#4F46E5] text-white rounded-[6px] text-[12px] font-medium transition cursor-pointer shadow-sm"
          title="Press Tab to accept suggestion"
        >
          <kbd className="px-1 py-0.2 bg-[rgba(255,255,255,0.2)] rounded text-[10px] font-mono">
            Tab ↵
          </kbd>
          <span>Accept</span>
        </button>

        <button
          onClick={onDismiss}
          className="p-1 text-[#8F8F96] hover:text-[#EDEDED] transition cursor-pointer"
          title="Dismiss suggestion"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
};
