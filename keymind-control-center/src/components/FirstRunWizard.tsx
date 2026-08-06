import React, { useState, useEffect } from "react";
import { Check, ArrowLeft, ShieldCheck, Zap, Briefcase, Sliders } from "lucide-react";
import { invoke } from "@tauri-apps/api/tauri";

interface FirstRunWizardProps {
  onComplete: () => void;
  onCheckAccessibility: () => Promise<boolean>;
  onSaveApiKey?: (key: string) => Promise<boolean>;
  onSaveAiKeys?: (groqKey: string, cerebrasKey: string) => Promise<{ groq_valid: boolean; cerebras_valid: boolean }>;
  onInstallLaunchAgent?: () => Promise<boolean>;
}

export const FirstRunWizard: React.FC<FirstRunWizardProps> = ({
  onComplete,
  onCheckAccessibility,
}) => {
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [granted, setGranted] = useState(false);
  const [groqKey, setGroqKey] = useState("");
  const [cerebrasKey, setCerebrasKey] = useState("");
  const [preset, setPreset] = useState<"developer" | "executive" | "minimalist">("developer");
  const timeoutRef = React.useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    let isCancelled = false;

    if (step === 1 && !granted) {
      const poll = async () => {
        if (isCancelled) return;
        const ok = await onCheckAccessibility();
        if (ok) {
          if (!isCancelled) {
            setGranted(true);
            timeoutRef.current = setTimeout(() => setStep(2), 600);
          }
        } else {
          if (!isCancelled) {
            timeoutRef.current = setTimeout(poll, 600);
          }
        }
      };
      poll();
      return () => {
        isCancelled = true;
        if (timeoutRef.current) clearTimeout(timeoutRef.current);
      };
    }
  }, [step, granted, onCheckAccessibility]);

  return (
    <div className="fixed inset-0 bg-[#0D0D0E] flex flex-col items-center justify-center p-6 select-none z-50 animate-fade-in font-sans text-[#EDEDED]">
      {/* Top Header Step Indicator */}
      <div className="flex items-center gap-2 mb-10">
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 1 ? "bg-[#6366F1]" : "bg-[#28282D]"}`} />
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 2 ? "bg-[#6366F1]" : "bg-[#28282D]"}`} />
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 3 ? "bg-[#6366F1]" : "bg-[#28282D]"}`} />
      </div>

      <div className="max-w-[500px] w-full text-center space-y-6 bg-[#161618] border border-[rgba(255,255,255,0.08)] p-8 rounded-[16px] shadow-2xl animate-pop-in relative">
        {/* Step 1: System Permissions */}
        {step === 1 && (
          <div className="space-y-5 animate-fade-in">
            <div className="w-12 h-12 rounded-full bg-[rgba(99,102,241,0.12)] border border-[rgba(99,102,241,0.2)] flex items-center justify-center mx-auto text-[#6366F1]">
              <ShieldCheck className="w-6 h-6" />
            </div>

            <h1 className="text-[20px] font-semibold text-[#EDEDED] tracking-tight">
              Keyboard Interceptor Setup
            </h1>
            <p className="text-[14px] text-[#8F8F96] leading-relaxed">
              KeyStroke runs locally on your computer to provide system-wide autocorrect and AI typing intelligence.
            </p>

            <div className="pt-3 space-y-3">
              <button
                onClick={async () => {
                  setGranted(true);
                  try {
                    await invoke("open_accessibility_settings");
                  } catch (e) {}
                  setStep(2);
                }}
                className="w-full py-2.5 bg-[#6366F1] hover:bg-[#4F46E5] text-white text-[14px] font-medium rounded-[8px] transition cursor-pointer shadow-sm"
              >
                Enable Keyboard Hook
              </button>

              <button
                onClick={() => setStep(2)}
                className="w-full py-2 bg-transparent text-[13px] text-[#8F8F96] hover:text-[#EDEDED] transition cursor-pointer"
              >
                Continue to Setup →
              </button>
            </div>
          </div>
        )}

        {/* Step 2: AI Provider Keys (Optional) */}
        {step === 2 && (
          <div className="space-y-5 animate-fade-in text-left">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setStep(1)}
                className="flex items-center gap-1 text-[13px] text-[#8F8F96] hover:text-[#EDEDED] cursor-pointer"
              >
                <ArrowLeft className="w-4 h-4" /> Back
              </button>
              <span className="text-[12px] font-mono text-[#6366F1]">STEP 2 OF 3</span>
            </div>

            <h1 className="text-[20px] font-semibold text-[#EDEDED] text-center tracking-tight">
              Connect AI Cloud Keys (Optional)
            </h1>
            <p className="text-[13px] text-[#8F8F96] leading-relaxed text-center">
              Local autocorrect & grammar run 100% offline. AI Copilot tone rewrites require Groq or Cerebras API keys.
            </p>

            <div className="space-y-3.5 pt-2">
              <div>
                <label className="block text-[12px] font-medium text-[#EDEDED] mb-1">Groq API Key</label>
                <input
                  type="password"
                  value={groqKey}
                  onChange={(e) => setGroqKey(e.target.value)}
                  placeholder="gsk_..."
                  className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#EDEDED] focus:outline-none focus:border-[#6366F1]"
                />
              </div>

              <div>
                <label className="block text-[12px] font-medium text-[#EDEDED] mb-1">Cerebras API Key</label>
                <input
                  type="password"
                  value={cerebrasKey}
                  onChange={(e) => setCerebrasKey(e.target.value)}
                  placeholder="csk_..."
                  className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#EDEDED] focus:outline-none focus:border-[#6366F1]"
                />
              </div>
            </div>

            <div className="flex items-center justify-between pt-4">
              <button
                onClick={() => setStep(3)}
                className="text-[13px] text-[#8F8F96] hover:text-[#EDEDED] cursor-pointer"
              >
                Skip for now
              </button>
              <button
                onClick={() => setStep(3)}
                className="px-5 py-2.5 bg-[#6366F1] hover:bg-[#4F46E5] text-white text-[13px] font-medium rounded-[8px] transition cursor-pointer"
              >
                Continue →
              </button>
            </div>
          </div>
        )}

        {/* Step 3: Typing Preset Selector */}
        {step === 3 && (
          <div className="space-y-5 animate-fade-in text-left">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setStep(2)}
                className="flex items-center gap-1 text-[13px] text-[#8F8F96] hover:text-[#EDEDED] cursor-pointer"
              >
                <ArrowLeft className="w-4 h-4" /> Back
              </button>
              <span className="text-[12px] font-mono text-[#6366F1]">STEP 3 OF 3</span>
            </div>

            <h1 className="text-[20px] font-semibold text-[#EDEDED] text-center tracking-tight">
              Select Typing Mode
            </h1>

            <div className="space-y-3">
              {[
                {
                  id: "developer",
                  title: "Power Developer",
                  desc: "Aggressive autocorrect, fast trigram predictions, tech whitelist active.",
                  icon: Zap,
                },
                {
                  id: "executive",
                  title: "Business & Executive",
                  desc: "Strict nlprule grammar checks, polite AI rewrites, email templates.",
                  icon: Briefcase,
                },
                {
                  id: "minimalist",
                  title: "Minimalist",
                  desc: "Next-word prediction only. Zero auto-modifications.",
                  icon: Sliders,
                },
              ].map((card) => {
                const Icon = card.icon;
                const isSelected = preset === card.id;
                return (
                  <div
                    key={card.id}
                    onClick={() => setPreset(card.id as any)}
                    className={`p-4 bg-[#1F1F23] rounded-[10px] border cursor-pointer transition ${
                      isSelected
                        ? "border-[#6366F1] bg-[#28282D]"
                        : "border-[rgba(255,255,255,0.08)] hover:border-[rgba(255,255,255,0.16)]"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2.5">
                        <Icon className={`w-4 h-4 ${isSelected ? "text-[#6366F1]" : "text-[#8F8F96]"}`} />
                        <span className="font-semibold text-[14px] text-[#EDEDED]">{card.title}</span>
                      </div>
                      <div className={`w-4 h-4 rounded-full border flex items-center justify-center ${isSelected ? "border-[#6366F1] bg-[#6366F1]" : "border-[rgba(255,255,255,0.2)]"}`}>
                        {isSelected && <Check className="w-3 h-3 text-white" />}
                      </div>
                    </div>
                    <p className="text-[12px] text-[#8F8F96] mt-1.5 leading-normal">{card.desc}</p>
                  </div>
                );
              })}
            </div>

            <div className="flex justify-end pt-4">
              <button
                onClick={async () => {
                  try {
                    if (groqKey || cerebrasKey) {
                      await invoke("save_ai_provider_keys", {
                        groqKey: groqKey || null,
                        cerebrasKey: cerebrasKey || null,
                      });
                    }
                    await invoke("set_typing_preset", { preset });
                  } catch (e) {}
                  onComplete();
                }}
                className="w-full py-2.5 bg-[#6366F1] hover:bg-[#4F46E5] text-white text-[14px] font-medium rounded-[8px] transition cursor-pointer shadow-sm"
              >
                Finish Setup & Launch KeyStroke →
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
