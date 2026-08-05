import React, { useState, useEffect } from "react";
import { ShieldCheck, Key, Rocket, Check, ArrowRight, ExternalLink, Loader2, Sparkles } from "lucide-react";

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
  onSaveApiKey,
  onSaveAiKeys,
  onInstallLaunchAgent,
}) => {
  const [currentStep, setCurrentStep] = useState<1 | 2 | 3>(1);
  const [axGranted, setAxGranted] = useState(false);
  const [apiKeyInput, setApiKeyInput] = useState("");
  const [cerebrasKeyInput, setCerebrasKeyInput] = useState("");
  const [isTestingKey, setIsTestingKey] = useState(false);
  const [keyValid, setKeyValid] = useState<boolean | null>(null);
  const [keyError, setKeyError] = useState("");
  const [isInstallingAgent, setIsInstallingAgent] = useState(false);
  const [agentInstalled, setAgentInstalled] = useState(false);

  // Step 1: Accessibility Polling every 2s
  useEffect(() => {
    if (currentStep === 1 && !axGranted) {
      const interval = setInterval(async () => {
        const granted = await onCheckAccessibility();
        if (granted) {
          setAxGranted(true);
          clearInterval(interval);
          setTimeout(() => setCurrentStep(2), 1000);
        }
      }, 2000);
      return () => clearInterval(interval);
    }
  }, [currentStep, axGranted, onCheckAccessibility]);

  const handleOpenSystemPrefs = () => {
    // Call open system prefs via Tauri or shell
    window.open("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility");
  };

  // Step 2: Test & Save Groq / Cerebras API Keys
  const handleTestAndSaveKey = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!apiKeyInput.trim() && !cerebrasKeyInput.trim()) {
      setKeyError("Please enter at least one API Key (Groq or Cerebras).");
      return;
    }

    setIsTestingKey(true);
    setKeyError("");

    try {
      if (onSaveAiKeys) {
        const result = await onSaveAiKeys(apiKeyInput.trim(), cerebrasKeyInput.trim());
        setIsTestingKey(false);

        if (result.groq_valid || result.cerebras_valid) {
          setKeyValid(true);
          setTimeout(() => setCurrentStep(3), 1000);
        } else {
          setKeyValid(false);
          setKeyError("Invalid API Key(s). Please check your key string.");
        }
      } else {
        const isValid = await onSaveApiKey(apiKeyInput.trim());
        setIsTestingKey(false);
        if (isValid) {
          setKeyValid(true);
          setTimeout(() => setCurrentStep(3), 1000);
        } else {
          setKeyValid(false);
          setKeyError("Invalid Groq API Key. Please verify your key.");
        }
      }
    } catch (err: any) {
      setIsTestingKey(false);
      setKeyValid(false);
      setKeyError(err?.message || "Failed to verify API key");
    }
  };

  // Step 3: Install LaunchAgent
  const handleEnableAutoStart = async () => {
    setIsInstallingAgent(true);
    const success = await onInstallLaunchAgent();
    setIsInstallingAgent(false);
    if (success) {
      setAgentInstalled(true);
      setTimeout(() => onComplete(), 1000);
    }
  };

  return (
    <div className="fixed inset-0 bg-[#0F0F0F] text-[#F5F5F5] flex flex-col items-center justify-center p-6 select-none z-50">
      <div className="max-w-xl w-full bg-[#141414] border border-[#2A2A2A] rounded-3xl p-8 shadow-2xl space-y-8">
        {/* Wizard Header */}
        <div className="text-center space-y-2">
          <div className="inline-flex p-3 bg-[#8B5CF6]/10 rounded-2xl text-[#8B5CF6] mb-2">
            <Sparkles className="w-8 h-8" />
          </div>
          <h1 className="text-2xl font-bold text-white">Welcome to KeyMind</h1>
          <p className="text-sm text-[#888888]">
            Let's get your macOS typing assistant configured in 3 simple steps.
          </p>
        </div>

        {/* Step Indicator Row */}
        <div className="flex items-center justify-between px-4">
          <div className="flex items-center gap-2">
            <div
              className={`w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold ${
                currentStep > 1
                  ? "bg-emerald-500 text-white"
                  : currentStep === 1
                  ? "bg-[#8B5CF6] text-white ring-4 ring-[#8B5CF6]/20"
                  : "bg-[#2A2A2A] text-[#888888]"
              }`}
            >
              {currentStep > 1 ? <Check className="w-4 h-4" /> : "1"}
            </div>
            <span className="text-xs font-medium text-white">Accessibility</span>
          </div>

          <div className="h-[2px] flex-1 mx-3 bg-[#2A2A2A]" />

          <div className="flex items-center gap-2">
            <div
              className={`w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold ${
                currentStep > 2
                  ? "bg-emerald-500 text-white"
                  : currentStep === 2
                  ? "bg-[#8B5CF6] text-white ring-4 ring-[#8B5CF6]/20"
                  : "bg-[#2A2A2A] text-[#888888]"
              }`}
            >
              {currentStep > 2 ? <Check className="w-4 h-4" /> : "2"}
            </div>
            <span className="text-xs font-medium text-white">Groq AI</span>
          </div>

          <div className="h-[2px] flex-1 mx-3 bg-[#2A2A2A]" />

          <div className="flex items-center gap-2">
            <div
              className={`w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold ${
                currentStep === 3
                  ? "bg-[#8B5CF6] text-white ring-4 ring-[#8B5CF6]/20"
                  : "bg-[#2A2A2A] text-[#888888]"
              }`}
            >
              3
            </div>
            <span className="text-xs font-medium text-white">Auto-Start</span>
          </div>
        </div>

        {/* STEP 1: ACCESSIBILITY */}
        {currentStep === 1 && (
          <div className="p-6 bg-[#1A1A1A] border border-[#2A2A2A] rounded-2xl space-y-4 text-center">
            <div className="p-3 bg-[#8B5CF6]/10 rounded-2xl w-fit mx-auto text-[#8B5CF6]">
              <ShieldCheck className="w-8 h-8" />
            </div>
            <div>
              <h3 className="text-base font-semibold text-white">Grant Accessibility Access</h3>
              <p className="text-xs text-[#888888] max-w-md mx-auto mt-1">
                KeyMind requires macOS Accessibility privacy permission to detect keystroke event boundaries and inject text expansions.
              </p>
            </div>

            <div className="pt-2">
              {axGranted ? (
                <div className="inline-flex items-center gap-2 px-4 py-2 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-semibold">
                  <Check className="w-4 h-4" /> Accessibility Permission Granted!
                </div>
              ) : (
                <button
                  onClick={handleOpenSystemPrefs}
                  className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-[#8B5CF6] hover:bg-[#7C3AED] text-xs font-semibold text-white shadow-lg shadow-[#8B5CF6]/20 transition"
                >
                  Open System Preferences <ExternalLink className="w-3.5 h-3.5" />
                </button>
              )}
            </div>

            <p className="text-[11px] text-[#888888] flex items-center justify-center gap-1.5 pt-2">
              <Loader2 className="w-3 h-3 animate-spin text-[#8B5CF6]" />
              Polling system accessibility trust status...
            </p>
          </div>
        )}

        {/* STEP 2: GROQ & CEREBRAS AI KEYS */}
        {currentStep === 2 && (
          <form onSubmit={handleTestAndSaveKey} className="p-6 bg-[#1A1A1A] border border-[#2A2A2A] rounded-2xl space-y-4">
            <div className="p-3 bg-[#8B5CF6]/10 rounded-2xl w-fit mx-auto text-[#8B5CF6]">
              <Key className="w-8 h-8" />
            </div>
            <div className="text-center">
              <h3 className="text-base font-semibold text-white">Configure AI Copilot Keys</h3>
              <p className="text-xs text-[#888888] max-w-md mx-auto mt-1">
                Provide a Groq API Key, a Cerebras API Key, or both. If configured together, requests automatically fallback if one service hits a rate limit.
              </p>
            </div>

            <div className="space-y-3">
              <div>
                <label className="block text-xs font-semibold text-white mb-1">Groq API Key (Primary)</label>
                <input
                  type="password"
                  value={apiKeyInput}
                  onChange={(e) => setApiKeyInput(e.target.value)}
                  placeholder="gsk_..."
                  className="w-full bg-[#0F0F0F] border border-[#2A2A2A] rounded-xl px-4 py-2 text-sm text-white font-mono placeholder-[#888888] focus:outline-none focus:border-[#8B5CF6]"
                />
              </div>

              <div>
                <label className="block text-xs font-semibold text-white mb-1">Cerebras API Key (Secondary / Fallback)</label>
                <input
                  type="password"
                  value={cerebrasKeyInput}
                  onChange={(e) => setCerebrasKeyInput(e.target.value)}
                  placeholder="csk_..."
                  className="w-full bg-[#0F0F0F] border border-[#2A2A2A] rounded-xl px-4 py-2 text-sm text-white font-mono placeholder-[#888888] focus:outline-none focus:border-[#8B5CF6]"
                />
              </div>
            </div>

            {keyError && (
              <p className="text-xs text-rose-400 text-center font-medium">{keyError}</p>
            )}

            {keyValid && (
              <p className="text-xs text-emerald-400 text-center font-medium flex items-center justify-center gap-1">
                <Check className="w-4 h-4" /> AI Provider Keys Verified & Saved to ~/.config/keymind/.env
              </p>
            )}

            <div className="flex justify-end pt-2">
              <button
                type="submit"
                disabled={isTestingKey}
                className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-[#8B5CF6] hover:bg-[#7C3AED] disabled:opacity-50 text-xs font-semibold text-white shadow-lg shadow-[#8B5CF6]/20 transition"
              >
                {isTestingKey ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" /> Verifying Keys...
                  </>
                ) : (
                  <>
                    Verify & Continue <ArrowRight className="w-4 h-4" />
                  </>
                )}
              </button>
            </div>
          </form>
        )}

        {/* STEP 3: AUTO-START */}
        {currentStep === 3 && (
          <div className="p-6 bg-[#1A1A1A] border border-[#2A2A2A] rounded-2xl space-y-4 text-center">
            <div className="p-3 bg-[#8B5CF6]/10 rounded-2xl w-fit mx-auto text-[#8B5CF6]">
              <Rocket className="w-8 h-8" />
            </div>
            <div>
              <h3 className="text-base font-semibold text-white">Enable Background Auto-Start</h3>
              <p className="text-xs text-[#888888] max-w-md mx-auto mt-1">
                Installs LaunchAgent daemon so KeyMind engine runs seamlessly in the background whenever you log into macOS.
              </p>
            </div>

            <div className="pt-2">
              {agentInstalled ? (
                <div className="inline-flex items-center gap-2 px-4 py-2 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-semibold">
                  <Check className="w-4 h-4" /> LaunchAgent Installed & Loaded!
                </div>
              ) : (
                <button
                  onClick={handleEnableAutoStart}
                  disabled={isInstallingAgent}
                  className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-[#8B5CF6] hover:bg-[#7C3AED] text-xs font-semibold text-white shadow-lg shadow-[#8B5CF6]/20 transition"
                >
                  {isInstallingAgent ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin" /> Installing...
                    </>
                  ) : (
                    <>
                      Install & Start Engine <Rocket className="w-4 h-4" />
                    </>
                  )}
                </button>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
