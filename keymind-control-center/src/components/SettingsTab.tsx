import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { SettingsRowGroup } from "./SettingsRowGroup";
import { X, Check } from "lucide-react";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SettingsTab: React.FC<SettingsModalProps> = ({
  isOpen,
  onClose,
}) => {
  const [activeSection, setActiveSection] = useState<
    "general" | "system" | "ai" | "account" | "billing" | "privacy"
  >("general");

  // State values
  const [autostart, setAutostart] = useState(true);
  const [minimizeTray, setMinimizeTray] = useState(true);
  const [showTaskbar, setShowTaskbar] = useState(true);
  const [soundFeedback, setSoundFeedback] = useState(false);
  const [soundPrediction, setSoundPrediction] = useState(false);
  const [groqKey, setGroqKey] = useState("");
  const [cerebrasKey, setCerebrasKey] = useState("");
  const [groqValid, setGroqValid] = useState(false);
  const [cerebrasValid, setCerebrasValid] = useState(false);
  const [isTestingKeys, setIsTestingKeys] = useState(false);

  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");

  useEffect(() => {
    if (isOpen) {
      invoke<{ groq_valid: boolean; cerebras_valid: boolean }>("get_ai_keys_status")
        .then((res) => {
          setGroqValid(res.groq_valid);
          setCerebrasValid(res.cerebras_valid);
        })
        .catch((err) => console.error("get_ai_keys_status error:", err));
    }
  }, [isOpen]);

  const handleToggleAutostart = (enabled: boolean) => {
    setAutostart(enabled);
    if (enabled) {
      invoke("install_launch_agent").catch((err) => console.error("install_launch_agent error:", err));
    } else {
      invoke("uninstall_launch_agent").catch((err) => console.error("uninstall_launch_agent error:", err));
    }
  };

  const handleSaveAiKeys = async () => {
    setIsTestingKeys(true);
    try {
      const res = await invoke<{ groq_valid: boolean; cerebras_valid: boolean }>(
        "save_ai_provider_keys",
        { groqKey: groqKey || undefined, cerebrasKey: cerebrasKey || undefined }
      );
      setGroqValid(res.groq_valid);
      setCerebrasValid(res.cerebras_valid);
    } catch (err) {
      console.error("save_ai_provider_keys error:", err);
    } finally {
      setIsTestingKeys(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/30 backdrop-blur-[2px] flex items-center justify-center p-4 z-50 animate-fade-in select-none">
      <div className="w-[860px] h-[560px] bg-[#FFFFFF] rounded-[16px] shadow-2xl border border-[#EBEBEB] flex overflow-hidden relative">
        {/* Close Button */}
        <button
          onClick={onClose}
          className="absolute top-4 right-4 text-[#AAAAAA] hover:text-[#111111] p-1 transition cursor-pointer z-10"
        >
          <X className="w-5 h-5" />
        </button>

        {/* Left Sub-nav (200px) */}
        <div className="w-[200px] min-w-[200px] bg-[#FFFFFF] border-r border-[#EBEBEB] p-4 flex flex-col justify-between">
          <div className="space-y-6">
            <div className="text-[12px] font-semibold text-[#111111] px-2 uppercase tracking-wider">
              SETTINGS
            </div>

            {/* General Section */}
            <div className="space-y-1">
              {[
                { id: "general", label: "General" },
                { id: "system", label: "System" },
                { id: "ai", label: "AI & Copilot" },
              ].map((item) => {
                const isActive = activeSection === item.id;
                return (
                  <button
                    key={item.id}
                    onClick={() => setActiveSection(item.id as any)}
                    className={`w-full h-[36px] px-3 rounded-[6px] text-[13px] font-normal text-left transition cursor-pointer ${
                      isActive
                        ? "bg-[#F0F0F0] text-[#111111]"
                        : "text-[#6B6B6B] hover:bg-[#F5F5F5] hover:text-[#111111]"
                    }`}
                  >
                    {item.label}
                  </button>
                );
              })}
            </div>

            {/* Account Section */}
            <div className="space-y-1">
              <div className="text-[11px] font-semibold text-[#AAAAAA] px-2 uppercase tracking-wider mb-2">
                ACCOUNT
              </div>
              {[
                { id: "account", label: "Account" },
                { id: "billing", label: "Plans & Billing" },
                { id: "privacy", label: "Data & Privacy" },
              ].map((item) => {
                const isActive = activeSection === item.id;
                return (
                  <button
                    key={item.id}
                    onClick={() => setActiveSection(item.id as any)}
                    className={`w-full h-[36px] px-3 rounded-[6px] text-[13px] font-normal text-left transition cursor-pointer ${
                      isActive
                        ? "bg-[#F0F0F0] text-[#111111]"
                        : "text-[#6B6B6B] hover:bg-[#F5F5F5] hover:text-[#111111]"
                    }`}
                  >
                    {item.label}
                  </button>
                );
              })}
            </div>
          </div>

          <div className="px-2 text-[11px] text-[#AAAAAA] font-mono">
            v1.0.0 — Wispr Flow
          </div>
        </div>

        {/* Right Scrollable Content Panel */}
        <div className="flex-1 p-8 overflow-y-auto space-y-6">
          {activeSection === "general" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                General Settings
              </h2>
              <SettingsRowGroup
                items={[
                  {
                    id: "shortcuts",
                    label: "Keyboard shortcuts",
                    subtitle: "Customize global system hotkeys",
                    type: "button",
                    buttonLabel: "Change",
                  },
                  {
                    id: "language",
                    label: "Primary language",
                    subtitle: "English (US)",
                    type: "button",
                    buttonLabel: "Change",
                  },
                  {
                    id: "sound",
                    label: "Sound feedback on autocorrect",
                    subtitle: "Low volume click on replacement",
                    type: "toggle",
                    checked: soundFeedback,
                    onToggle: (v) => {
                      setSoundFeedback(v);
                      invoke("update_system_setting", { key: "sound_feedback", value: v }).catch(console.error);
                    },
                  },
                ]}
              />
            </div>
          )}

          {activeSection === "system" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                System Preferences
              </h2>
              <SettingsRowGroup
                items={[
                  {
                    id: "autostart",
                    label: "Launch at login",
                    subtitle: "Start KeyStroke background daemon on startup",
                    type: "toggle",
                    checked: autostart,
                    onToggle: handleToggleAutostart,
                  },
                  {
                    id: "tray",
                    label: "Minimize to system tray on close",
                    subtitle: "Keep running silently in system tray",
                    type: "toggle",
                    checked: minimizeTray,
                    onToggle: (v) => {
                      setMinimizeTray(v);
                      invoke("update_system_setting", { key: "minimize_tray", value: v }).catch(console.error);
                    },
                  },
                  {
                    id: "taskbar",
                    label: "Show in taskbar",
                    subtitle: "Display main window icon when active",
                    type: "toggle",
                    checked: showTaskbar,
                    onToggle: (v) => {
                      setShowTaskbar(v);
                      invoke("update_system_setting", { key: "show_taskbar", value: v }).catch(console.error);
                    },
                  },
                  {
                    id: "sound_pred",
                    label: "Prediction chip sound",
                    subtitle: "Audible click when suggestion appears",
                    type: "toggle",
                    checked: soundPrediction,
                    onToggle: (v) => {
                      setSoundPrediction(v);
                      invoke("update_system_setting", { key: "sound_prediction", value: v }).catch(console.error);
                    },
                  },
                ]}
              />
            </div>
          )}

          {activeSection === "ai" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                AI & Copilot Configuration
              </h2>

              <div className="space-y-4">
                <div>
                  <div className="flex items-center justify-between mb-1.5">
                    <label className="text-[13px] font-medium text-[#111111]">
                      Groq API Key (Primary)
                    </label>
                    {groqValid ? (
                      <span className="px-2 py-0.5 bg-[#DCFCE7] text-[#16A34A] text-[11px] font-mono rounded-[4px]">
                        Connected
                      </span>
                    ) : (
                      <span className="px-2 py-0.5 bg-[#F5F5F5] text-[#6B6B6B] text-[11px] font-mono rounded-[4px]">
                        Not Connected
                      </span>
                    )}
                  </div>
                  <div className="flex gap-2">
                    <input
                      type="password"
                      placeholder="Enter Groq API Key (gsk_...)"
                      value={groqKey}
                      onChange={(e) => setGroqKey(e.target.value)}
                      className="flex-1 bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#111111] focus:outline-none"
                    />
                    <button
                      onClick={handleSaveAiKeys}
                      disabled={isTestingKeys}
                      className="px-3 py-2 bg-[#111111] text-[#FFFFFF] text-[13px] font-medium rounded-[8px] cursor-pointer disabled:opacity-50"
                    >
                      {isTestingKeys ? "Testing..." : "Save & Test"}
                    </button>
                  </div>
                </div>

                <div>
                  <div className="flex items-center justify-between mb-1.5">
                    <label className="block text-[13px] font-medium text-[#111111]">
                      Cerebras API Key (Failover)
                    </label>
                    {cerebrasValid ? (
                      <span className="px-2 py-0.5 bg-[#DCFCE7] text-[#16A34A] text-[11px] font-mono rounded-[4px]">
                        Connected
                      </span>
                    ) : (
                      <span className="px-2 py-0.5 bg-[#F5F5F5] text-[#6B6B6B] text-[11px] font-mono rounded-[4px]">
                        Not Connected
                      </span>
                    )}
                  </div>
                  <input
                    type="password"
                    placeholder="Enter Cerebras API Key (csk_...)"
                    value={cerebrasKey}
                    onChange={(e) => setCerebrasKey(e.target.value)}
                    className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#111111] focus:outline-none"
                  />
                </div>

                <SettingsRowGroup
                  items={[
                    {
                      id: "failover",
                      label: "Failover behavior",
                      subtitle: "Auto switch to Cerebras on 429/5xx API errors",
                      type: "toggle",
                      checked: true,
                    },
                  ]}
                />
              </div>
            </div>
          )}

          {activeSection === "account" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                Account & Profile
              </h2>

              <div className="flex items-center gap-4 pb-2 border-b border-[#EBEBEB]">
                <div className="w-14 h-14 rounded-full bg-[#F5F5F5] border border-[#EBEBEB] flex items-center justify-center font-bold text-[18px] text-[#111111]">
                  AD
                </div>
                <div>
                  <p className="text-[14px] font-medium text-[#111111]">{firstName} {lastName}</p>
                  <p className="text-[13px] text-[#6B6B6B]">{email}</p>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-[13px] font-medium text-[#111111] mb-1">First Name</label>
                  <input
                    type="text"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                    className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[14px] text-[#111111]"
                  />
                </div>
                <div>
                  <label className="block text-[13px] font-medium text-[#111111] mb-1">Last Name</label>
                  <input
                    type="text"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                    className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[14px] text-[#111111]"
                  />
                </div>
              </div>

              <div className="flex items-center justify-between pt-4 border-t border-[#EBEBEB]">
                <button
                  onClick={async () => { if (window.confirm("Are you sure you want to delete all data? This cannot be undone.")) { await invoke("purge_database"); } }}
                  className="text-[#EF4444] text-[13px] font-medium cursor-pointer hover:underline"
                >
                  Delete account
                </button>
                <button
                  onClick={async () => {
                    try {
                      await invoke("save_profile", { firstName, lastName, email });
                      onClose();
                    } catch (e) { console.error("Failed to save profile:", e); }
                  }}
                  className="px-4 py-2 bg-[#111111] text-[#FFFFFF] text-[14px] font-medium rounded-[8px]"
                >
                  Save changes
                </button>
              </div>
            </div>
          )}

          {activeSection === "billing" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                Plans & Billing
              </h2>
              <div className="p-4 bg-[#F5F5F5] rounded-[10px] space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-[14px] font-semibold text-[#111111]">KeyStroke Pro Plan</span>
                  <span className="px-2.5 py-1 bg-[#111111] text-[#FFFFFF] text-[12px] font-medium rounded-[6px]">
                    ACTIVE
                  </span>
                </div>
                <p className="text-[13px] text-[#6B6B6B]">
                  Unlimited local autocorrect, instant Groq AI prompts, and full team dictionary sync.
                </p>
              </div>
            </div>
          )}

          {activeSection === "privacy" && (
            <div className="space-y-5">
              <h2 className="font-sans text-[18px] font-semibold text-[#111111]">
                Data & Privacy
              </h2>

              <div className="p-4 bg-[#F5F5F5] rounded-[10px] flex items-center gap-3">
                <span className="w-2.5 h-2.5 rounded-full bg-[#22C55E]" />
                <span className="text-[14px] text-[#111111]">100% Local Engine Processing Verified</span>
              </div>

              <div className="space-y-2 pt-2">
                <button
                  onClick={async () => {
                    try {
                      const data = await invoke<string>("export_local_data");
                      const blob = new Blob([data], { type: "application/json" });
                      const url = URL.createObjectURL(blob);
                      const a = document.createElement("a");
                      a.href = url;
                      a.download = "keystroke_backup.json";
                      a.click();
                      URL.revokeObjectURL(url);
                    } catch (e) {
                      console.error(e);
                    }
                  }}
                  className="w-full h-[40px] px-4 bg-[#F5F5F5] text-[#111111] text-[14px] font-medium rounded-[8px] text-left hover:bg-[#EBEBEB]">
                  Export all local data
                </button>
                <button
                  onClick={async () => { if (window.confirm("Clear all activity history?")) { await invoke("clear_activity_history"); } }}
                  className="w-full h-[40px] px-4 bg-[#F5F5F5] text-[#EF4444] text-[14px] font-medium rounded-[8px] text-left hover:bg-[#FEE2E2]">
                  Clear activity history
                </button>
                <button
                  onClick={async () => { if (window.confirm("Purge entire database? This will reset everything.")) { await invoke("purge_database"); } }}
                  className="w-full h-[40px] px-4 bg-[#FEE2E2] text-[#EF4444] text-[14px] font-semibold rounded-[8px] text-left hover:bg-[#FCA5A5]">
                  Purge database
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
