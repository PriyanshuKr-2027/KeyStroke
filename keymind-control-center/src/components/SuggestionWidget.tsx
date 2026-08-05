import React from "react";
import { ActivePrediction } from "../types";
import { Sparkles, X } from "lucide-react";

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
    <div className="fixed bottom-8 right-8 z-50 flex items-center gap-3.5 px-4.5 py-3 bg-[#191820]/95 border border-[#DA7756]/35 backdrop-blur-2xl rounded-2xl shadow-2xl shadow-[#DA7756]/20 animate-fade-in text-xs select-none">
      <div className="flex items-center gap-2.5">
        <div className="p-1.5 bg-[#DA7756]/20 rounded-xl text-[#DA7756] border border-[#DA7756]/30">
          <Sparkles className="w-3.5 h-3.5 animate-pulse" />
        </div>
        <span className="text-[#A1A0AB] font-medium">Next word:</span>
        <span className="font-mono font-extrabold text-white text-sm bg-[#DA7756]/15 px-3 py-1 rounded-xl border border-[#DA7756]/30 tracking-wide text-[#FAF8F5]">
          {prediction.candidate_word}
        </span>
      </div>

      <div className="flex items-center gap-2 border-l border-white/10 pl-3">
        <button
          onClick={() => onAccept(prediction.candidate_word)}
          className="flex items-center gap-1.5 px-3 py-1.5 bg-[#DA7756] hover:bg-[#C86544] text-white rounded-xl text-xs font-bold transition-all duration-200 shadow-md shadow-[#DA7756]/30 cursor-pointer"
          title="Press Tab to insert next word"
        >
          <kbd className="px-1.5 py-0.5 bg-black/40 rounded-lg text-[10px] font-mono font-black tracking-wider border border-white/10">
            Tab ↹
          </kbd>
          <span>Accept</span>
        </button>

        <button
          onClick={onDismiss}
          className="p-1.5 text-[#A1A0AB] hover:text-white hover:bg-zinc-800/80 rounded-xl transition"
          title="Dismiss suggestion"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
};
