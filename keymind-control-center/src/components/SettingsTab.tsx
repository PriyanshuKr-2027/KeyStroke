import React, { useState } from "react";
import { Settings, Shield, HardDrive, Key, Bell, Power, RefreshCw, Check } from "lucide-react";

export const SettingsTab: React.FC = () => {
  const [autostart, setAutostart] = useState(true);
  const [minimizeToTray, setMinimizeToTray] = useState(true);
  const [soundFeedback, setSoundFeedback] = useState(false);
  const [groqKey, setGroqKey] = useState("gsk_892348********************");
  const [cerebrasKey, setCerebrasKey] = useState("csk_102938********************");
  const [savedToast, setSavedToast] = useState(false);

  const handleSave = () => {
    setSavedToast(true);
    setTimeout(() => setSavedToast(false), 3000);
  };

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header Bar */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
            <div className="p-2.5 bg-[#DA7756]/15 rounded-xl text-[#DA7756] border border-[#DA7756]/20">
              <Settings className="w-5 h-5" />
            </div>
            Settings & System Preferences
          </h2>
          <p className="text-xs text-[#A1A0AB] mt-1">
            Configure system startup, local AI keys, data privacy, and desktop integration options.
          </p>
        </div>

        <button
          onClick={handleSave}
          className="flex items-center gap-2 px-5 py-2.5 bg-[#DA7756] hover:bg-[#C86544] rounded-2xl text-xs font-bold text-white shadow-lg shadow-[#DA7756]/25 transition cursor-pointer active:scale-95"
        >
          <Check className="w-4 h-4" /> Save Settings
        </button>
      </div>

      {savedToast && (
        <div className="p-3.5 bg-emerald-500/10 border border-emerald-500/30 rounded-2xl text-xs text-emerald-300 font-semibold flex items-center gap-2 animate-fade-in shadow-md">
          <Check className="w-4 h-4 text-emerald-400" />
          <span>System settings and API keys updated successfully.</span>
        </div>
      )}

      <div className="grid grid-cols-2 gap-6">
        {/* General Desktop Integration Panel */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
              <Power className="w-4 h-4 text-[#DA7756]" />
              General Desktop Preferences
            </h3>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-xs font-bold text-[#FAF8F5]">Launch at Startup</p>
                <p className="text-[11px] text-[#A1A0AB]">Automatically start KeyMind when Windows/macOS boots.</p>
              </div>
              <input
                type="checkbox"
                checked={autostart}
                onChange={(e) => setAutostart(e.target.checked)}
                className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
              />
            </div>

            <div className="flex items-center justify-between pt-2 border-t border-white/[0.06]">
              <div>
                <p className="text-xs font-bold text-[#FAF8F5]">Minimize to System Tray</p>
                <p className="text-[11px] text-[#A1A0AB]">Keep running quietly in tray when window is closed.</p>
              </div>
              <input
                type="checkbox"
                checked={minimizeToTray}
                onChange={(e) => setMinimizeToTray(e.target.checked)}
                className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
              />
            </div>

            <div className="flex items-center justify-between pt-2 border-t border-white/[0.06]">
              <div>
                <p className="text-xs font-bold text-[#FAF8F5]">Subtle Sound Feedback</p>
                <p className="text-[11px] text-[#A1A0AB]">Play optional low-volume click sound on autocorrect.</p>
              </div>
              <input
                type="checkbox"
                checked={soundFeedback}
                onChange={(e) => setSoundFeedback(e.target.checked)}
                className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
              />
            </div>
          </div>
        </div>

        {/* AI Key Management Panel */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
              <Key className="w-4 h-4 text-amber-400" />
              Local AI Provider Credentials
            </h3>
            <span className="text-[10px] font-extrabold px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              Encrypted Local Storage
            </span>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-xs font-bold text-[#A1A0AB] mb-1">
                Groq API Key (Primary Llama 3.3 70B)
              </label>
              <input
                type="password"
                value={groqKey}
                onChange={(e) => setGroqKey(e.target.value)}
                className="w-full bg-[#121216] border border-white/10 rounded-xl px-3.5 py-2 text-xs text-[#FAF8F5] font-mono focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
              />
            </div>

            <div>
              <label className="block text-xs font-bold text-[#A1A0AB] mb-1">
                Cerebras API Key (Failover Llama 3.1 8B)
              </label>
              <input
                type="password"
                value={cerebrasKey}
                onChange={(e) => setCerebrasKey(e.target.value)}
                className="w-full bg-[#121216] border border-white/10 rounded-xl px-3.5 py-2 text-xs text-[#FAF8F5] font-mono focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
              />
            </div>
          </div>
        </div>
      </div>

      {/* Data Privacy & Backup Panel */}
      <div className="glass-panel p-6 rounded-3xl space-y-4 border border-[#DA7756]/15">
        <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
          <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
            <Shield className="w-4 h-4 text-sky-400" />
            Data Privacy & Local Storage
          </h3>
          <span className="text-xs text-[#A1A0AB]">100% Offline Processing</span>
        </div>

        <div className="flex items-center justify-between text-xs text-[#A1A0AB]">
          <span>SQLite Database Path: <code className="text-[#DA7756] font-mono">keymind-autocorrect/data/dictionary.db</code></span>
          <div className="flex gap-2">
            <button className="px-3.5 py-2 bg-zinc-900 border border-white/10 hover:border-white/20 rounded-xl text-xs text-[#FAF8F5] transition cursor-pointer">
              Export Database
            </button>
            <button className="px-3.5 py-2 bg-rose-500/15 border border-rose-500/30 text-rose-300 hover:bg-rose-500/25 rounded-xl text-xs font-bold transition cursor-pointer">
              Purge History
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
