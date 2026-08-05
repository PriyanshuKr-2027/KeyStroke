import React, { useState } from "react";
import { AppSettings } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { MoreHorizontal, MonitorOff } from "lucide-react";

interface AppsTabProps {
  apps: AppSettings[];
  onUpdateApp: (updated: AppSettings) => void;
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
}

export const AppsTab: React.FC<AppsTabProps> = ({
  apps,
  onUpdateApp,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [showCallout, setShowCallout] = useState(true);
  const [openMenuAppId, setOpenMenuAppId] = useState<string | null>(null);

  const toggleMenu = (bundleId: string) => {
    setOpenMenuAppId((prev) => (prev === bundleId ? null : bundleId));
  };

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="App Detection Unavailable"
          message={errorMessage || "Could not query running applications or retrieve app permissions."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Header */}
      <h1 className="font-sans text-[22px] font-semibold text-[#111111]">
        App Rules
      </h1>

      {/* Callout Card */}
      {showCallout && (
        <CalloutCard
          headline="KeyStroke, everywhere — except where you say."
          body="Disable features for specific apps. Block it entirely in password managers or banking apps."
          chips={[
            { label: "VS Code" },
            { label: "Slack" },
            { label: "1Password" },
            { label: "Chrome" },
          ]}
          ctaLabel="Manage apps"
          onCtaClick={() => {
            try { window.scrollTo({ top: 300, behavior: "smooth" }) } catch (e) {}
          }}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* List Header */}
      <div className="text-[11px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase pt-2">
        DETECTED APPLICATIONS
      </div>

      {/* App List */}
      {isLoading ? (
        <TableRowSkeleton count={4} />
      ) : apps.length > 0 ? (
        <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
          {apps.map((app) => (
            <div
              key={app.app_bundle_id}
              className="h-[56px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors relative"
            >
              {/* App Info */}
              <div className="flex items-center gap-3">
                <div className="w-6 h-6 rounded-md bg-[#F5F5F5] border border-[#EBEBEB] flex items-center justify-center font-bold text-[12px] text-[#111111]">
                  {app.app_name.charAt(0)}
                </div>
                <span className="font-sans text-[14px] font-medium text-[#111111]">
                  {app.app_name}
                </span>
              </div>

              {/* Feature Controls / Blocked Badge */}
              <div className="flex items-center gap-5">
                {app.is_blocked ? (
                  <span className="px-2.5 py-1 bg-[#FEE2E2] text-[#EF4444] text-[12px] font-medium rounded-[6px]">
                    BLOCKED
                  </span>
                ) : (
                  <div className="flex items-center gap-4 text-[13px] text-[#6B6B6B]">
                    {/* Autocorrect Toggle */}
                    <div className="flex items-center gap-1.5">
                      <span>Autocorrect</span>
                      <button
                        type="button"
                        onClick={() =>
                          onUpdateApp({
                            ...app,
                            autocorrect_enabled: !app.autocorrect_enabled,
                          })
                        }
                        className={`w-[32px] h-[18px] rounded-full p-[2px] transition-colors cursor-pointer flex items-center ${
                          app.autocorrect_enabled ? "bg-[#22C55E]" : "bg-[#D1D5DB]"
                        }`}
                      >
                        <div
                          className={`w-[14px] h-[14px] rounded-full bg-[#FFFFFF] shadow-sm transform transition-transform ${
                            app.autocorrect_enabled ? "translate-x-[14px]" : "translate-x-0"
                          }`}
                        />
                      </button>
                    </div>

                    {/* Grammar Toggle */}
                    <div className="flex items-center gap-1.5">
                      <span>Grammar</span>
                      <button
                        type="button"
                        onClick={() =>
                          onUpdateApp({
                            ...app,
                            grammar_enabled: !app.grammar_enabled,
                          })
                        }
                        className={`w-[32px] h-[18px] rounded-full p-[2px] transition-colors cursor-pointer flex items-center ${
                          app.grammar_enabled ? "bg-[#22C55E]" : "bg-[#D1D5DB]"
                        }`}
                      >
                        <div
                          className={`w-[14px] h-[14px] rounded-full bg-[#FFFFFF] shadow-sm transform transition-transform ${
                            app.grammar_enabled ? "translate-x-[14px]" : "translate-x-0"
                          }`}
                        />
                      </button>
                    </div>

                    {/* AI Toggle */}
                    <div className="flex items-center gap-1.5">
                      <span>AI</span>
                      <button
                        type="button"
                        onClick={() =>
                          onUpdateApp({
                            ...app,
                            ai_copilot_enabled: !app.ai_copilot_enabled,
                          })
                        }
                        className={`w-[32px] h-[18px] rounded-full p-[2px] transition-colors cursor-pointer flex items-center ${
                          app.ai_copilot_enabled ? "bg-[#22C55E]" : "bg-[#D1D5DB]"
                        }`}
                      >
                        <div
                          className={`w-[14px] h-[14px] rounded-full bg-[#FFFFFF] shadow-sm transform transition-transform ${
                            app.ai_copilot_enabled ? "translate-x-[14px]" : "translate-x-0"
                          }`}
                        />
                      </button>
                    </div>
                  </div>
                )}

                {/* Action Dropdown Menu */}
                <button
                  onClick={() => toggleMenu(app.app_bundle_id)}
                  className="text-[#AAAAAA] hover:text-[#111111] p-1 cursor-pointer transition"
                >
                  <MoreHorizontal className="w-4 h-4" />
                </button>

                {openMenuAppId === app.app_bundle_id && (
                  <div className="absolute right-0 top-12 w-44 bg-[#FFFFFF] border border-[#EBEBEB] rounded-[8px] shadow-lg py-1 z-30 animate-fade-in text-[13px]">
                    <button
                      onClick={() => {
                        onUpdateApp({ ...app, is_blocked: !app.is_blocked });
                        setOpenMenuAppId(null);
                      }}
                      className="w-full px-3 py-1.5 text-left text-[#EF4444] hover:bg-[#F5F5F5] cursor-pointer"
                    >
                      {app.is_blocked ? "Unblock app" : "Block app entirely"}
                    </button>
                    <button
                      onClick={() => {
                        onUpdateApp({
                          ...app,
                          autocorrect_enabled: true,
                          grammar_enabled: true,
                          ai_copilot_enabled: true,
                          is_blocked: false,
                        });
                        setOpenMenuAppId(null);
                      }}
                      className="w-full px-3 py-1.5 text-left text-[#111111] hover:bg-[#F5F5F5] cursor-pointer"
                    >
                      Reset to default
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      ) : (
        <EmptyState
          icon={MonitorOff}
          title="No applications detected"
          description="KeyStroke automatically detects desktop applications as you switch between windows. Open any text application to configure custom rules."
          actionLabel="Refresh Applications"
          onAction={onRetry}
        />
      )}
    </div>
  );
};
