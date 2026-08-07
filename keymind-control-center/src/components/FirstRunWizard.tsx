import React, { useState, useEffect } from "react";
import { Check, ArrowLeft, ShieldCheck, User, Zap, Briefcase, Sliders } from "lucide-react";
import { invoke } from "@tauri-apps/api/tauri";

interface FirstRunWizardProps {
  onComplete: () => void;
  onCheckAccessibility: () => Promise<boolean>;
}

export const FirstRunWizard: React.FC<FirstRunWizardProps> = ({
  onComplete,
  onCheckAccessibility,
}) => {
  const [step, setStep] = useState<1 | 2 | 3 | 4>(1);
  const [granted, setGranted] = useState(false);

  // Profile state
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [dob, setDob] = useState("");

  // AI keys
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

  const handleFinishSetup = async () => {
    try {
      // 1. Save Profile
      if (firstName.trim() || lastName.trim() || email.trim() || dob.trim()) {
        await invoke("save_profile", {
          firstName: firstName.trim(),
          lastName: lastName.trim(),
          email: email.trim(),
          dateOfBirth: dob.trim() || null,
        });

        // 2. Auto-generate user shortcuts
        const fullName = `${firstName.trim()} ${lastName.trim()}`.trim();
        if (fullName) {
          await invoke("upsert_variable", {
            v: { key: "name", var_type: "static", value: fullName, use_count: 0 },
          });
        }
        if (email.trim()) {
          await invoke("upsert_variable", {
            v: { key: "email", var_type: "static", value: email.trim(), use_count: 0 },
          });
        }
        if (dob.trim()) {
          await invoke("upsert_variable", {
            v: { key: "dob", var_type: "static", value: dob.trim(), use_count: 0 },
          });
        }
        if (firstName.trim()) {
          await invoke("upsert_variable", {
            v: { key: "firstname", var_type: "static", value: firstName.trim(), use_count: 0 },
          });
        }
      }

      // 3. Save AI Keys if entered
      if (groqKey.trim() || cerebrasKey.trim()) {
        await invoke("save_ai_provider_keys", {
          groqKey: groqKey.trim() || null,
          cerebrasKey: cerebrasKey.trim() || null,
        });
      }

      await invoke("set_typing_preset", { preset });

      // 4. Install Windows autostart agent by default
      await invoke("install_launch_agent");
    } catch (e) {
      console.error("Setup completion error:", e);
    }
    onComplete();
  };

  return (
    <div className="fixed inset-0 bg-black/40 backdrop-blur-sm flex flex-col items-center justify-center p-6 select-none z-50 animate-fade-in font-sans text-[#1E1E1E]">
      {/* Top Header Step Indicator */}
      <div className="flex items-center gap-2 mb-6">
        {[1, 2, 3, 4].map((s) => (
          <div
            key={s}
            className={`w-2.5 h-2.5 rounded-full transition-colors ${
              step >= s ? "bg-[#DA7756]" : "bg-[#E8E4DC]"
            }`}
          />
        ))}
      </div>

      <div className="max-w-[520px] w-full text-center space-y-6 bg-white border border-[#E8E4DC] p-8 rounded-[16px] shadow-2xl animate-pop-in relative">
        {/* Step 1: System Permissions */}
        {step === 1 && (
          <div className="space-y-5 animate-fade-in">
            <div className="w-12 h-12 rounded-full bg-[rgba(218,119,86,0.12)] border border-[rgba(218,119,86,0.2)] flex items-center justify-center mx-auto text-[#DA7756]">
              <ShieldCheck className="w-6 h-6" />
            </div>

            <h1 className="text-[20px] font-semibold text-[#1E1E1E] tracking-tight">
              Keyboard Interceptor Setup
            </h1>
            <p className="text-[14px] text-[#6B6963] leading-relaxed">
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
                className="w-full py-2.5 bg-[#DA7756] hover:bg-[#C26242] text-white text-[14px] font-medium rounded-[8px] transition cursor-pointer shadow-sm"
              >
                Enable Keyboard Hook
              </button>

              <button
                onClick={() => setStep(2)}
                className="w-full py-2 bg-transparent text-[13px] text-[#6B6963] hover:text-[#1E1E1E] transition cursor-pointer"
              >
                Continue to Profile →
              </button>
            </div>
          </div>
        )}

        {/* Step 2: Personal Profile & Auto-Shortcuts */}
        {step === 2 && (
          <div className="space-y-5 animate-fade-in text-left">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setStep(1)}
                className="flex items-center gap-1 text-[13px] text-[#6B6963] hover:text-[#1E1E1E] cursor-pointer"
              >
                <ArrowLeft className="w-4 h-4" /> Back
              </button>
              <span className="text-[12px] font-mono text-[#DA7756] font-semibold">STEP 2 OF 4</span>
            </div>

            <div className="flex items-center gap-3">
              <div className="w-9 h-9 rounded-full bg-[rgba(218,119,86,0.12)] border border-[rgba(218,119,86,0.2)] flex items-center justify-center text-[#DA7756]">
                <User className="w-4 h-4" />
              </div>
              <div>
                <h1 className="text-[18px] font-semibold text-[#1E1E1E] tracking-tight">Your Profile</h1>
                <p className="text-[12px] text-[#6B6963]">Creates instant shortcuts (/name, /email, /dob) for you</p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3 pt-1">
              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">First Name</label>
                <input
                  type="text"
                  value={firstName}
                  onChange={(e) => setFirstName(e.target.value)}
                  placeholder="John"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[13px] text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>

              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">Last Name</label>
                <input
                  type="text"
                  value={lastName}
                  onChange={(e) => setLastName(e.target.value)}
                  placeholder="Doe"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[13px] text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>
            </div>

            <div className="space-y-3">
              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">Email Address</label>
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="john@example.com"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[13px] text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>

              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">Date of Birth</label>
                <input
                  type="date"
                  value={dob}
                  onChange={(e) => setDob(e.target.value)}
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3 py-2 text-[13px] text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>
            </div>

            <div className="flex items-center justify-between pt-2">
              <button
                onClick={() => setStep(3)}
                className="text-[13px] text-[#6B6963] hover:text-[#1E1E1E] cursor-pointer"
              >
                Skip for now
              </button>
              <button
                onClick={() => setStep(3)}
                className="px-5 py-2 bg-[#DA7756] hover:bg-[#C26242] text-white text-[13px] font-medium rounded-[8px] transition cursor-pointer"
              >
                Next →
              </button>
            </div>
          </div>
        )}

        {/* Step 3: AI Provider Keys (Optional) */}
        {step === 3 && (
          <div className="space-y-5 animate-fade-in text-left">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setStep(2)}
                className="flex items-center gap-1 text-[13px] text-[#6B6963] hover:text-[#1E1E1E] cursor-pointer"
              >
                <ArrowLeft className="w-4 h-4" /> Back
              </button>
              <span className="text-[12px] font-mono text-[#DA7756] font-semibold">STEP 3 OF 4</span>
            </div>

            <h1 className="text-[20px] font-semibold text-[#1E1E1E] text-center tracking-tight">
              Connect AI API Keys (Optional)
            </h1>
            <p className="text-[13px] text-[#6B6963] leading-relaxed text-center">
              Local autocorrect & grammar run 100% offline. AI Copilot prompts require Groq or Cerebras API keys.
            </p>

            <div className="space-y-3.5 pt-2">
              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">Groq API Key</label>
                <input
                  type="password"
                  value={groqKey}
                  onChange={(e) => setGroqKey(e.target.value)}
                  placeholder="gsk_..."
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>

              <div>
                <label className="block text-[12px] font-medium text-[#1E1E1E] mb-1">Cerebras API Key</label>
                <input
                  type="password"
                  value={cerebrasKey}
                  onChange={(e) => setCerebrasKey(e.target.value)}
                  placeholder="csk_..."
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[13px] font-mono text-[#1E1E1E] focus:outline-none focus:border-[#DA7756]"
                />
              </div>
            </div>

            <div className="flex items-center justify-between pt-4">
              <button
                onClick={() => setStep(4)}
                className="text-[13px] text-[#6B6963] hover:text-[#1E1E1E] cursor-pointer"
              >
                Skip for now
              </button>
              <button
                onClick={() => setStep(4)}
                className="px-5 py-2 bg-[#DA7756] hover:bg-[#C26242] text-white text-[13px] font-medium rounded-[8px] transition cursor-pointer"
              >
                Next →
              </button>
            </div>
          </div>
        )}

        {/* Step 4: Typing Mode Selector */}
        {step === 4 && (
          <div className="space-y-5 animate-fade-in text-left">
            <div className="flex items-center justify-between">
              <button
                onClick={() => setStep(3)}
                className="flex items-center gap-1 text-[13px] text-[#6B6963] hover:text-[#1E1E1E] cursor-pointer"
              >
                <ArrowLeft className="w-4 h-4" /> Back
              </button>
              <span className="text-[12px] font-mono text-[#DA7756] font-semibold">STEP 4 OF 4</span>
            </div>

            <h1 className="text-[20px] font-semibold text-[#1E1E1E] text-center tracking-tight">
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
                    className={`p-4 rounded-[10px] border cursor-pointer transition ${
                      isSelected
                        ? "border-[#DA7756] bg-[#FAF8F5]"
                        : "bg-[#F5F5F5] border-[#EBEBEB] hover:border-[#DA7756]"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2.5">
                        <Icon className={`w-4 h-4 ${isSelected ? "text-[#DA7756]" : "text-[#6B6963]"}`} />
                        <span className="font-semibold text-[14px] text-[#1E1E1E]">{card.title}</span>
                      </div>
                      <div className={`w-4 h-4 rounded-full border flex items-center justify-center ${isSelected ? "border-[#DA7756] bg-[#DA7756]" : "border-[#CCCCCC]"}`}>
                        {isSelected && <Check className="w-3 h-3 text-white" />}
                      </div>
                    </div>
                    <p className="text-[12px] text-[#6B6963] mt-1.5 leading-normal">{card.desc}</p>
                  </div>
                );
              })}
            </div>

            <div className="flex justify-end pt-4">
              <button
                onClick={handleFinishSetup}
                className="w-full py-2.5 bg-[#DA7756] hover:bg-[#C26242] text-white text-[14px] font-medium rounded-[8px] transition cursor-pointer shadow-sm"
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
