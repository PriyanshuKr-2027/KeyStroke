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
  const [statusState, setStatusState] = useState<"idle" | "loading" | "result" | "error">("idle");
  const [resultText, setResultText] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const hasTextField = Boolean(targetApp);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (statusState === "loading") {
          setStatusState("idle");
        } else {
          onClose();
        }
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [statusState, onClose]);

  const executePrompt = async () => {
    if (!promptInput.trim()) return;

    setStatusState("loading");
    if (onExecutePrompt) onExecutePrompt(promptInput);

    try {
      const res = await invoke<string>("run_copilot_prompt", {
        prompt: promptInput,
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

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    executePrompt();
  };

  return (
    <div className="fixed inset-0 bg-black/10 flex items-start justify-center pt-[38vh] z-50 select-none animate-fade-in font-sans">
      <div className="w-[560px] bg-[#FFFFFF] border border-[#EBEBEB] rounded-[14px] shadow-2xl overflow-hidden text-[#111111]">
        {/* Context Strip */}
        <div className="bg-[#F5F5F5] border-b border-[#EBEBEB] px-3.5 py-2">
          <div className="flex items-center gap-2">
            <span className="font-mono text-[11px] font-semibold text-[#6B6B6B] uppercase tracking-wider">
              context
            </span>
            <p className="font-sans text-[12px] text-[#6B6B6B] truncate flex-1">
              {hasTextField ? (
                <span className="font-medium text-[#111111]">"{selectedText}"</span>
              ) : (
                <span className="italic text-[#AAAAAA]">
                  No text field focused — result will be copied to clipboard.
                </span>
              )}
            </p>
          </div>
        </div>

        {/* Dynamic Body State */}
        {statusState === "idle" && (
          <form onSubmit={handleSubmit} className="px-3.5 py-3 flex items-center gap-3">
            <div className="w-5 h-5 rounded-[5px] bg-[#111111] flex items-center justify-center text-[#FFFFFF]">
              <Sparkles className="w-3 h-3" />
            </div>

            <input
              ref={inputRef}
              type="text"
              value={promptInput}
              onChange={(e) => setPromptInput(e.target.value)}
              placeholder="Ask anything — rewrite, explain, continue, translate…"
              className="flex-1 bg-transparent text-[14px] text-[#111111] placeholder-[#AAAAAA] focus:outline-none"
            />

            <div className="px-1.5 py-0.5 bg-[#F5F5F5] border border-[#EBEBEB] rounded-[5px] text-[11px] font-mono text-[#6B6B6B]">
              ↵ enter
            </div>
          </form>
        )}

        {statusState === "loading" && (
          <div className="p-3.5 space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-3.5 h-3.5 rounded-full border-2 border-[#111111] border-t-transparent animate-spin" />
                <span className="text-[13px] font-medium text-[#111111]">
                  "{promptInput}"
                </span>
              </div>
              <span className="font-mono text-[11px] text-[#AAAAAA]">
                groq · llama-3.3-70b
              </span>
            </div>
            <PaletteSkeleton />
          </div>
        )}

        {statusState === "result" && (
          <div className="p-3.5 space-y-3">
            <p className="text-[14px] text-[#111111] leading-relaxed font-sans">
              {resultText}
            </p>

            <div className="flex items-center gap-2 pt-1">
              {hasTextField && (
                <button
                  onClick={() => {
                    onInsert(resultText);
                    onClose();
                  }}
                  className="px-3 py-1.5 bg-[#111111] hover:bg-[#333333] text-[#FFFFFF] text-[13px] font-medium rounded-[6px] transition flex items-center gap-1.5 cursor-pointer"
                >
                  <CornerDownLeft className="w-3.5 h-3.5" /> Insert
                </button>
              )}

              <button
                onClick={() => {
                  navigator.clipboard.writeText(resultText);
                  onClose();
                }}
                className="px-3 py-1.5 bg-[#F0F0F0] hover:bg-[#E5E5E5] text-[#111111] text-[13px] font-medium rounded-[6px] transition flex items-center gap-1.5 cursor-pointer"
              >
                <Copy className="w-3.5 h-3.5" /> Copy
              </button>

              <button
                onClick={executePrompt}
                className="px-3 py-1.5 bg-[#F0F0F0] hover:bg-[#E5E5E5] text-[#111111] text-[13px] font-medium rounded-[6px] transition flex items-center gap-1.5 cursor-pointer"
              >
                <RotateCcw className="w-3.5 h-3.5" /> Retry
              </button>
            </div>
          </div>
        )}

        {statusState === "error" && (
          <div className="p-3.5">
            <ErrorState
              compact
              title="Copilot Error"
              message={errorMessage || "An error occurred while processing prompt."}
              onRetry={executePrompt}
            />
          </div>
        )}

        {/* Footer Bar */}
        <div className="border-t border-[#EBEBEB] px-3.5 py-1.5 flex items-center justify-between text-[11px] text-[#6B6B6B]">
          <div className="flex items-center gap-1.5">
            <span
              className={`w-1.5 h-1.5 rounded-full ${
                hasTextField ? "bg-[#22C55E]" : "bg-[#D1D5DB]"
              }`}
            />
            <span>
              {hasTextField
                ? `Result types into ${targetApp}`
                : "Result copied to clipboard"}
            </span>
          </div>

          <span className="font-mono text-[#AAAAAA]">esc to close</span>
        </div>
      </div>
    </div>
  );
};
