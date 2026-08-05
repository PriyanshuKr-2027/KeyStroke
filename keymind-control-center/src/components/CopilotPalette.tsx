import React, { useState, useEffect, useRef } from "react";
import { CopilotAction } from "../types";
import { Sparkles, X, Loader2, Check, ArrowRight, CornerDownLeft } from "lucide-react";

const ACTIONS: CopilotAction[] = [
  { id: "rewrite", icon: "🔁", name: "Rewrite", description: "Rephrase clearly and fluidly" },
  { id: "grammar", icon: "✅", name: "Fix Grammar", description: "Correct spelling and grammar" },
  { id: "translate", icon: "🌐", name: "Translate", description: "Translate into fluent English" },
  { id: "professional", icon: "💼", name: "Professional", description: "Polished, business tone" },
  { id: "friendly", icon: "😊", name: "Friendly", description: "Warm and engaging tone" },
  { id: "summarize", icon: "📋", name: "Summarize", description: "Bullet-point key summary" },
  { id: "expand", icon: "📖", name: "Expand", description: "Elaborate with details" },
  { id: "concise", icon: "✂️", name: "Make Concise", description: "Punchy, direct, no fluff" },
  { id: "continue", icon: "✍️", name: "Continue Writing", description: "Write next paragraphs" },
  { id: "explain", icon: "💡", name: "Explain", description: "Explain concepts simply" },
];

interface CopilotPaletteProps {
  selectedText?: string;
  onExecuteAction: (actionId: string, text: string) => void;
  onAccept: (finalText: string) => void;
  onClose: () => void;
  isStreaming?: boolean;
  streamedText?: string;
  isComplete?: boolean;
}

export const CopilotPalette: React.FC<CopilotPaletteProps> = ({
  selectedText = "The project roadmap needs to be updated before the upcoming team meeting.",
  onExecuteAction,
  onAccept,
  onClose,
  isStreaming = false,
  streamedText = "",
  isComplete = false,
}) => {
  const [search, setSearch] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [activeAction, setActiveAction] = useState<CopilotAction | null>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);

  const filteredActions = ACTIONS.filter(
    (a) =>
      a.name.toLowerCase().includes(search.toLowerCase()) ||
      a.description.toLowerCase().includes(search.toLowerCase())
  );

  // Focus search input on mount
  useEffect(() => {
    searchInputRef.current?.focus();
  }, []);

  // Reset selected index when search changes
  useEffect(() => {
    setSelectedIndex(0);
  }, [search]);

  // Keyboard navigation listener
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (activeAction) {
          // Cancel active execution
          setActiveAction(null);
        } else {
          onClose();
        }
        return;
      }

      if (!activeAction) {
        if (e.key === "ArrowDown") {
          e.preventDefault();
          setSelectedIndex((prev) =>
            filteredActions.length ? (prev + 1) % filteredActions.length : 0
          );
        } else if (e.key === "ArrowUp") {
          e.preventDefault();
          setSelectedIndex((prev) =>
            filteredActions.length
              ? (prev - 1 + filteredActions.length) % filteredActions.length
              : 0
          );
        } else if (e.key === "Enter") {
          e.preventDefault();
          if (filteredActions[selectedIndex]) {
            const action = filteredActions[selectedIndex];
            setActiveAction(action);
            onExecuteAction(action.id, selectedText);
          }
        }
      } else if (isComplete && e.key === "Enter") {
        e.preventDefault();
        onAccept(streamedText);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [filteredActions, selectedIndex, activeAction, isComplete, streamedText, selectedText, onExecuteAction, onAccept, onClose]);

  const truncatedPreview =
    selectedText.length > 80 ? `${selectedText.substring(0, 80)}...` : selectedText;

  return (
    <div className="fixed inset-0 flex items-center justify-center p-4 bg-transparent z-50 select-none font-['Inter',sans-serif]">
      <div className="w-[420px] h-[520px] bg-[#141414]/94 backdrop-blur-2xl border border-[#2A2A2A] rounded-2xl shadow-2xl flex flex-col justify-between overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#2A2A2A]/80 bg-[#1A1A1A]/40">
          <div className="flex items-center gap-2">
            <Sparkles className="w-4 h-4 text-[#8B5CF6] animate-pulse" />
            <span className="text-xs font-bold text-white tracking-wide">AI Copilot</span>
          </div>
          <button
            onClick={onClose}
            className="text-[11px] font-medium text-[#888888] hover:text-white flex items-center gap-1 bg-[#222222] px-2 py-0.5 rounded-md border border-[#333333]"
          >
            Esc closes
          </button>
        </div>

        {/* Selected Text Preview Bar */}
        <div className="px-4 py-2.5 bg-[#0F0F0F]/80 border-b border-[#2A2A2A]/60 text-xs text-[#888888] italic truncate">
          "{truncatedPreview}"
        </div>

        {/* Middle Content Body */}
        <div className="flex-1 overflow-y-auto p-2 space-y-1">
          {!activeAction ? (
            /* Action List */
            filteredActions.map((action, idx) => {
              const isSelected = idx === selectedIndex;
              return (
                <div
                  key={action.id}
                  onClick={() => {
                    setSelectedIndex(idx);
                    setActiveAction(action);
                    onExecuteAction(action.id, selectedText);
                  }}
                  className={`flex items-center justify-between px-3 py-2.5 rounded-xl cursor-pointer transition-all duration-150 ${
                    isSelected
                      ? "bg-[#8B5CF6]/20 border-l-2 border-[#8B5CF6] text-white"
                      : "text-[#888888] hover:text-white hover:bg-[#1A1A1A]"
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <span className="text-base">{action.icon}</span>
                    <div>
                      <h4 className="text-xs font-semibold text-white">{action.name}</h4>
                      <p className="text-[11px] text-[#888888]">{action.description}</p>
                    </div>
                  </div>
                  {isSelected && (
                    <CornerDownLeft className="w-3.5 h-3.5 text-[#8B5CF6]" />
                  )}
                </div>
              );
            })
          ) : (
            /* Streaming Response Body */
            <div className="p-3 space-y-3 h-full flex flex-col justify-between">
              <div className="flex items-center gap-2 text-xs font-semibold text-[#8B5CF6]">
                {isStreaming ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin text-[#8B5CF6]" />
                    <span>Thinking with Groq...</span>
                  </>
                ) : (
                  <>
                    <Check className="w-4 h-4 text-emerald-400" />
                    <span className="text-emerald-400">Response Generated</span>
                  </>
                )}
              </div>

              {/* Streaming Output Preview Panel */}
              <div className="flex-1 bg-[#0F0F0F] border border-[#2A2A2A] rounded-xl p-3 text-xs font-mono text-white overflow-y-auto whitespace-pre-wrap leading-relaxed">
                {streamedText || (isStreaming ? "Generating response..." : "")}
              </div>
            </div>
          )}
        </div>

        {/* Footer Bar */}
        {!activeAction ? (
          /* Search Bar at Bottom */
          <div className="p-3 border-t border-[#2A2A2A]/80 bg-[#1A1A1A]/40">
            <input
              ref={searchInputRef}
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="🔍 Search actions..."
              className="w-full bg-[#0F0F0F] border border-[#2A2A2A] rounded-xl px-3.5 py-2 text-xs text-white placeholder-[#888888] focus:outline-none focus:border-[#8B5CF6]"
            />
          </div>
        ) : (
          /* Accept / Cancel Action Bar */
          <div className="p-3 border-t border-[#2A2A2A]/80 bg-[#1A1A1A]/40 flex items-center justify-between">
            <button
              onClick={() => setActiveAction(null)}
              className="px-3 py-1.5 rounded-lg border border-[#333333] bg-[#222222] text-xs font-medium text-[#888888] hover:text-white"
            >
              Cancel (⎋)
            </button>

            <button
              disabled={!isComplete}
              onClick={() => onAccept(streamedText)}
              className="px-4 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] disabled:opacity-50 text-xs font-semibold text-white shadow-lg shadow-[#8B5CF6]/20 flex items-center gap-1.5"
            >
              Accept (↵)
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
