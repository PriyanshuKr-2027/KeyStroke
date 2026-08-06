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
    <div className="fixed inset-0 bg-black/25 backdrop-blur-xs flex items-start justify-center pt-[24vh] z-50 select-none animate-fade-in font-sans">
      <div className="w-[580px] bg-[#FAF8F5] border border-[#E8E4DC] rounded-[14px] shadow-2xl overflow-hidden text-[#1E1E1E] animate-pop-in">
        {/* Context Bar */}
        <div className="bg-[#F3EFEA] border-b border-[#E8E4DC] px-4 py-2 flex items-center justify-between">
          <div className="flex items-center gap-2 min-w-0 flex-1">
            <span className="font-mono text-[10px] font-semibold text-[#DA7756] uppercase tracking-wider px-1.5 py-0.5 rounded bg-[#F7EEEC] border border-[#F0D5CD]">
              CONTEXT
            </span>
            <p className="font-sans text-[12px] text-[#6B6963] truncate">
              {selectedText ? (
                <span className="font-medium text-[#1E1E1E]">"{selectedText}"</span>
              ) : (
                <span className="italic text-[#96938A]">
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
              <div className="w-6 h-6 rounded-[6px] bg-[#DA7756] flex items-center justify-center text-white shrink-0 mt-1 shadow-sm">
                <Sparkles className="w-3.5 h-3.5" />
              </div>

              <textarea
                ref={textareaRef}
                rows={1}
                value={promptInput}
                onChange={(e) => setPromptInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Ask anything — rewrite, summarize, translate, explain…"
                className="flex-1 bg-transparent text-[14px] text-[#1E1E1E] placeholder-[#96938A] resize-none focus:outline-none min-h-[36px] max-h-[140px] leading-relaxed"
              />

              <div className="flex items-center gap-1.5 shrink-0 self-center">
                <span className="px-2 py-0.5 bg-white border border-[#E8E4DC] rounded-[5px] text-[10px] font-mono text-[#6B6963] shadow-xs">
                  ↵ enter
                </span>
              </div>
            </div>

            {/* Quick Action Chips */}
            <div className="flex items-center gap-2 pt-1 border-t border-[#E8E4DC]">
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
                  className="px-2.5 py-1 bg-white hover:bg-[#F3EFEA] border border-[#E8E4DC] text-[#6B6963] hover:text-[#1E1E1E] rounded-[6px] text-[12px] font-medium transition cursor-pointer shadow-xs"
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
                <div className="w-4 h-4 rounded-full border-2 border-[#DA7756] border-t-transparent animate-spin" />
                <span className="text-[13px] font-medium text-[#1E1E1E]">
                  "{promptInput}"
                </span>
              </div>
              <span className="font-mono text-[11px] text-[#6B6963]">
                groq · llama-3.3-70b
              </span>
            </div>
            <PaletteSkeleton />
          </div>
        )}

        {statusState === "result" && (
          <div className="p-4 space-y-3">
            <div className="max-h-[220px] overflow-y-auto pr-1">
              <p className="text-[14px] text-[#1E1E1E] leading-relaxed font-sans select-text whitespace-pre-wrap">
                {resultText}
              </p>
            </div>

            <div className="flex items-center gap-2 pt-2 border-t border-[#E8E4DC]">
              {hasTextField && (
                <button
                  onClick={() => {
                    onInsert(resultText);
                    onClose();
                  }}
                  className="px-3.5 py-1.5 bg-[#DA7756] hover:bg-[#C26243] text-white text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer shadow-sm"
                >
                  <CornerDownLeft className="w-3.5 h-3.5" /> Insert into {targetApp}
                </button>
              )}

              <button
                onClick={() => {
                  navigator.clipboard.writeText(resultText);
                  onClose();
                }}
                className="px-3.5 py-1.5 bg-white hover:bg-[#F3EFEA] text-[#1E1E1E] border border-[#E8E4DC] text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer shadow-xs"
              >
                <Copy className="w-3.5 h-3.5" /> Copy
              </button>

              <button
                onClick={executePrompt}
                className="px-3.5 py-1.5 bg-white hover:bg-[#F3EFEA] text-[#1E1E1E] border border-[#E8E4DC] text-[13px] font-medium rounded-[7px] transition flex items-center gap-1.5 cursor-pointer shadow-xs"
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
        <div className="border-t border-[#E8E4DC] px-4 py-2 flex items-center justify-between text-[11px] text-[#6B6963] bg-[#F3EFEA]">
          <div className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-[#22C55E]" />
            <span>↑↓ history · Shift+↵ newline</span>
          </div>

          <span className="font-mono text-[#96938A]">esc to dismiss</span>
        </div>
      </div>
    </div>
  );
};
