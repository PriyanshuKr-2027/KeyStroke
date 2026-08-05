import React from "react";
import {
  LayoutDashboard,
  Code2,
  CheckCheck,
  Keyboard,
  AppWindow,
  Brain,
  Zap,
  Activity,
  Settings,
} from "lucide-react";

export type TabType =
  | "dashboard"
  | "variables"
  | "grammar"
  | "shortcuts"
  | "apps"
  | "memory"
  | "settings";

interface SidebarProps {
  activeTab: TabType;
  setActiveTab: (tab: TabType) => void;
  engineRunning: boolean;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  setActiveTab,
  engineRunning,
}) => {
  const tabs = [
    { id: "dashboard", label: "Overview", icon: LayoutDashboard, shortcut: "⌘1" },
    { id: "variables", label: "Snippets & Variables", icon: Code2, shortcut: "⌘2" },
    { id: "grammar", label: "Grammar & Autocorrect", icon: CheckCheck, badge: "AI", shortcut: "⌘3" },
    { id: "memory", label: "Memory & Dictionary", icon: Brain, shortcut: "⌘4" },
    { id: "apps", label: "App Rules & Exclusions", icon: AppWindow, shortcut: "⌘5" },
    { id: "shortcuts", label: "Shortcuts & Hotkeys", icon: Keyboard, shortcut: "⌘6" },
  ];

  return (
    <aside className="w-64 bg-[#151419]/90 border-r border-[#DA7756]/15 flex flex-col justify-between h-screen p-5 select-none relative z-20 backdrop-blur-2xl">
      <div>
        {/* Brand Header */}
        <div className="flex items-center gap-3.5 px-2 py-3 mb-6 group cursor-pointer">
          <div className="relative">
            <div className="w-10 h-10 rounded-2xl bg-gradient-to-tr from-[#C86544] via-[#DA7756] to-[#F59E0B] flex items-center justify-center shadow-lg shadow-[#DA7756]/25 group-hover:scale-105 transition-transform duration-300">
              <Zap className="w-5 h-5 text-white fill-white/20" />
            </div>
            <div className="absolute -inset-1 bg-[#DA7756]/20 rounded-2xl blur-sm -z-10 group-hover:bg-[#DA7756]/35 transition-all" />
          </div>

          <div>
            <div className="flex items-center gap-1.5">
              <h1 className="font-extrabold text-base tracking-tight text-[#FAF8F5] font-['Plus_Jakarta_Sans']">
                KeyMind
              </h1>
              <span className="px-1.5 py-0.5 rounded text-[9px] font-bold bg-[#DA7756]/20 text-[#DA7756] border border-[#DA7756]/30 tracking-wider uppercase">
                Pro
              </span>
            </div>
            <p className="text-[11px] text-[#A1A0AB] font-medium">Control Center v1.0</p>
          </div>
        </div>

        {/* Navigation Section */}
        <div className="space-y-1">
          <div className="px-3 mb-2 text-[10px] font-extrabold uppercase tracking-wider text-[#71707C] font-mono">
            Product Features
          </div>

          {tabs.map((tab) => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as TabType)}
                className={`w-full flex items-center justify-between px-3.5 py-2.5 rounded-xl text-xs font-semibold transition-all duration-200 group relative active:scale-[0.98] ${
                  isActive
                    ? "bg-[#DA7756]/15 text-[#FAF8F5] border border-[#DA7756]/35 shadow-md shadow-[#DA7756]/10"
                    : "text-[#A1A0AB] hover:text-[#FAF8F5] hover:bg-white/[0.04]"
                }`}
              >
                <div className="flex items-center gap-3">
                  <div
                    className={`p-1.5 rounded-lg transition-colors ${
                      isActive
                        ? "bg-[#DA7756] text-white shadow-sm shadow-[#DA7756]/50"
                        : "bg-zinc-800/50 text-[#A1A0AB] group-hover:text-[#FAF8F5] group-hover:bg-zinc-800"
                    }`}
                  >
                    <Icon className="w-3.5 h-3.5" />
                  </div>
                  <span className="tracking-tight">{tab.label}</span>
                </div>

                <div className="flex items-center gap-1.5">
                  {tab.badge && (
                    <span className="text-[9px] font-extrabold px-1.5 py-0.5 rounded-full bg-[#DA7756]/20 text-[#DA7756] border border-[#DA7756]/30">
                      {tab.badge}
                    </span>
                  )}
                </div>

                {isActive && (
                  <div className="absolute left-0 top-2 bottom-2 w-1 bg-[#DA7756] rounded-r-full shadow-sm shadow-[#DA7756]" />
                )}
              </button>
            );
          })}
        </div>

        {/* System Settings Divider */}
        <div className="pt-4 space-y-1">
          <div className="px-3 mb-2 text-[10px] font-extrabold uppercase tracking-wider text-[#71707C] font-mono">
            Preferences
          </div>

          <button
            onClick={() => setActiveTab("settings")}
            className={`w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-semibold transition-all duration-200 active:scale-[0.98] ${
              activeTab === "settings"
                ? "bg-[#DA7756]/15 text-[#FAF8F5] border border-[#DA7756]/35"
                : "text-[#A1A0AB] hover:text-[#FAF8F5] hover:bg-white/[0.04]"
            }`}
          >
            <div className="p-1.5 bg-zinc-800/50 rounded-lg text-[#A1A0AB]">
              <Settings className="w-3.5 h-3.5" />
            </div>
            <span>Settings & Preferences</span>
          </button>
        </div>
      </div>

      {/* Engine Status Card Footer */}
      <div className="p-3.5 bg-gradient-to-b from-[#1C1B22]/80 to-[#17161C]/90 rounded-2xl border border-[#DA7756]/15 shadow-lg backdrop-blur-md">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2.5">
            <div className="relative">
              <span
                className={`w-2.5 h-2.5 rounded-full block ${
                  engineRunning ? "bg-emerald-400 animate-pulse-glow" : "bg-rose-500"
                }`}
              />
            </div>
            <div>
              <p className="text-xs font-bold text-[#FAF8F5] tracking-tight">
                {engineRunning ? "Engine Active" : "Engine Offline"}
              </p>
              <p className="text-[10px] text-[#A1A0AB]">Local Interceptor</p>
            </div>
          </div>

          <div className="p-1.5 bg-[#DA7756]/10 rounded-lg border border-[#DA7756]/20 text-[#DA7756]">
            <Activity className="w-3.5 h-3.5 animate-pulse" />
          </div>
        </div>
      </div>
    </aside>
  );
};
