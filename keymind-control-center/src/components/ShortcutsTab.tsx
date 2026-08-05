import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { ShortcutBinding } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Pencil, Keyboard } from "lucide-react";

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
}

export const ShortcutsTab: React.FC<ShortcutsTabProps> = ({
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [showCallout, setShowCallout] = useState(true);
  const [shortcuts, setShortcuts] = useState<ShortcutBinding[]>([
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
  ]);

  const [recordingId, setRecordingId] = useState<string | null>(null);

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

  useEffect(() => {
    if (!recordingId) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();

      if (e.key === "Escape") {
        setRecordingId(null);
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
        const combo = parts.join(" + ");

        setShortcuts((prev) =>
          prev.map((s) => (s.id === recordingId ? { ...s, shortcut: combo } : s))
        );

        invoke("update_shortcut_binding", {
          id: recordingId,
          binding: combo,
        }).catch((err) => console.error("Failed to update shortcut binding:", err));

        setRecordingId(null);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [recordingId]);

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="Keybindings Unavailable"
          message={errorMessage || "Unable to register or query global keyboard shortcuts."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Header */}
      <h1 className="font-sans text-[22px] font-semibold text-[#111111]">
        Shortcuts
      </h1>

      {/* Callout Card */}
      {showCallout && (
        <CalloutCard
          headline="One key combination. Instant action."
          body="Press any shortcut below to trigger KeyMind instantly across any app. Click a row to record a new combination."
          chips={[
            { label: "Ctrl+Alt+Space" },
            { label: "Ctrl+Alt+G" },
            { label: "Ctrl+Alt+K" },
          ]}
          ctaLabel="Customize shortcuts"
          onCtaClick={() => window.scrollTo({ top: 300, behavior: "smooth" })}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* List Header */}
      <div className="text-[11px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase pt-2">
        GLOBAL KEYBINDINGS
      </div>

      {/* Shortcuts List */}
      {isLoading ? (
        <TableRowSkeleton count={5} />
      ) : shortcuts.length > 0 ? (
        <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
          {shortcuts.map((sc) => {
            const isRecording = recordingId === sc.id;
            return (
              <div
                key={sc.id}
                onClick={() => setRecordingId(sc.id)}
                className="h-[48px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors cursor-pointer group"
              >
                {/* Action Name */}
                <span className="font-sans text-[14px] text-[#111111]">
                  {sc.name}
                </span>

                {/* Right Side: Keybinding or Recording Capture Zone */}
                <div className="flex items-center gap-3">
                  {isRecording ? (
                    <div className="px-3 py-1 bg-[#F5F5F5] border border-[#111111] rounded-[6px] text-[13px] font-mono text-[#111111] animate-pulse">
                      Press any key combination... Esc to cancel
                    </div>
                  ) : (
                    <>
                      <span className="font-mono text-[13px] text-[#111111]">
                        {sc.shortcut}
                      </span>
                      <button
                        className="text-[#AAAAAA] hover:text-[#111111] opacity-0 group-hover:opacity-100 transition-opacity p-1"
                        title="Edit keybinding"
                      >
                        <Pencil className="w-4 h-4" />
                      </button>
                    </>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      ) : (
        <EmptyState
          icon={Keyboard}
          title="No keybindings registered"
          description="Global hotkeys allow you to summon the Copilot Palette or trigger instant grammar fixes from anywhere on your system."
          actionLabel="Reset Default Keybindings"
          onAction={onRetry}
        />
      )}
    </div>
  );
};
