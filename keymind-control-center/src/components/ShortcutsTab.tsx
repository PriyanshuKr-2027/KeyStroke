import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { ShortcutBinding } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Pencil, Keyboard, RotateCcw, AlertTriangle, X } from "lucide-react";

interface BackendShortcut {
  id: string;
  label: string;
  default_binding: string;
  current_binding: string;
}

interface ShortcutsTabProps {
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
  onShowToast?: (title: string, message?: string, type?: "success" | "error" | "info") => void;
}

const DEFAULT_SHORTCUTS: ShortcutBinding[] = [
  {
    id: "copilot_palette",
    name: "AI Copilot Palette",
    action: "Opens AI Copilot floating palette window",
    shortcut: "Ctrl + Alt + Space",
  },
  {
    id: "grammar_fix",
    name: "Grammar Fix Selection",
    action: "Fixes highlighted text via LanguageTool / AI",
    shortcut: "Ctrl + Alt + G",
  },
  {
    id: "tone_rewrite",
    name: "Tone Rewriter — Formal",
    action: "Rewrites selection in formal business tone",
    shortcut: "Ctrl + Alt + P",
  },
  {
    id: "summarize",
    name: "Summarize Selection",
    action: "Summarizes selection into bullet points",
    shortcut: "Ctrl + Alt + S",
  },
  {
    id: "toggle_engine",
    name: "Toggle Interceptor On/Off",
    action: "Pauses or resumes global keyboard interception",
    shortcut: "Ctrl + Alt + K",
  },
];

export const ShortcutsTab: React.FC<ShortcutsTabProps> = ({
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
  onShowToast,
}) => {
  const [showCallout, setShowCallout] = useState(true);
  const [shortcuts, setShortcuts] = useState<ShortcutBinding[]>(DEFAULT_SHORTCUTS);
  const [recordingId, setRecordingId] = useState<string | null>(null);
  const [pressedKeys, setPressedKeys] = useState<string[]>([]);
  const [conflictWarning, setConflictWarning] = useState<string | null>(null);

  useEffect(() => {
    invoke<BackendShortcut[]>("get_shortcuts_list")
      .then((res) => {
        if (res && res.length > 0) {
          setShortcuts((prev) =>
            prev.map((s) => {
              const found = res.find((b) => b.id === s.id);
              return found ? { ...s, shortcut: found.current_binding } : s;
            })
          );
        }
      })
      .catch(() => {});
  }, []);

  // Key recording listener
  useEffect(() => {
    if (!recordingId) {
      setPressedKeys([]);
      setConflictWarning(null);
      return;
    }

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();

      if (e.key === "Escape") {
        setRecordingId(null);
        setPressedKeys([]);
        setConflictWarning(null);
        return;
      }

      const parts: string[] = [];
      if (e.ctrlKey) parts.push("Ctrl");
      if (e.altKey) parts.push("Alt");
      if (e.shiftKey) parts.push("Shift");

      let keyName = e.key.length === 1 ? e.key.toUpperCase() : e.key;
      if (keyName === " ") keyName = "Space";

      if (!["Control", "Alt", "Shift", "Meta"].includes(keyName)) {
        parts.push(keyName);
        const combo = parts.join(" + ");

        // Conflict check
        if (["Ctrl + C", "Ctrl + V", "Ctrl + X", "Ctrl + Z", "Ctrl + A"].includes(combo)) {
          setConflictWarning(`"${combo}" is an OS clipboard shortcut! Recording ignored.`);
          return;
        }

        setShortcuts((prev) =>
          prev.map((s) => (s.id === recordingId ? { ...s, shortcut: combo } : s))
        );

        invoke("update_shortcut_binding", {
          id: recordingId,
          binding: combo,
        })
          .then(() => {
            if (onShowToast) onShowToast("Shortcut Updated", `Set to ${combo}`);
          })
          .catch((err) => console.error("Failed to update binding:", err));

        setRecordingId(null);
        setPressedKeys([]);
        setConflictWarning(null);
      } else {
        setPressedKeys(parts);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [recordingId, onShowToast]);

  const handleResetDefaults = () => {
    setShortcuts(DEFAULT_SHORTCUTS);
    if (onShowToast) onShowToast("Reset Defaults", "Restored original global keybindings", "info");
  };

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10 font-sans">
        <ErrorState
          title="Keybindings Unavailable"
          message={errorMessage || "Unable to register or query global keyboard shortcuts."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10 font-sans select-none text-[#EDEDED]">
      {/* Top Header & Actions */}
      <div className="flex items-center justify-between">
        <h1 className="text-[22px] font-semibold text-[#EDEDED] tracking-tight">
          Shortcuts
        </h1>

        <button
          onClick={handleResetDefaults}
          className="flex items-center gap-1.5 px-3 py-1.5 bg-[#28282D] hover:bg-[#333338] border border-[rgba(255,255,255,0.08)] rounded-[7px] text-[12px] font-medium text-[#8F8F96] hover:text-[#EDEDED] transition cursor-pointer"
        >
          <RotateCcw className="w-3.5 h-3.5" />
          <span>Reset Defaults</span>
        </button>
      </div>

      {/* Callout */}
      {showCallout && (
        <CalloutCard
          headline="One key combination. Instant action."
          body="Press any shortcut below to trigger KeyStroke instantly across any desktop app. Click a row to record a new combination."
          chips={[
            { label: "Ctrl+Alt+Space" },
            { label: "Ctrl+Alt+G" },
            { label: "Ctrl+Alt+K" },
          ]}
          ctaLabel="Customize shortcuts"
          onCtaClick={() => setRecordingId("copilot_palette")}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      <div className="text-[11px] font-mono font-semibold tracking-wider text-[#8F8F96] uppercase pt-2">
        GLOBAL KEYBINDINGS
      </div>

      {/* Shortcuts List */}
      {isLoading ? (
        <TableRowSkeleton count={5} />
      ) : shortcuts.length > 0 ? (
        <div className="divide-y divide-[rgba(255,255,255,0.08)] border-t border-b border-[rgba(255,255,255,0.08)]">
          {shortcuts.map((sc) => {
            const isRecording = recordingId === sc.id;
            return (
              <div
                key={sc.id}
                onClick={() => setRecordingId(sc.id)}
                className="h-[52px] px-3 flex items-center justify-between hover:bg-[rgba(255,255,255,0.03)] transition rounded-[8px] cursor-pointer group"
              >
                <div>
                  <p className="text-[14px] font-medium text-[#EDEDED]">{sc.name}</p>
                  <p className="text-[12px] text-[#8F8F96]">{sc.action}</p>
                </div>

                <div className="flex items-center gap-3">
                  <span className="px-2.5 py-1 bg-[#28282D] border border-[rgba(255,255,255,0.08)] rounded-[6px] font-mono text-[12px] text-[#EDEDED]">
                    {sc.shortcut}
                  </span>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setRecordingId(sc.id);
                    }}
                    className="text-[#8F8F96] hover:text-[#EDEDED] opacity-0 group-hover:opacity-100 transition p-1 cursor-pointer"
                    title="Click to record new shortcut"
                  >
                    <Pencil className="w-4 h-4" />
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      ) : (
        <EmptyState
          icon={Keyboard}
          title="No keybindings registered"
          description="Global hotkeys allow you to summon the Copilot Palette or trigger instant grammar fixes."
          actionLabel="Reset Default Keybindings"
          onAction={handleResetDefaults}
        />
      )}

      {/* Recording Modal Overlay */}
      {recordingId && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center animate-fade-in">
          <div className="w-[420px] bg-[#161618] border border-[rgba(255,255,255,0.12)] rounded-[14px] p-6 shadow-2xl space-y-4 animate-pop-in text-center">
            <div className="flex items-center justify-between">
              <span className="text-[13px] font-mono text-[#6366F1] font-semibold">RECORDING KEYBINDING</span>
              <button
                onClick={() => setRecordingId(null)}
                className="text-[#8F8F96] hover:text-[#EDEDED]"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            <p className="text-[15px] font-semibold text-[#EDEDED]">
              Press your key combination now
            </p>

            <div className="py-6 px-4 bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[10px] flex items-center justify-center gap-2 animate-record-glow min-h-[64px]">
              {pressedKeys.length > 0 ? (
                pressedKeys.map((k, idx) => (
                  <kbd key={idx} className="px-3 py-1.5 bg-[#28282D] border border-[rgba(255,255,255,0.12)] rounded-[6px] font-mono text-[14px] text-[#EDEDED] shadow-sm">
                    {k}
                  </kbd>
                ))
              ) : (
                <span className="text-[13px] font-mono text-[#8F8F96]">
                  Hold modifiers (Ctrl, Alt, Shift) + Key...
                </span>
              )}
            </div>

            {conflictWarning && (
              <div className="flex items-center gap-2 text-[12px] text-[#EF4444] bg-[#EF4444]/10 p-2.5 rounded-[8px] border border-[#EF4444]/20">
                <AlertTriangle className="w-4 h-4 shrink-0" />
                <span>{conflictWarning}</span>
              </div>
            )}

            <p className="text-[12px] text-[#8F8F96]">
              Press <kbd className="px-1.5 py-0.5 bg-[#28282D] rounded font-mono">Esc</kbd> to cancel
            </p>
          </div>
        </div>
      )}
    </div>
  );
};
