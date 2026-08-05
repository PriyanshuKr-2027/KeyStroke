import React, { useState, useEffect } from "react";
import { Check } from "lucide-react";

interface FirstRunWizardProps {
  onComplete: () => void;
  onCheckAccessibility: () => Promise<boolean>;
  onSaveApiKey: (key: string) => Promise<boolean>;
  onSaveAiKeys?: (groqKey: string, cerebrasKey: string) => Promise<{ groq_valid: boolean; cerebras_valid: boolean }>;
  onInstallLaunchAgent: () => Promise<boolean>;
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

  // Step 1: Auto-poll permission status
  useEffect(() => {
    if (step === 1 && !granted) {
      const timer = setInterval(async () => {
        const ok = await onCheckAccessibility();
        if (ok) {
          setGranted(true);
          clearInterval(timer);
          setTimeout(() => setStep(2), 800);
        }
      }, 500);
      return () => clearInterval(timer);
    }
  }, [step, granted, onCheckAccessibility]);

  return (
    <div className="fixed inset-0 bg-[#FFFFFF] flex flex-col items-center justify-center p-6 select-none z-50 animate-fade-in font-sans">
      {/* Top 3-dot progress bar */}
      <div className="flex items-center gap-2 mb-10">
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 1 ? "bg-[#111111]" : "bg-[#D1D5DB]"}`} />
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 2 ? "bg-[#111111]" : "bg-[#D1D5DB]"}`} />
        <div className={`w-2.5 h-2.5 rounded-full ${step >= 3 ? "bg-[#111111]" : "bg-[#D1D5DB]"}`} />
      </div>

      <div className="max-w-[480px] w-full text-center space-y-6">
        {/* Step 1: Permissions */}
        {step === 1 && (
          <div className="space-y-5 animate-fade-in">
            <h1 className="text-[22px] font-semibold text-[#111111]">
              KeyStroke needs one permission to get started.
            </h1>
            <p className="text-[15px] text-[#6B6B6B] leading-relaxed">
              To type intelligently across all your apps, KeyStroke needs low-level keyboard access. This runs entirely on your device — nothing leaves your machine.
            </p>

            <div className="pt-3 space-y-3">
              <button
                onClick={() => setGranted(true)}
                className="px-5 py-2.5 bg-[#111111] text-[#FFFFFF] text-[14px] font-medium rounded-[8px] hover:bg-[#333333] transition cursor-pointer"
              >
                Open Accessibility Settings
              </button>

              <div className="text-[13px] text-[#6B6B6B] flex items-center justify-center gap-2">
                <span className={`w-2 h-2 rounded-full ${granted ? "bg-[#22C55E]" : "bg-[#F59E0B]"}`} />
                <span>{granted ? "Permission Granted" : "Waiting for permission"}</span>
              </div>
            </div>
          </div>
        )}

        {/* Step 2: AI Setup */}
        {step === 2 && (
          <div className="space-y-5 animate-fade-in">
            <h1 className="text-[22px] font-semibold text-[#111111]">
              Connect your AI keys (optional).
            </h1>
            <p className="text-[15px] text-[#6B6B6B] leading-relaxed">
              KeyStroke uses Groq and Cerebras for AI-powered features — tone rewrites, /reply expansions, and the AI Copilot palette. You can skip this and add them later in Settings.
            </p>

            <div className="space-y-3 text-left">
              <div>
                <label className="block text-[13px] font-medium text-[#111111] mb-1">Groq API key</label>
                <input
                  type="password"
                  value={groqKey}
                  onChange={(e) => setGroqKey(e.target.value)}
                  placeholder="gsk_..."
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#111111] focus:outline-none"
                />
              </div>

              <div>
                <label className="block text-[13px] font-medium text-[#111111] mb-1">Cerebras API key</label>
                <input
                  type="password"
                  value={cerebrasKey}
                  onChange={(e) => setCerebrasKey(e.target.value)}
                  placeholder="csk_..."
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#111111] focus:outline-none"
                />
              </div>
            </div>

            <div className="flex items-center justify-between pt-4">
              <button
                onClick={() => setStep(3)}
                className="text-[14px] text-[#6B6B6B] hover:text-[#111111]"
              >
                Skip for now
              </button>
              <button
                onClick={() => setStep(3)}
                className="px-5 py-2.5 bg-[#111111] text-[#FFFFFF] text-[14px] font-medium rounded-[8px] hover:bg-[#333333]"
              >
                Continue →
              </button>
            </div>
          </div>
        )}

        {/* Step 3: Preset */}
        {step === 3 && (
          <div className="space-y-5 animate-fade-in">
            <h1 className="text-[22px] font-semibold text-[#111111]">
              How do you type?
            </h1>

            <div className="space-y-3 text-left">
              {[
                {
                  id: "developer",
                  title: "Power Developer",
                  desc: "Fast triggers, aggressive autocorrect, technical whitelist on.",
                },
                {
                  id: "executive",
                  title: "Business & Executive",
                  desc: "Formal grammar, email templates, polite AI rewrites.",
                },
                {
                  id: "minimalist",
                  title: "Minimalist",
                  desc: "Next-word prediction only. Non-intrusive, no auto-changes.",
                },
              ].map((card) => {
                const isSelected = preset === card.id;
                return (
                  <div
                    key={card.id}
                    onClick={() => setPreset(card.id as any)}
                    className={`p-4 bg-[#F5F5F5] rounded-[12px] border cursor-pointer transition ${
                      isSelected
                        ? "border-[#111111] shadow-sm"
                        : "border-[#EBEBEB] hover:border-[#D1D5DB]"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-semibold text-[15px] text-[#111111]">{card.title}</span>
                      <div className={`w-4 h-4 rounded-full border flex items-center justify-center ${isSelected ? "border-[#111111] bg-[#111111]" : "border-[#D1D5DB]"}`}>
                        {isSelected && <Check className="w-3 h-3 text-[#FFFFFF]" />}
                      </div>
                    </div>
                    <p className="text-[13px] text-[#6B6B6B] mt-1">{card.desc}</p>
                  </div>
                );
              })}
            </div>

            <div className="flex justify-end pt-4">
              <button
                onClick={onComplete}
                className="px-5 py-2.5 bg-[#111111] text-[#FFFFFF] text-[14px] font-medium rounded-[8px] hover:bg-[#333333] cursor-pointer"
              >
                Finish setup →
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
