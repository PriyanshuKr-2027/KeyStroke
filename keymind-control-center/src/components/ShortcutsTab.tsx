import React, { useState, useEffect } from "react";
import { ShortcutBinding } from "../types";
import { Keyboard, RotateCcw, Sparkles, Check, Info } from "lucide-react";

export const ShortcutsTab: React.FC = () => {
  const [shortcuts, setShortcuts] = useState<ShortcutBinding[]>([
    {
      id: "copilot_palette",
      name: "AI Copilot Palette",
      action: "Opens AI Copilot floating palette window",
      shortcut: "Ctrl+Alt+Space",
    },
    {
      id: "grammar_fix",
      name: "Grammar Fix Selection",
      action: "Fixes highlighted text via LanguageTool / AI",
      shortcut: "Ctrl+Alt+G",
    },
    {
      id: "copilot_professional",
      name: "Copilot Professional",
      action: "Rewrites selection in formal business tone",
      shortcut: "Ctrl+Alt+P",
    },
    {
      id: "copilot_summarize",
      name: "Copilot Summarize",
      action: "Summarizes selection into bullet points",
      shortcut: "Ctrl+Alt+S",
    },
    {
      id: "ai_expand",
      name: "Trigger AI Prompt",
      action: "Invokes Groq/Cerebras AI prompt pipeline",
      shortcut: "Ctrl+Alt+X",
    },
    {
      id: "toggle_engine",
      name: "Toggle KeyMind Interceptor",
      action: "Pauses or resumes global keyboard interception",
      shortcut: "Ctrl+Alt+K",
    },
  ]);

  const [capturingId, setCapturingId] = useState<string | null>(null);
  const [lastRecorded, setLastRecorded] = useState<string | null>(null);

  // Live keypress listener during capture
  useEffect(() => {
    if (!capturingId) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();

      if (e.key === "Escape") {
        setCapturingId(null);
        return;
      }

      const parts: string[] = [];
      if (e.ctrlKey) parts.push("Ctrl");
      if (e.altKey) parts.push("Alt");
      if (e.shiftKey) parts.push("Shift");

      let keyName = e.key.toUpperCase();
      if (keyName === " ") keyName = "Space";

      if (!["CONTROL", "ALT", "SHIFT", "META"].includes(keyName)) {
        parts.push(keyName);
        const combo = parts.join("+");

        setShortcuts((prev) =>
          prev.map((s) => (s.id === capturingId ? { ...s, shortcut: combo } : s))
        );
        setLastRecorded(`Updated shortcut to: ${combo}`);
        setCapturingId(null);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [capturingId]);

  const handleRowClick = (id: string) => {
    setCapturingId(id);
    setLastRecorded(null);
  };

  const handleResetDefaults = () => {
    setShortcuts([
      {
        id: "copilot_palette",
        name: "AI Copilot Palette",
        action: "Opens AI Copilot floating palette window",
        shortcut: "Ctrl+Alt+Space",
      },
      {
        id: "grammar_fix",
        name: "Grammar Fix Selection",
        action: "Fixes highlighted text via LanguageTool / AI",
        shortcut: "Ctrl+Alt+G",
      },
      {
        id: "copilot_professional",
        name: "Copilot Professional",
        action: "Rewrites selection in formal business tone",
        shortcut: "Ctrl+Alt+P",
      },
      {
        id: "copilot_summarize",
        name: "Copilot Summarize",
        action: "Summarizes selection into bullet points",
        shortcut: "Ctrl+Alt+S",
      },
      {
        id: "ai_expand",
        name: "Trigger AI Prompt",
        action: "Invokes Groq/Cerebras AI prompt pipeline",
        shortcut: "Ctrl+Alt+X",
      },
      {
        id: "toggle_engine",
        name: "Toggle KeyMind Interceptor",
        action: "Pauses or resumes global keyboard interception",
        shortcut: "Ctrl+Alt+K",
      },
    ]);
    setLastRecorded("Reset all keybindings to defaults");
  };

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header Bar */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
            <div className="p-2.5 bg-[#DA7756]/15 rounded-xl text-[#DA7756] border border-[#DA7756]/20">
              <Keyboard className="w-5 h-5" />
            </div>
            Global Keybindings & Shortcuts
          </h2>
          <p className="text-xs text-[#A1A0AB] mt-1">
            System-wide shortcuts conflict-free with browsers and IDEs. Click any row to record a new key combination.
          </p>
        </div>

        <button
          onClick={handleResetDefaults}
          className="flex items-center gap-2 px-4 py-2 bg-[#1B1A22] border border-[#DA7756]/20 hover:border-[#DA7756]/50 rounded-2xl text-xs font-semibold text-[#FAF8F5] transition shadow-sm cursor-pointer active:scale-95"
        >
          <RotateCcw className="w-3.5 h-3.5 text-[#A1A0AB]" />
          Reset to Defaults
        </button>
      </div>

      {lastRecorded && (
        <div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-2xl text-xs text-emerald-300 font-mono flex items-center gap-2 animate-fade-in">
          <Check className="w-4 h-4 text-emerald-400" />
          <span>{lastRecorded}</span>
        </div>
      )}

      {/* Shortcuts Glass Table */}
      <div className="glass-panel rounded-3xl overflow-hidden shadow-2xl">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-[#DA7756]/15 bg-[#16151B]/80 text-xs font-bold uppercase tracking-wider text-[#A1A0AB]">
              <th className="py-4 px-6">Action Name</th>
              <th className="py-4 px-6">Description</th>
              <th className="py-4 px-6 text-right">Shortcut Combination</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-[#DA7756]/10 text-sm">
            {shortcuts.map((sc) => {
              const isCapturing = capturingId === sc.id;
              return (
                <tr
                  key={sc.id}
                  onClick={() => handleRowClick(sc.id)}
                  className={`cursor-pointer transition-all duration-200 ${
                    isCapturing
                      ? "bg-[#DA7756]/15 border-l-4 border-[#DA7756]"
                      : "hover:bg-white/[0.03]"
                  }`}
                >
                  <td className="py-4 px-6 font-bold text-[#FAF8F5] flex items-center gap-2">
                    <Sparkles className="w-3.5 h-3.5 text-[#DA7756]" />
                    {sc.name}
                  </td>
                  <td className="py-4 px-6 text-xs text-[#A1A0AB]">{sc.action}</td>
                  <td className="py-4 px-6 text-right">
                    {isCapturing ? (
                      <span className="inline-flex items-center gap-1.5 px-3.5 py-1.5 bg-[#DA7756] text-white rounded-xl text-xs font-mono font-bold animate-pulse shadow-lg shadow-[#DA7756]/30">
                        Press key combo (Esc to cancel)...
                      </span>
                    ) : (
                      <kbd className="px-3.5 py-1.5 bg-[#1B1A22] border border-[#DA7756]/30 rounded-xl text-xs font-mono font-bold text-[#DA7756] shadow-inner">
                        {sc.shortcut}
                      </kbd>
                    )}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>

      <div className="flex items-center gap-2 text-xs text-[#71707C] p-2">
        <Info className="w-4 h-4 text-[#DA7756]" />
        <span>Tip: Shortcuts use global Windows Low-Level Hook (`SetWindowsHookExW`) to capture keypresses instantly without input delay.</span>
      </div>
    </div>
  );
};
