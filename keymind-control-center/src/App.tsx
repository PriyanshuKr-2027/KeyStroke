import React, { useState, useEffect } from "react";
import { Sidebar, TabType } from "./components/Sidebar";
import { DashboardTab } from "./components/DashboardTab";
import { MemoryTab } from "./components/MemoryTab";
import { VariablesTab } from "./components/VariablesTab";
import { GrammarTab } from "./components/GrammarTab";
import { AppsTab } from "./components/AppsTab";
import { ShortcutsTab } from "./components/ShortcutsTab";
import { SettingsTab } from "./components/SettingsTab";
import { FirstRunWizard } from "./components/FirstRunWizard";
import { SuggestionWidget } from "./components/SuggestionWidget";
import { ErrorBoundary } from "./components/ErrorBoundary";
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
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [showWizard, setShowWizard] = useState(() => {
    return localStorage.getItem("keystroke_onboarding_done") !== "true";
  });

  const handleWizardComplete = () => {
    localStorage.setItem("keystroke_onboarding_done", "true");
    setShowWizard(false);
  };

  const [activePrediction, setActivePrediction] = useState<ActivePrediction | null>(null);

  // State initialization connected to backend
  const [engineStatus, setEngineStatus] = useState<EngineStatus>({
    engine: "running",
    ai: "connected",
    grammar: "ready",
  });

  const [dailyStats, setDailyStats] = useState<DailyStats>({
    words_typed: 0,
    corrections_made: 0,
    variables_used: 0,
    ai_requests: 0,
  });

  const [feed, setFeed] = useState<AutocorrectFeedItem[]>([]);
  const [variables, setVariables] = useState<Variable[]>([]);
  const [grammarStatus, setGrammarStatus] = useState<GrammarStatus>({
    enabled: true,
    mode: "Aggressive",
    language: "en-US",
  });

  const [grammarFixes, setGrammarFixes] = useState<GrammarFix[]>([]);
  const [apps, setApps] = useState<AppSettings[]>([]);

  // Top-level loading and error state
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  const loadAllData = () => {
    setIsLoading(true);
    setIsError(false);
    setErrorMessage("");

    Promise.allSettled([
      invoke<EngineStatus>("get_engine_status").then((res) => setEngineStatus(res)),
      invoke<DailyStats>("get_stats").then((res) => setDailyStats(res)),
      invoke<GrammarFix[]>("get_recent_grammar_fixes").then((res) => {
        setGrammarFixes(res || []);
        if (res) {
          setFeed(
            res.map((f) => ({
              id: f.id,
              original: f.original,
              corrected: f.fixed,
              time_ago: f.timestamp,
            }))
          );
        }
      }),
      invoke<Variable[]>("get_variables").then((res) => setVariables(res || [])),
      invoke<GrammarStatus>("get_grammar_status").then((res) => setGrammarStatus(res)),
      invoke<AppSettings[]>("get_app_settings").then((res) => setApps(res || [])),
    ])
      .then((results) => {
        const hasError = results.some((r) => r.status === "rejected");
        if (hasError) {
          setIsError(true);
          setErrorMessage("Failed to load some components. The backend engine may be unreachable.");
        }
      })
      .finally(() => {
        setIsLoading(false);
      });
  };

  useEffect(() => {
    loadAllData();
  }, []);

  // Variable CRUD
  const handleUpsertVariable = (v: Variable) => {
    invoke("upsert_variable", { v })
      .then(() => {
        setVariables((prev) => {
          const idx = prev.findIndex((item) => item.key === v.key);
          if (idx >= 0) {
            const updated = [...prev];
            updated[idx] = v;
            return updated;
          }
          return [...prev, v];
        });
      })
      .catch((err) => console.error("upsert_variable error:", err));
  };

  const handleDeleteVariable = (key: string) => {
    invoke("delete_variable", { key })
      .then(() => {
        setVariables((prev) => prev.filter((v) => v.key !== key));
      })
      .catch((err) => console.error("delete_variable error:", err));
  };

  const handleTestVariable = async (key: string): Promise<string> => {
    try {
      return await invoke<string>("test_variable", { key });
    } catch (e) {
      console.error("test_variable error:", e);
      return "";
    }
  };

  const handleUpdateApp = (updated: AppSettings) => {
    invoke("update_app_settings", { s: updated })
      .then(() => {
        setApps((prev) =>
          prev.map((a) => (a.app_bundle_id === updated.app_bundle_id ? updated : a))
        );
      })
      .catch((err) => console.error("update_app_settings error:", err));
  };

  const handleToggleGrammar = (enabled: boolean) => {
    invoke("toggle_grammar", { enabled })
      .then(() => setGrammarStatus((prev) => ({ ...prev, enabled })))
      .catch((err) => console.error("toggle_grammar error:", err));
  };

  const handleModeChange = (mode: GrammarMode) => {
    invoke("set_grammar_mode", { mode })
      .then(() => setGrammarStatus((prev) => ({ ...prev, mode })))
      .catch((err) => console.error("set_grammar_mode error:", err));
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
    if (!activePrediction) return;

    const handleKeyDown = (e: KeyboardEvent) => {
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
    <div className="flex h-screen bg-[#FFFFFF] text-[#111111] overflow-hidden select-none relative font-sans">
      {/* Onboarding Wizard if first run */}
      {showWizard && (
        <FirstRunWizard
          onComplete={handleWizardComplete}
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

      {/* Fixed 200px Left Sidebar */}
      <Sidebar
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        engineRunning={engineStatus.engine === "running"}
        onOpenSettings={() => setIsSettingsOpen(true)}
        onOpenWizard={() => setShowWizard(true)}
      />

      {/* Scrollable Right Content Area */}
      <ErrorBoundary>
        <main className="flex-1 overflow-y-auto px-12 pt-10 pb-16 relative bg-[#FFFFFF]">
        {activeTab === "dashboard" && (
          <DashboardTab
            status={engineStatus}
            stats={dailyStats}
            feed={feed}
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}

        {activeTab === "memory" && (
          <MemoryTab
            onAddWord={async (word) => {
              try { await invoke("add_personal_word", { word }); loadAllData(); }
              catch (e) { console.error("Failed to add word:", e); }
            }}
            onDeleteWord={async (id) => {
              try { await invoke("delete_personal_word", { id }); loadAllData(); }
              catch (e) { console.error("Failed to delete word:", e); }
            }}
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}

        {activeTab === "variables" && (
          <VariablesTab
            variables={variables}
            onUpsert={handleUpsertVariable}
            onDelete={handleDeleteVariable}
            onTest={handleTestVariable}
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}

        {activeTab === "grammar" && (
          <GrammarTab
            status={grammarStatus}
            fixes={grammarFixes}
            onToggle={handleToggleGrammar}
            onModeChange={handleModeChange}
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}

        {activeTab === "apps" && (
          <AppsTab
            apps={apps}
            onUpdateApp={handleUpdateApp}
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}

        {activeTab === "shortcuts" && (
          <ShortcutsTab
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
            onRetry={loadAllData}
          />
        )}
      </main>
      </ErrorBoundary>

      {/* Settings Modal Overlay */}
      <SettingsTab
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
      />

      {/* Floating Next-Word Suggestion Pill */}
      <SuggestionWidget
        prediction={activePrediction}
        onAccept={handleAcceptPrediction}
        onDismiss={handleDismissPrediction}
      />
    </div>
  );
};
