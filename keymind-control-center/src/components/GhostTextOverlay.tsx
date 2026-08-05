import React, { useState, useEffect } from "react";

interface GhostTextOverlayProps {
  suggestions?: string[];
  confidence?: number;
  onAcceptFull: (text: string) => void;
  onAcceptWord: (word: string) => void;
  onDismiss: () => void;
}

export const GhostTextOverlay: React.FC<GhostTextOverlayProps> = ({
  suggestions = ["future", "me know", "financial results"],
  confidence = 0.85,
  onAcceptFull,
  onAcceptWord,
  onDismiss,
}) => {
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Tab") {
        e.preventDefault();
        if (suggestions[0]) {
          onAcceptFull(suggestions[0]);
          setVisible(false);
        }
      } else if (e.ctrlKey && e.key === "ArrowRight") {
        e.preventDefault();
        if (suggestions[0]) {
          const firstWord = suggestions[0].split(" ")[0];
          onAcceptWord(firstWord);
        }
      } else if (e.key === "Escape") {
        setVisible(false);
        onDismiss();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [suggestions, onAcceptFull, onAcceptWord, onDismiss]);

  if (!visible || !suggestions || suggestions.length === 0) {
    return null;
  }

  const primarySuggestion = suggestions[0];

  return (
    <div className="fixed top-12 left-12 z-50 font-['Inter',sans-serif] pointer-events-none select-none">
      <div className="bg-[#141414]/90 backdrop-blur-md border border-[#2A2A2A] rounded-lg px-2.5 py-1 shadow-xl flex items-center gap-2">
        <span className="text-[14px] italic font-medium text-[#888888] tracking-wide">
          {primarySuggestion}
        </span>
        <span className="text-[9px] font-bold text-[#8B5CF6] bg-[#8B5CF6]/20 px-1 py-0.2 rounded border border-[#8B5CF6]/30">
          Tab ⇆
        </span>
      </div>
    </div>
  );
};
