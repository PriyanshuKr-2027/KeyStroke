import React from "react";
import { ActivePrediction } from "../types";
import { X } from "lucide-react";

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
    <div className="fixed bottom-6 right-6 z-50 flex items-center gap-3 px-3.5 py-2 bg-[#FFFFFF] border border-[#EBEBEB] rounded-[10px] shadow-lg animate-fade-in text-[13px] font-sans select-none text-[#111111]">
      <div className="flex items-center gap-2">
        <span className="text-[#6B6B6B]">Next word:</span>
        <span className="font-mono font-medium text-[#111111] bg-[#F5F5F5] px-2 py-0.5 rounded border border-[#EBEBEB]">
          {prediction.candidate_word}
        </span>
      </div>

      <div className="flex items-center gap-2 border-l border-[#EBEBEB] pl-2.5">
        <button
          onClick={() => onAccept(prediction.candidate_word)}
          className="flex items-center gap-1 px-2.5 py-1 bg-[#111111] hover:bg-[#333333] text-[#FFFFFF] rounded-[6px] text-[12px] font-medium transition cursor-pointer"
          title="Press Tab to insert next word"
        >
          <kbd className="px-1 py-0.2 bg-[#333333] rounded text-[10px] font-mono">
            Tab ↹
          </kbd>
          <span>Accept</span>
        </button>

        <button
          onClick={onDismiss}
          className="p-1 text-[#AAAAAA] hover:text-[#111111] transition cursor-pointer"
          title="Dismiss suggestion"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
};
