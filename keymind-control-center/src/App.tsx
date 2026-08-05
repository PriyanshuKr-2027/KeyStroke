import React, { useState, useEffect } from "react";
import { Sidebar, TabType } from "./components/Sidebar";
import { DashboardTab } from "./components/DashboardTab";
import { VariablesTab } from "./components/VariablesTab";
import { GrammarTab } from "./components/GrammarTab";
import { ShortcutsTab } from "./components/ShortcutsTab";
import { AppsTab } from "./components/AppsTab";
import { MemoryTab } from "./components/MemoryTab";
import { SettingsTab } from "./components/SettingsTab";
import { FirstRunWizard } from "./components/FirstRunWizard";
import { SuggestionWidget } from "./components/SuggestionWidget";
import {
  EngineStatus,
  DailyStats,
  AutocorrectFeedItem,
  Variable,
  GrammarStatus,
  GrammarFix,
  GrammarMode,
  AppSettings,
  ActivePrediction,
} from "./types";
import { invoke } from "@tauri-apps/api/tauri";

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>("dashboard");
  const [showWizard, setShowWizard] = useState(false);
  const [activePrediction, setActivePrediction] = useState<ActivePrediction | null>({
    candidate_word: "you",
    full_suggestions: ["you", "there"],
    confidence: 0.85,
    context: "how are",
  });

  // State initialization
  const [engineStatus, setEngineStatus] = useState<EngineStatus>({
    engine: "running",
    ai: "connected",
    grammar: "ready",
  });

  const [dailyStats, setDailyStats] = useState<DailyStats>({
    words_typed: 4820,
    corrections_made: 142,
    variables_used: 28,
    ai_requests: 12,
  });

  const [feed, setFeed] = useState<AutocorrectFeedItem[]>([
    { id: "1", original: "recieve", corrected: "receive", time_ago: "2 min ago" },
    { id: "2", original: "teh", corrected: "the", time_ago: "5 min ago" },
    { id: "3", original: "their", corrected: "there", time_ago: "12 min ago" },
    { id: "4", original: "taht", corrected: "that", time_ago: "18 min ago" },
  ]);

  const [variables, setVariables] = useState<Variable[]>([
    { key: "phone", var_type: "static", value: "+1-555-0199", description: "Mobile phone snippet", use_count: 14 },
    { key: "date", var_type: "dynamic", description: "Current local formatted date", use_count: 8 },
    { key: "leave", var_type: "ai", ai_prompt: "Draft formal leave application letter...", use_count: 3 },
  ]);

  const [grammarStatus, setGrammarStatus] = useState<GrammarStatus>({
    enabled: true,
    mode: "Aggressive",
    language: "en-US",
  });

  const [grammarFixes, setGrammarFixes] = useState<GrammarFix[]>([
    {
      id: "f1",
      original: "He are going to teh store.",
      fixed: "He is going to the store.",
      rule_id: "HE_ARE",
      category: "GRAMMAR",
      timestamp: "3 min ago",
    },
  ]);

  const [apps, setApps] = useState<AppSettings[]>([
    {
      app_bundle_id: "com.apple.Safari",
      app_name: "Safari",
      autocorrect_enabled: true,
      grammar_enabled: true,
      ai_copilot_enabled: true,
      is_blocked: false,
    },
    {
      app_bundle_id: "com.microsoft.VSCode",
      app_name: "Visual Studio Code",
      autocorrect_enabled: false,
      grammar_enabled: false,
      ai_copilot_enabled: true,
      is_blocked: false,
    },
    {
      app_bundle_id: "com.slack.Slack",
      app_name: "Slack",
      autocorrect_enabled: true,
      grammar_enabled: true,
      ai_copilot_enabled: true,
      is_blocked: false,
    },
  ]);

  // Handle Variable CRUD
  const handleUpsertVariable = (v: Variable) => {
    setVariables((prev) => {
      const idx = prev.findIndex((item) => item.key === v.key);
      if (idx >= 0) {
        const updated = [...prev];
        updated[idx] = v;
        return updated;
      }
      return [...prev, v];
    });
  };

  const handleDeleteVariable = (key: string) => {
    setVariables((prev) => prev.filter((v) => v.key !== key));
  };

  const handleTestVariable = async (key: string): Promise<string> => {
    if (key === "date") return new Date().toLocaleDateString("en-US", { month: "long", day: "numeric", year: "numeric" });
    if (key === "phone") return "+1-555-0199";
    if (key === "leave") return "Dear Manager, Please accept this formal leave application...";
    return "Resolved value sample";
  };

  const handleUpdateApp = (updated: AppSettings) => {
    setApps((prev) =>
      prev.map((a) => (a.app_bundle_id === updated.app_bundle_id ? updated : a))
    );
  };

  const handleAcceptPrediction = async (word: string) => {
    try {
      await invoke("accept_prediction_word", { word });
    } catch (e) {
      console.log("Accept prediction word error:", e);
    }
    setActivePrediction(null);
  };

  const handleDismissPrediction = () => {
    setActivePrediction(null);
  };

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!activePrediction) return;

      if (e.key === "Tab") {
        e.preventDefault();
        handleAcceptPrediction(activePrediction.candidate_word);
      } else if (e.key === "Escape") {
        handleDismissPrediction();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [activePrediction]);

  return (
    <div className="flex h-screen bg-[#121215] bg-ambient-glow text-[#FAF8F5] overflow-hidden select-none relative font-['Plus_Jakarta_Sans',sans-serif]">
      {showWizard && (
        <FirstRunWizard
          onComplete={() => setShowWizard(false)}
          onCheckAccessibility={async () => true}
          onSaveApiKey={async (key) => key.startsWith("gsk_") || key.length > 10}
          onSaveAiKeys={async (groqKey, cerebrasKey) => {
            try {
              return await invoke<{ groq_valid: boolean; cerebras_valid: boolean }>(
                "save_ai_provider_keys",
                { groqKey, cerebrasKey }
              );
            } catch (e) {
              return { groq_valid: false, cerebras_valid: false };
            }
          }}
          onInstallLaunchAgent={async () => true}
        />
      )}

      {/* Left Sidebar */}
      <Sidebar
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        engineRunning={engineStatus.engine === "running"}
      />

      {/* Main Content Area */}
      <main className="flex-1 overflow-y-auto p-10 relative z-10">
        {activeTab === "dashboard" && (
          <DashboardTab status={engineStatus} stats={dailyStats} feed={feed} />
        )}
        {activeTab === "variables" && (
          <VariablesTab
            variables={variables}
            onUpsert={handleUpsertVariable}
            onDelete={handleDeleteVariable}
            onTest={handleTestVariable}
          />
        )}
        {activeTab === "grammar" && (
          <GrammarTab
            status={grammarStatus}
            fixes={grammarFixes}
            onToggle={(enabled) =>
              setGrammarStatus((prev) => ({ ...prev, enabled }))
            }
            onModeChange={(mode: GrammarMode) =>
              setGrammarStatus((prev) => ({ ...prev, mode }))
            }
          />
        )}
        {activeTab === "memory" && (
          <MemoryTab
            onPinPhrase={(id) => console.log("Pin phrase", id)}
            onDeletePhrase={(id) => console.log("Delete phrase", id)}
            onIgnorePhrase={(id) => console.log("Ignore phrase", id)}
            onClearAllPhrases={() => console.log("Clear all phrases")}
            onAddWord={(word) => console.log("Add word", word)}
            onDeleteWord={(id) => console.log("Delete word", id)}
            onToggleLearning={(enabled) => console.log("Toggle learning", enabled)}
          />
        )}
        {activeTab === "shortcuts" && <ShortcutsTab />}
        {activeTab === "apps" && (
          <AppsTab apps={apps} onUpdateApp={handleUpdateApp} />
        )}
        {activeTab === "settings" && <SettingsTab />}
      </main>

      {/* Floating Gboard-style Suggestion Widget */}
      <SuggestionWidget
        prediction={activePrediction}
        onAccept={handleAcceptPrediction}
        onDismiss={handleDismissPrediction}
      />
    </div>
  );
};
