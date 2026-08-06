import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Sparkles, CornerDownLeft, Copy, RotateCcw } from "lucide-react";
import { PaletteSkeleton } from "./Skeleton";
import { ErrorState } from "./ErrorState";

interface CopilotPaletteProps {
  selectedText?: string;
  targetApp?: string;
  onExecutePrompt?: (prompt: string) => void;
  onInsert: (text: string) => void;
  onClose: () => void;
}

export const CopilotPalette: React.FC<CopilotPaletteProps> = ({
  selectedText = "",
  targetApp = "",
  onExecutePrompt,
  onInsert,
  onClose,
}) => {
  const [promptInput, setPromptInput] = useState("");
  const [history, setHistory] = useState<string[]>([]);
  const [historyIdx, setHistoryIdx] = useState<number>(-1);
  const [statusState, setStatusState] = useState<"idle" | "loading" | "result" | "error">("idle");
  const [resultText, setResultText] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const hasTextField = Boolean(targetApp);

  useEffect(() => {
    textareaRef.current?.focus();
  }, [statusState]);

  // Handle keyboard events (History navigation, Escape, Shift+Enter)
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Escape") {
      e.preventDefault();
      if (statusState === "loading") {
        setStatusState("idle");
      } else {
        onClose();
      }
    } else if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      executePrompt();
    } else if (e.key === "ArrowUp") {
      if (history.length > 0 && historyIdx < history.length - 1) {
        e.preventDefault();
        const newIdx = historyIdx + 1;
        setHistoryIdx(newIdx);
        setPromptInput(history[history.length - 1 - newIdx]);
      }
    } else if (e.key === "ArrowDown") {
      if (historyIdx > 0) {
        e.preventDefault();
        const newIdx = historyIdx - 1;
        setHistoryIdx(newIdx);
        setPromptInput(history[history.length - 1 - newIdx]);
      } else if (historyIdx === 0) {
        e.preventDefault();
        setHistoryIdx(-1);
        setPromptInput("");
      }
    }
  };

  const executePrompt = async () => {
    const trimmed = promptInput.trim();
    if (!trimmed) return;

    if (!history.includes(trimmed)) {
      setHistory((prev) => [...prev, trimmed]);
    }
    setHistoryIdx(-1);

    setStatusState("loading");
    if (onExecutePrompt) onExecutePrompt(trimmed);

    try {
      const res = await invoke<string>("run_copilot_prompt", {
        prompt: trimmed,
        contextBefore: selectedText || "",
        contextAfter: "",
      });
      setResultText(res);
      setStatusState("result");
    } catch (err) {
      setErrorMessage(String(err));
      setStatusState("error");
    }
  };

  return (
    <div className="fixed inset-0 bg-black/40 backdrop-blur-sm flex items-start justify-center pt-[28vh] z-50 select-none animate-fade-in font-sans">
      <div className="w-[580px] bg-[#161618] border border-[rgba(255,255,255,0.12)] rounded-[14px] shadow-2xl overflow-hidden text-[#EDEDED] animate-pop-in">
        {/* Context Bar */}
        <div className="bg-[#1F1F23] border-b border-[rgba(255,255,255,0.08)] px-4 py-2 flex items-center justify-between">
          <div className="flex items-center gap-2 min-w-0 flex-1">
            <span className="font-mono text-[10px] font-semibold text-[#6366F1] uppercase tracking-wider px-1.5 py-0.5 rounded bg-[rgba(99,102,241,0.12)] border border-[rgba(99,102,241,0.2)]">
              CONTEXT
            </span>
            <p className="font-sans text-[12px] text-[#8F8F96] truncate">
              {selectedText ? (
                <span className="font-medium text-[#EDEDED]">"{selectedText}"</span>
              ) : (
                <span className="italic text-[#5C5C62]">
                  {hasTextField ? `Focused in ${targetApp}` : "Global context — results copied to clipboard"}
                </span>
              )}
            </p>
          </div>
        </div>

        {/* Dynamic Body */}
        {statusState === "idle" && (
          <div className="p-4 space-y-3">
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 rounded-[6px] bg-[#6366F1] flex items-center justify-center text-white shrink-0 mt-1 shadow-sm">
                <Sparkles className="w-3.5 h-3.5" />
              </div>

              <textarea
                ref={textareaRef}
                rows={1}
                value={promptInput}
                onChange={(e) => setPromptInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Ask anything — rewrite, summarize, translate, explain…"
                className="flex-1 bg-transparent text-[14px] text-[#EDEDED] placeholder-[#5C5C62] resize-none focus:outline-none min-h-[36px] max-h-[140px] leading-relaxed"
              />

              <div className="flex items-center gap-1.5 shrink-0 self-center">
                <span className="px-2 py-0.5 bg-[#28282D] border border-[rgba(255,255,255,0.08)] rounded-[5px] text-[10px] font-mono text-[#8F8F96]">
                  ↵ enter
                </span>
              </div>
            </div>

            {/* Friendly Quick Action Chips */}
            <div className="flex items-center gap-2 pt-1 border-t border-[rgba(255,255,255,0.06)]">
              {[
                { label: "Rewrite Clearly", prompt: "Rewrite clearly to improve flow and readability" },
                { label: "Fix Spelling & Grammar", prompt: "Fix all spelling and grammar errors" },
                { label: "Make Formal", prompt: "Rewrite in a professional, formal tone" },
                { label: "Summarize", prompt: "Summarize into bullet points" },
              ].map((chip, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setPromptInput(chip.prompt);
                    executePrompt();
                  }}
                  className="px-2.5 py-1 bg-[#1F1F23] hover:bg-[#28282D] border border-[rgba(255,255,255,0.08)] text-[#8F8F96] hover:text-[#EDEDED] rounded-[6px] text-[12px] font-medium transition cursor-pointer"
                >
                  {chip.label}
                </button>
              ))}
            </div>
          </div>
        )}

        {statusState === "loading" && (
          <div className="p-4 space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 rounded-full border-2 border-[#6366F1] border-t-transparent animate-spin" />
                <span className="text-[13px] font-medium text-[#EDEDED]">
                  "{promptInput}"
                </span>
              </div>
              <span className="font-mono text-[11px] text-[#8F8F96]">
                groq · llama-3.3-70b
              </span>
            </div>
            <PaletteSkeleton />
          </div>
        )}

        {statusState === "result" && (
          <div className="p-4 space-y-3">
            <div className="max-h-[220px] overflow-y-auto pr-1">
              <p className="text-[14px] text-[#EDEDED] leading-relaxed font-sans select-text whitespace-pre-wrap">
                {resultText}
              </p>
            </div>

            <div className="flex items-center gap-2 pt-2 border-t border-[rgba(255,255,255,0.08)]">
              {hasTextField && (
                <button
                  onClick={() => {
                    onInsert(resultText);
                    onClose();
                  }}
                  className="px-3.5 py-1.5 bg-[#6366F1] hover:bg-[#4F46E5] text-white text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer shadow-sm"
                >
                  <CornerDownLeft className="w-3.5 h-3.5" /> Insert into {targetApp}
                </button>
              )}

              <button
                onClick={() => {
                  navigator.clipboard.writeText(resultText);
                  onClose();
                }}
                className="px-3.5 py-1.5 bg-[#28282D] hover:bg-[#333338] text-[#EDEDED] border border-[rgba(255,255,255,0.08)] text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer"
              >
                <Copy className="w-3.5 h-3.5" /> Copy
              </button>

              <button
                onClick={executePrompt}
                className="px-3.5 py-1.5 bg-[#28282D] hover:bg-[#333338] text-[#EDEDED] border border-[rgba(255,255,255,0.08)] text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer"
              >
                <RotateCcw className="w-3.5 h-3.5" /> Retry
              </button>
            </div>
          </div>
        )}

        {statusState === "error" && (
          <div className="p-4">
            <ErrorState
              compact
              title="Copilot Error"
              message={errorMessage || "An error occurred while processing prompt."}
              onRetry={executePrompt}
            />
          </div>
        )}

        {/* Footer Bar */}
        <div className="border-t border-[rgba(255,255,255,0.08)] px-4 py-2 flex items-center justify-between text-[11px] text-[#8F8F96] bg-[#1F1F23]">
          <div className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-[#22C55E]" />
            <span>↑↓ history · Shift+↵ newline</span>
          </div>

          <span className="font-mono text-[#5C5C62]">esc to dismiss</span>
        </div>
      </div>
    </div>
  );
};
