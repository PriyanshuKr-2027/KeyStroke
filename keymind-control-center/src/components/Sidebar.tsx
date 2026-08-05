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
  onOpenSettings: () => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeTab,
  setActiveTab,
  onOpenSettings,
}) => {
  const navItems = [
    { id: "dashboard", label: "Home", icon: Home },
    { id: "memory", label: "Dictionary", icon: Book },
    { id: "variables", label: "Snippets", icon: Code2 },
    { id: "grammar", label: "Grammar", icon: CheckCheck },
    { id: "apps", label: "App Rules", icon: AppWindow },
    { id: "shortcuts", label: "Shortcuts", icon: Keyboard },
  ];

  return (
    <aside className="w-[200px] min-w-[200px] max-w-[200px] h-screen bg-[#FFFFFF] border-r border-[#EBEBEB] flex flex-col justify-between select-none relative z-20">
      <div>
        {/* Top Header Section — 60px tall, 24px padding */}
        <div className="h-[60px] px-6 flex items-center gap-2">
          <span className="font-sans font-semibold text-[15px] text-[#111111] tracking-tight">
            KeyMind
          </span>
          <span className="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-[#F5F5F5] text-[#6B6B6B] border border-[#EBEBEB] uppercase tracking-wider">
            PRO
          </span>
        </div>

        {/* Navigation Items Section */}
        <div className="px-3 space-y-1 mt-2">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id as TabType)}
                className={`w-full h-[40px] px-3 rounded-[6px] text-[14px] font-normal flex items-center gap-2.5 transition-colors cursor-pointer ${
                  isActive
                    ? "bg-[#F0F0F0] text-[#111111]"
                    : "text-[#6B6B6B] hover:bg-[#F5F5F5] hover:text-[#111111]"
                }`}
              >
                <Icon
                  className={`w-[18px] h-[18px] transition-colors ${
                    isActive ? "text-[#111111]" : "text-[#6B6B6B]"
                  }`}
                  strokeWidth={2}
                />
                <span className="truncate">{item.label}</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Bottom Pinned Section */}
      <div className="px-3 pb-4 space-y-1">
        <div className="my-2 border-t border-[#EBEBEB]" />

        {/* Settings Trigger */}
        <button
          onClick={onOpenSettings}
          className="w-full h-[40px] px-3 rounded-[6px] text-[14px] font-normal flex items-center gap-2.5 text-[#6B6B6B] hover:bg-[#F5F5F5] hover:text-[#111111] transition-colors cursor-pointer"
        >
          <Settings className="w-[18px] h-[18px] text-[#6B6B6B]" strokeWidth={2} />
          <span>Settings</span>
        </button>

        {/* Help Link */}
        <button
          onClick={() => window.open("https://github.com", "_blank")}
          className="w-full h-[40px] px-3 rounded-[6px] text-[14px] font-normal flex items-center gap-2.5 text-[#6B6B6B] hover:bg-[#F5F5F5] hover:text-[#111111] transition-colors cursor-pointer"
        >
          <HelpCircle className="w-[18px] h-[18px] text-[#6B6B6B]" strokeWidth={2} />
          <span>Help</span>
        </button>
      </div>
    </aside>
  );
};
