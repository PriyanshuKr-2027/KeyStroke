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
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  setActiveTab,
  engineRunning,
  onToggleEngine,
  onOpenSettings,
  onOpenWizard,
}) => {
  const navItems = [
    { id: "dashboard", label: "Home", icon: Home },
    { id: "memory", label: "My Dictionary", icon: Book },
    { id: "variables", label: "Text Shortcuts", icon: Code2 },
    { id: "grammar", label: "Grammar & Auto-Fix", icon: CheckCheck },
    { id: "apps", label: "App Rules", icon: AppWindow },
    { id: "shortcuts", label: "Keybindings", icon: Keyboard },
  ];

  return (
    <aside className="w-[220px] min-w-[220px] max-w-[220px] h-screen bg-[#F3EFEA] border-r border-[#E8E4DC] flex flex-col justify-between select-none relative z-20 font-sans">
      <div>
        {/* Header */}
        <div data-tauri-drag-region className="h-[60px] px-5 flex items-center justify-between cursor-default">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded-[6px] bg-[#DA7756] flex items-center justify-center font-mono text-[11px] font-bold text-white shadow-sm">
              K
            </div>
            <span className="font-semibold text-[15px] text-[#1E1E1E] tracking-tight">
              KeyStroke
            </span>
          </div>

          <button
            onClick={onToggleEngine}
            title={engineRunning ? "Interceptor Active — Click to pause" : "Interceptor Paused — Click to resume"}
            className="flex items-center gap-1.5 px-2 py-1 rounded-full bg-[#EAE4DC] hover:bg-[#E2DDD3] border border-[#DDD7CD] transition cursor-pointer"
          >
            <span className={`w-2 h-2 rounded-full ${engineRunning ? "bg-[#22C55E]" : "bg-[#F59E0B]"}`} />
            <span className="text-[11px] font-mono text-[#6B6963] font-medium">
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
                    ? "bg-white text-[#1E1E1E] shadow-sm border border-[#E8E4DC]"
                    : "text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E]"
                }`}
              >
                <Icon
                  className={`w-[16px] h-[16px] transition ${
                    isActive ? "text-[#DA7756]" : "text-[#8F8E88]"
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
        <div className="my-2 border-t border-[#E8E4DC]" />

        <button
          onClick={onOpenSettings}
          className="w-full h-[38px] px-3 rounded-[8px] text-[13px] font-medium flex items-center gap-2.5 text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E] transition cursor-pointer"
        >
          <Settings className="w-[16px] h-[16px] text-[#8F8E88]" strokeWidth={2} />
          <span>Settings</span>
        </button>

        <button
          onClick={() => (onOpenWizard ? onOpenWizard() : undefined)}
          className="w-full h-[38px] px-3 rounded-[8px] text-[13px] font-medium flex items-center gap-2.5 text-[#6B6963] hover:bg-[#EAE4DC] hover:text-[#1E1E1E] transition cursor-pointer"
        >
          <HelpCircle className="w-[16px] h-[16px] text-[#8F8E88]" strokeWidth={2} />
          <span>Setup Wizard</span>
        </button>
      </div>
    </aside>
  );
};
