import React, { useState } from "react";
import { Variable, VariableType } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Pencil, Trash2, X, Plus, Code, Zap, Sparkles, Layers } from "lucide-react";

interface VariablesTabProps {
  variables: Variable[];
  onUpsert: (variable: Variable) => void;
  onDelete: (key: string) => void;
  onTest: (key: string) => Promise<string>;
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
}

export const VariablesTab: React.FC<VariablesTabProps> = ({
  variables,
  onUpsert,
  onDelete,
  onTest,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [activeSubTab, setActiveSubTab] = useState<"all" | "static" | "dynamic" | "ai">("all");
  const [showCallout, setShowCallout] = useState(true);

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingVar, setEditingVar] = useState<Variable | null>(null);

  // Form state
  const [keyInput, setKeyInput] = useState("");
  const [varType, setVarType] = useState<VariableType>("static");
  const [valueInput, setValueInput] = useState("");
  const [aiPromptInput, setAiPromptInput] = useState("");

  const filteredVariables = variables.filter((v) => {
    if (activeSubTab === "static") return v.var_type === "static";
    if (activeSubTab === "dynamic") return v.var_type === "dynamic";
    if (activeSubTab === "ai") return v.var_type === "ai";
    return true;
  });

  const handleOpenAddModal = () => {
    setEditingVar(null);
    setKeyInput("");
    setVarType("static");
    setValueInput("");
    setAiPromptInput("");
    setIsModalOpen(true);
  };

  const handleOpenEditModal = (v: Variable) => {
    setEditingVar(v);
    setKeyInput(v.key);
    setVarType(v.var_type);
    setValueInput(v.value || "");
    setAiPromptInput(v.ai_prompt || "");
    setIsModalOpen(true);
  };

  const handleSaveModal = (e: React.FormEvent) => {
    e.preventDefault();
    if (!keyInput.trim()) return;

    let cleanKey = keyInput.trim();
    if (cleanKey.startsWith("/")) {
      cleanKey = cleanKey.substring(1);
    }

    const updatedVar: Variable = {
      key: cleanKey,
      var_type: varType,
      value: varType === "static" ? valueInput : undefined,
      ai_prompt: varType === "ai" ? aiPromptInput : undefined,
      use_count: editingVar ? editingVar.use_count : 0,
    };

    if (editingVar && editingVar.key !== cleanKey) {
      onDelete(editingVar.key);
    }
    onUpsert(updatedVar);
    setIsModalOpen(false);
  };

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="Failed to load Snippets"
          message={errorMessage || "Could not retrieve variables and snippets from KeyStroke engine."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  const getEmptyStateDetails = () => {
    if (activeSubTab === "static") {
      return {
        icon: Code,
        title: "No static snippets yet",
        description: "Static snippets expand trigger words like /email, /address, or /boilerplate into fixed text blocks.",
        actionLabel: "Add static snippet",
      };
    }
    if (activeSubTab === "dynamic") {
      return {
        icon: Zap,
        title: "No dynamic variables yet",
        description: "Dynamic variables auto-compute dynamic values such as current time, formatted dates, or clipboard content.",
        actionLabel: "Add dynamic variable",
      };
    }
    if (activeSubTab === "ai") {
      return {
        icon: Sparkles,
        title: "No AI prompt templates yet",
        description: "AI prompts send contextual instructions to Groq/Cerebras models to draft, summarize, or translate on the fly.",
        actionLabel: "Add AI prompt template",
      };
    }
    return {
      icon: Layers,
      title: "No snippets or variables saved",
      description: "Save shortcuts for your most frequently typed text blocks, dates, and AI instructions.",
      actionLabel: "Add your first snippet",
    };
  };

  const emptyInfo = getEmptyStateDetails();

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Header Row */}
      <div className="flex items-center justify-between">
        <h1 className="font-sans text-[22px] font-semibold text-[#111111]">
          Snippets & Variables
        </h1>

        <button
          onClick={handleOpenAddModal}
          className="inline-flex items-center gap-1.5 px-4 py-2 bg-[#111111] hover:bg-[#333333] text-[#FFFFFF] text-[14px] font-medium rounded-[8px] transition cursor-pointer"
        >
          <Plus className="w-4 h-4" /> Add new
        </button>
      </div>

      {/* Sub-tab Bar */}
      <div className="flex items-center gap-6 border-b border-[#EBEBEB] text-[14px]">
        {[
          { id: "all", label: "All" },
          { id: "static", label: "Static" },
          { id: "dynamic", label: "Dynamic" },
          { id: "ai", label: "AI Prompts" },
        ].map((tab) => {
          const isActive = activeSubTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveSubTab(tab.id as any)}
              className={`pb-2.5 font-normal transition-colors relative cursor-pointer ${
                isActive ? "text-[#111111]" : "text-[#6B6B6B] hover:text-[#111111]"
              }`}
            >
              {tab.label}
              {isActive && (
                <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-[#111111]" />
              )}
            </button>
          );
        })}
      </div>

      {/* Feature Callout Card */}
      {showCallout && (
        <CalloutCard
          headline="The stuff you shouldn't have to retype."
          body="Save shortcuts for everything you type all the time — emails, dates, addresses, templates. Type the trigger and KeyStroke expands it instantly."
          chips={[
            { trigger: "/email", arrow: "→", label: "alex@..." },
            { trigger: "/date", arrow: "→", label: "August 5, 2026" },
            { trigger: "/reply", arrow: "→", label: "AI draft" },
          ]}
          ctaLabel="Add new variable"
          onCtaClick={handleOpenAddModal}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* List Header */}
      <div className="text-[11px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase pt-2">
        SNIPPETS & EXPANSIONS
      </div>

      {/* List Rows */}
      {isLoading ? (
        <TableRowSkeleton count={4} />
      ) : filteredVariables.length > 0 ? (
        <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
          {filteredVariables.map((v) => {
            const displayExpansion =
              v.var_type === "static"
                ? v.value || "—"
                : v.var_type === "dynamic"
                ? new Date().toLocaleDateString("en-US", { month: "long", day: "numeric", year: "numeric" })
                : v.ai_prompt || "Draft AI output";

            return (
              <div
                key={v.key}
                className="h-[48px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors group"
              >
                <div className="flex items-center gap-4 text-[14px]">
                  <span className="font-mono text-[13px] font-normal text-[#111111] w-[80px]">
                    /{v.key}
                  </span>

                  <span className="text-[#AAAAAA]">→</span>

                  <span className="font-sans text-[#6B6B6B] max-w-[360px] truncate">
                    {displayExpansion}
                  </span>

                  {v.var_type !== "static" && (
                    <span className="px-2 py-0.5 bg-[#FFFFFF] border border-[#EBEBEB] text-[#6B6B6B] text-[11px] rounded-[4px] font-sans uppercase tracking-wider">
                      {v.var_type}
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
                  <button
                    onClick={() => {
                      onTest(v.key)
                        .then((res) => alert(`Test Output:\n${res}`))
                        .catch((err) => alert(`Error:\n${err}`));
                    }}
                    className="text-[#AAAAAA] hover:text-[#22C55E] p-1 cursor-pointer transition"
                    title="Test snippet"
                  >
                    <Zap className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => handleOpenEditModal(v)}
                    className="text-[#AAAAAA] hover:text-[#111111] p-1 cursor-pointer transition"
                    title="Edit snippet"
                  >
                    <Pencil className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => onDelete(v.key)}
                    className="text-[#AAAAAA] hover:text-[#EF4444] p-1 cursor-pointer transition"
                    title="Delete snippet"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      ) : (
        <EmptyState
          icon={emptyInfo.icon}
          title={emptyInfo.title}
          description={emptyInfo.description}
          actionLabel={emptyInfo.actionLabel}
          onAction={handleOpenAddModal}
        />
      )}

      {/* Add / Edit Variable Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/30 flex items-center justify-center p-4 z-50 animate-fade-in">
          <div className="bg-[#FFFFFF] rounded-[16px] max-w-md w-full p-6 space-y-5 shadow-2xl border border-[#EBEBEB]">
            <div className="flex items-center justify-between border-b border-[#EBEBEB] pb-3">
              <h3 className="text-[18px] font-semibold text-[#111111]">
                {editingVar ? "Edit Snippet / Variable" : "Add New Variable"}
              </h3>
              <button
                onClick={() => setIsModalOpen(false)}
                className="text-[#AAAAAA] hover:text-[#111111] p-1"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            <form onSubmit={handleSaveModal} className="space-y-4">
              <div>
                <label className="block text-[13px] font-medium text-[#111111] mb-1">
                  Trigger Key (without leading /)
                </label>
                <input
                  type="text"
                  required
                  value={keyInput}
                  onChange={(e) => setKeyInput(e.target.value)}
                  placeholder="e.g. email or date"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 font-mono text-[13px] text-[#111111] focus:outline-none focus:ring-1 focus:ring-[#111111]"
                />
              </div>

              <div>
                <label className="block text-[13px] font-medium text-[#111111] mb-1">
                  Variable Type
                </label>
                <div className="flex bg-[#F5F5F5] p-1 rounded-[8px] gap-1">
                  {(["static", "dynamic", "ai"] as VariableType[]).map((t) => (
                    <button
                      key={t}
                      type="button"
                      onClick={() => setVarType(t)}
                      className={`flex-1 py-1.5 text-[13px] font-medium rounded-[6px] transition cursor-pointer capitalize ${
                        varType === t
                          ? "bg-[#FFFFFF] text-[#111111] shadow-sm"
                          : "text-[#6B6B6B] hover:text-[#111111]"
                      }`}
                    >
                      {t === "ai" ? "AI Prompt" : t}
                    </button>
                  ))}
                </div>
              </div>

              {varType === "static" && (
                <div>
                  <label className="block text-[13px] font-medium text-[#111111] mb-1">
                    Static Value
                  </label>
                  <textarea
                    required
                    rows={3}
                    value={valueInput}
                    onChange={(e) => setValueInput(e.target.value)}
                    placeholder="Enter fixed snippet expansion..."
                    className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] p-3 text-[14px] text-[#111111] focus:outline-none focus:ring-1 focus:ring-[#111111]"
                  />
                </div>
              )}

              {varType === "dynamic" && (
                <div className="p-3 bg-[#F5F5F5] rounded-[8px] text-[13px] text-[#6B6B6B]">
                  Dynamic variables resolve automatically at expansion time (e.g. current date, time, formatted timestamp).
                </div>
              )}

              {varType === "ai" && (
                <div>
                  <label className="block text-[13px] font-medium text-[#111111] mb-1">
                    AI Prompt Instructions
                  </label>
                  <textarea
                    required
                    rows={3}
                    value={aiPromptInput}
                    onChange={(e) => setAiPromptInput(e.target.value)}
                    placeholder="e.g. Draft polite response to clipboard..."
                    className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] p-3 text-[14px] text-[#111111] focus:outline-none focus:ring-1 focus:ring-[#111111]"
                  />
                </div>
              )}

              <div className="flex justify-end gap-3 pt-3 border-t border-[#EBEBEB]">
                <button
                  type="button"
                  onClick={() => setIsModalOpen(false)}
                  className="px-4 py-2 rounded-[8px] border border-[#EBEBEB] text-[14px] text-[#6B6B6B] hover:text-[#111111]"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-4 py-2 rounded-[8px] bg-[#111111] text-[#FFFFFF] text-[14px] font-medium hover:bg-[#333333]"
                >
                  Save Variable
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
