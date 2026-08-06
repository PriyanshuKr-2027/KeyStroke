import React, { useState, useEffect } from "react";

interface GhostTextOverlayProps {
  suggestions?: string[];
  confidence?: number;
  onAcceptFull: (text: string) => void;
  onAcceptWord: (word: string) => void;
  onDismiss: () => void;
}

export const GhostTextOverlay: React.FC<GhostTextOverlayProps> = ({
  suggestions = ["further assistance", "let me know", "best regards"],
  confidence = 0.88,
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
  const firstWord = primarySuggestion.split(" ")[0];
  const hasMultipleWords = primarySuggestion.trim().includes(" ");

  return (
    <div className="inline-flex items-center gap-2 font-['Inter',sans-serif] pointer-events-none select-none animate-in fade-in zoom-in-95 duration-150">
      {/* Ghost text display */}
      <span className="text-[13px] font-medium text-emerald-400/80 bg-emerald-950/30 px-2 py-0.5 rounded border border-emerald-500/20 shadow-sm flex items-center gap-2">
        <span className="opacity-60">…</span>
        <span className="tracking-wide">{primarySuggestion}</span>

        {/* Shortcut Badges */}
        <div className="flex items-center gap-1.5 ml-1">
          <span className="text-[10px] font-mono font-semibold bg-emerald-500/20 text-emerald-300 px-1.5 py-0.5 rounded border border-emerald-500/30">
            Tab ↵
          </span>
          {hasMultipleWords && (
            <span className="text-[9px] font-mono font-medium bg-neutral-800 text-neutral-400 px-1.5 py-0.5 rounded border border-neutral-700">
              Ctrl+→ <span className="text-neutral-500">({firstWord})</span>
            </span>
          )}
        </div>
      </span>
    </div>
  );
};
