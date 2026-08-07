import React from "react";
import {
  Home,
  Book,
  Code2,
  CheckCheck,
  AppWindow,
  Keyboard,
  Settings,
  HelpCircle,
  Sun,
  Moon,
} from "lucide-react";

export type TabType =
  | "dashboard"
  | "memory"
  | "variables"
  | "grammar"
  | "apps"
  | "shortcuts";

interface SidebarProps {
  activeTab: TabType;
  setActiveTab: (tab: TabType) => void;
  engineRunning: boolean;
  onToggleEngine: () => void;
  onOpenSettings: () => void;
  onOpenWizard?: () => void;
  theme?: "light" | "dark";
  onToggleTheme?: () => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  setActiveTab,
  engineRunning,
  onToggleEngine,
  onOpenSettings,
  onOpenWizard,
  theme = "light",
  onToggleTheme,
}) => {
  const navItems = [
    { id: "dashboard", label: "Home", icon: Home },
    { id: "memory", label: "My Dictionary", icon: Book },
    { id: "variables", label: "Text Shortcuts", icon: Code2 },
    { id: "grammar", label: "Grammar & Auto-Fix", icon: CheckCheck },
    { id: "apps", label: "App Rules", icon: AppWindow },
    { id: "shortcuts", label: "Keybindings", icon: Keyboard },
  ];

  const isDark = theme === "dark";

  return (
    <aside
      className={`w-[220px] min-w-[220px] max-w-[220px] h-screen border-r flex flex-col justify-between select-none relative z-20 font-sans transition-colors duration-150 ${
        isDark
          ? "bg-[#22201D] border-[#383430] text-[#ECE9E3]"
          : "bg-[#F3EFEA] border-[#E8E4DC] text-[#1E1E1E]"
      }`}
    >
      <div>
        {/* Header */}
        <div data-tauri-drag-region className="h-[60px] px-5 flex items-center justify-between cursor-default">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded-[6px] bg-[#DA7756] flex items-center justify-center font-mono text-[11px] font-bold text-white shadow-sm">
              K
            </div>
            <span className={`font-semibold text-[15px] tracking-tight ${isDark ? "text-[#ECE9E3]" : "text-[#1E1E1E]"}`}>
              KeyStroke
            </span>
          </div>

          <button
            onClick={onToggleEngine}
            title={engineRunning ? "Interceptor Active — Click to pause" : "Interceptor Paused — Click to resume"}
            className={`flex items-center gap-1.5 px-2 py-1 rounded-full border transition cursor-pointer ${
              isDark
                ? "bg-[#2D2A26] hover:bg-[#383430] border-[#423E39]"
                : "bg-[#EAE4DC] hover:bg-[#E2DDD3] border-[#DDD7CD]"
            }`}
          >
            <span className={`w-2 h-2 rounded-full ${engineRunning ? "bg-[#22C55E]" : "bg-[#F59E0B]"}`} />
            <span className={`text-[11px] font-mono font-medium ${isDark ? "text-[#A39E93]" : "text-[#6B6963]"}`}>
              {engineRunning ? "ON" : "OFF"}
            </span>
          </button>
        </div>

        {/* Navigation */}
        <div className="px-3 space-y-1 mt-2">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id as TabType)}
                className={`w-full h-[38px] px-3 rounded-[8px] text-[13px] font-medium flex items-center gap-2.5 transition cursor-pointer ${
                  isActive
                    ? isDark
                      ? "bg-[#282522] text-[#ECE9E3] shadow-sm border border-[#383430]"
                      : "bg-white text-[#1E1E1E] shadow-sm border border-[#E8E4DC]"
                    : isDark
                    ? "text-[#A39E93] hover:bg-[#2D2A26] hover:text-[#ECE9E3]"
                    : "text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E]"
                }`}
              >
                <Icon
                  className={`w-[16px] h-[16px] transition ${
                    isActive
                      ? "text-[#DA7756]"
                      : isDark
                      ? "text-[#78736B]"
                      : "text-[#8F8E88]"
                  }`}
                  strokeWidth={2}
                />
                <span className="truncate">{item.label}</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Bottom Pinned Actions */}
      <div className="px-3 pb-4 space-y-1">
        {/* Quick Theme Switcher Button in Sidebar */}
        {onToggleTheme && (
          <button
            onClick={onToggleTheme}
            className={`w-full h-[36px] px-3 rounded-[8px] text-[12px] font-medium flex items-center justify-between transition cursor-pointer mb-1 border ${
              isDark
                ? "bg-[#282522] border-[#383430] text-[#ECE9E3] hover:bg-[#33302B]"
                : "bg-white border-[#E8E4DC] text-[#1E1E1E] hover:bg-[#F3EFEA]"
            }`}
          >
            <span className="flex items-center gap-2">
              {isDark ? <Moon className="w-3.5 h-3.5 text-[#E07A5F]" /> : <Sun className="w-3.5 h-3.5 text-[#DA7756]" />}
              <span>{isDark ? "Claude Dark" : "Claude Light"}</span>
            </span>
            <span className="text-[10px] font-mono opacity-60 uppercase">Switch</span>
          </button>
        )}

        <div className={`my-2 border-t ${isDark ? "border-[#383430]" : "border-[#E8E4DC]"}`} />

        <button
          onClick={onOpenSettings}
          className={`w-full h-[38px] px-3 rounded-[8px] text-[13px] font-medium flex items-center gap-2.5 transition cursor-pointer ${
            isDark
              ? "text-[#A39E93] hover:bg-[#2D2A26] hover:text-[#ECE9E3]"
              : "text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E]"
          }`}
        >
          <Settings className={`w-[16px] h-[16px] ${isDark ? "text-[#78736B]" : "text-[#8F8E88]"}`} strokeWidth={2} />
          <span>Settings</span>
        </button>

        <button
          onClick={() => (onOpenWizard ? onOpenWizard() : undefined)}
          className={`w-full h-[38px] px-3 rounded-[8px] text-[13px] font-medium flex items-center gap-2.5 transition cursor-pointer ${
            isDark
              ? "text-[#A39E93] hover:bg-[#2D2A26] hover:text-[#ECE9E3]"
              : "text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E]"
          }`}
        >
          <HelpCircle className={`w-[16px] h-[16px] ${isDark ? "text-[#78736B]" : "text-[#8F8E88]"}`} strokeWidth={2} />
          <span>Setup Wizard</span>
        </button>
      </div>
    </aside>
  );
};
