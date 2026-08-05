import React from "react";
import { AppSettings } from "../types";
import { AppWindow, ShieldAlert, CheckCircle2 } from "lucide-react";

interface AppsTabProps {
  apps: AppSettings[];
  onUpdateApp: (updated: AppSettings) => void;
}

export const AppsTab: React.FC<AppsTabProps> = ({ apps, onUpdateApp }) => {
  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
          <div className="p-2.5 bg-[#DA7756]/15 rounded-xl text-[#DA7756] border border-[#DA7756]/20">
            <AppWindow className="w-5 h-5" />
          </div>
          Per-Application Rules & Preferences
        </h2>
        <p className="text-xs text-[#A1A0AB] mt-1">
          Customize or block KeyMind features per application bundle (e.g. Safari, Slack, VS Code).
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4">
        {apps.map((app) => (
          <div
            key={app.app_bundle_id}
            className="glass-panel glass-panel-hover p-6 rounded-3xl flex items-center justify-between"
          >
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 rounded-2xl bg-gradient-to-tr from-[#C86544] to-[#DA7756] flex items-center justify-center font-bold text-white shadow-md">
                {app.app_name.charAt(0)}
              </div>
              <div>
                <h3 className="text-base font-extrabold text-[#FAF8F5]">{app.app_name}</h3>
                <p className="text-xs text-[#71707C] font-mono">{app.app_bundle_id}</p>
              </div>
            </div>

            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <span className="text-xs font-semibold text-[#A1A0AB]">Autocorrect</span>
                <input
                  type="checkbox"
                  checked={app.autocorrect_enabled}
                  onChange={(e) =>
                    onUpdateApp({ ...app, autocorrect_enabled: e.target.checked })
                  }
                  className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
                />
              </div>

              <div className="flex items-center gap-2">
                <span className="text-xs font-semibold text-[#A1A0AB]">Grammar</span>
                <input
                  type="checkbox"
                  checked={app.grammar_enabled}
                  onChange={(e) =>
                    onUpdateApp({ ...app, grammar_enabled: e.target.checked })
                  }
                  className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
                />
              </div>

              <div className="flex items-center gap-2">
                <span className="text-xs font-semibold text-[#A1A0AB]">AI Copilot</span>
                <input
                  type="checkbox"
                  checked={app.ai_copilot_enabled}
                  onChange={(e) =>
                    onUpdateApp({ ...app, ai_copilot_enabled: e.target.checked })
                  }
                  className="w-4 h-4 rounded accent-[#DA7756] cursor-pointer"
                />
              </div>

              <button
                onClick={() => onUpdateApp({ ...app, is_blocked: !app.is_blocked })}
                className={`px-3 py-1.5 rounded-xl text-xs font-bold transition cursor-pointer flex items-center gap-1.5 ${
                  app.is_blocked
                    ? "bg-rose-500/20 text-rose-300 border border-rose-500/30"
                    : "bg-zinc-800 text-[#A1A0AB] hover:text-white"
                }`}
              >
                {app.is_blocked ? (
                  <>
                    <ShieldAlert className="w-3.5 h-3.5" /> Blocked
                  </>
                ) : (
                  <>
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" /> Active
                  </>
                )}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
