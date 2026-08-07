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
import { ToastContainer, ToastMessage } from "./components/Toast";
import {
  EngineStatus,
  EngineStatusType,
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
import { listen } from "@tauri-apps/api/event";

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>("dashboard");
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [showWizard, setShowWizard] = useState(() => {
    return localStorage.getItem("keystroke_onboarding_done") !== "true";
  });

  const [toasts, setToasts] = useState<ToastMessage[]>([]);

  const showToast = (title: string, message?: string, type: "success" | "error" | "info" = "success") => {
    const id = `toast_${Date.now()}_${Math.random().toString(36).substring(2, 6)}`;
    setToasts((prev) => [...prev, { id, title, message, type }]);
  };

  const handleDismissToast = (id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  };

  const handleWizardComplete = () => {
    localStorage.setItem("keystroke_onboarding_done", "true");
    setShowWizard(false);
    showToast("Setup Complete", "KeyStroke is ready to use", "success");
  };

  const [activePrediction, setActivePrediction] = useState<ActivePrediction | null>(null);

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

    const unlisten = listen<{ candidate: string; suggestions: string[] }>("prediction-update", (event) => {
      if (event.payload && event.payload.candidate) {
        setActivePrediction({
          candidate_word: event.payload.candidate,
          full_suggestions: event.payload.suggestions || [event.payload.candidate],
          confidence: 0.88,
          context: "",
        });
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const handleToggleEngine = () => {
    const isRunning = engineStatus.engine === "running";
    const nextState: EngineStatusType = isRunning ? "stopped" : "running";
    setEngineStatus((prev) => ({ ...prev, engine: nextState }));
    showToast(
      isRunning ? "Interceptor Paused" : "Interceptor Resumed",
      isRunning ? "Keyboard interception is temporarily paused" : "Keyboard hook is listening system-wide",
      isRunning ? "info" : "success"
    );
  };

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
    } catch (e) {}
    setActivePrediction(null);
  };

  const handleDismissPrediction = () => {
    setActivePrediction(null);
  };

  const [theme, setTheme] = useState<"light" | "dark">(() => {
    return (localStorage.getItem("keystroke_theme") as "light" | "dark") || "light";
  });

  const handleToggleTheme = () => {
    const nextTheme = theme === "light" ? "dark" : "light";
    setTheme(nextTheme);
    localStorage.setItem("keystroke_theme", nextTheme);
  };

  return (
    <div className={`flex h-screen overflow-hidden select-none relative font-sans transition-colors duration-150 ${theme === "dark" ? "theme-dark bg-[#1B1917] text-[#ECE9E3]" : "theme-light bg-[#FAF8F5] text-[#1E1E1E]"}`}>
      {/* Toast Notification Layer */}
      <ToastContainer toasts={toasts} onDismiss={handleDismissToast} />

      {/* Onboarding Wizard if first run */}
      {showWizard && (
        <FirstRunWizard
          onComplete={handleWizardComplete}
          onCheckAccessibility={async () => true}
        />
      )}

      {/* Fixed 220px Left Sidebar */}
      <Sidebar
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        engineRunning={engineStatus.engine === "running"}
        onToggleEngine={handleToggleEngine}
        onOpenSettings={() => setIsSettingsOpen(true)}
        onOpenWizard={() => setShowWizard(true)}
        theme={theme}
        onToggleTheme={handleToggleTheme}
      />

      {/* Scrollable Main Content Area with Drag Region Header */}
      <div className={`flex-1 flex flex-col h-screen overflow-hidden transition-colors duration-150 ${theme === "dark" ? "bg-[#1B1917]" : "bg-[#FAF8F5]"}`}>
        {/* Top Window Drag Region Bar */}
        <div data-tauri-drag-region className={`h-[44px] shrink-0 border-b flex items-center justify-end px-4 cursor-default transition-colors duration-150 ${theme === "dark" ? "border-[#383430] bg-[#1B1917] text-[#A39E93]" : "border-[#E8E4DC] bg-[#FAF8F5] text-[#6B6963]"}`}>
          <div className="flex items-center gap-3">
            <button
              onClick={handleToggleTheme}
              className={`px-2.5 py-1 text-[11px] font-medium rounded-full border transition cursor-pointer flex items-center gap-1.5 ${
                theme === "dark"
                  ? "bg-[#282522] border-[#383430] text-[#ECE9E3] hover:bg-[#33302B]"
                  : "bg-white border-[#E8E4DC] text-[#1E1E1E] hover:bg-[#F3EFEA]"
              }`}
              title="Toggle Claude Light / Dark Mode"
            >
              <span>{theme === "light" ? "☀️ Claude Light Mode" : "🌙 Claude Dark Mode"}</span>
            </button>
            <span className="font-mono text-[11px]">KeyStroke Desktop v0.1.0</span>
          </div>
        </div>

        <ErrorBoundary>
          <main className="flex-1 overflow-y-auto px-10 pt-6 pb-16 relative">
            {activeTab === "dashboard" && (
              <DashboardTab
                status={engineStatus}
                stats={dailyStats}
                feed={feed}
                isLoading={isLoading}
                isError={isError}
                errorMessage={errorMessage}
                onRetry={loadAllData}
                onShowToast={showToast}
              />
            )}

            {activeTab === "memory" && (
              <MemoryTab
                onAddWord={async (word) => {
                  try {
                    await invoke("add_personal_word", { word });
                    loadAllData();
                    showToast("Dictionary Updated", `Added "${word}" to personal dictionary`);
                  } catch (e) {}
                }}
                onDeleteWord={async (id) => {
                  try {
                    await invoke("delete_personal_word", { id });
                    loadAllData();
                    showToast("Word Removed", "Removed from dictionary", "info");
                  } catch (e) {}
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
                onShowToast={showToast}
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
                onShowToast={showToast}
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
                onShowToast={showToast}
              />
            )}
          </main>
        </ErrorBoundary>
      </div>

      {/* Settings Modal */}
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
