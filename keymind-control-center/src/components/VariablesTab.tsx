import React, { useState } from "react";
import { Variable, VariableType } from "../types";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Pencil, Trash2, X, Plus, Search, Download, Upload } from "lucide-react";

interface VariablesTabProps {
  variables: Variable[];
  onUpsert: (variable: Variable) => void;
  onDelete: (key: string) => void;
  onTest: (key: string) => Promise<string>;
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
  onShowToast?: (title: string, message?: string, type?: "success" | "error" | "info") => void;
}

export const VariablesTab: React.FC<VariablesTabProps> = ({
  variables,
  onUpsert,
  onDelete,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
  onShowToast,
}) => {
  const [activeSubTab, setActiveSubTab] = useState<"all" | "static" | "dynamic" | "ai">("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [showCallout, setShowCallout] = useState(true);

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingVar, setEditingVar] = useState<Variable | null>(null);

  const [keyInput, setKeyInput] = useState("");
  const [varType, setVarType] = useState<VariableType>("static");
  const [valueInput, setValueInput] = useState("");
  const [aiPromptInput, setAiPromptInput] = useState("");

  const filteredVariables = variables.filter((v) => {
    const matchesTab =
      activeSubTab === "all" ||
      (activeSubTab === "static" && v.var_type === "static") ||
      (activeSubTab === "dynamic" && v.var_type === "dynamic") ||
      (activeSubTab === "ai" && v.var_type === "ai");

    const matchesSearch =
      v.key.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (v.value && v.value.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (v.ai_prompt && v.ai_prompt.toLowerCase().includes(searchQuery.toLowerCase()));

    return matchesTab && matchesSearch;
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
    if (onShowToast) onShowToast("Snippet Saved", `/${cleanKey} is ready to use`);
  };

  const handleExportJSON = () => {
    const jsonStr = JSON.stringify(variables, null, 2);
    const blob = new Blob([jsonStr], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "keystroke_snippets.json";
    a.click();
    if (onShowToast) onShowToast("Snippets Exported", "Exported JSON configuration pack", "info");
  };

  if (isError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10 font-sans">
        <ErrorState
          title="Failed to load Snippets"
          message={errorMessage || "Could not retrieve variables and snippets."}
          onRetry={onRetry}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10 font-sans select-none text-[#EDEDED]">
      {/* Top Header Bar */}
      <div className="flex items-center justify-between">
        <h1 className="text-[22px] font-semibold text-[#EDEDED] tracking-tight">
          Text Shortcuts
        </h1>

        <div className="flex items-center gap-2">
          <button
            onClick={handleExportJSON}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-[#28282D] hover:bg-[#333338] border border-[rgba(255,255,255,0.08)] rounded-[7px] text-[12px] font-medium text-[#8F8F96] hover:text-[#EDEDED] transition cursor-pointer"
          >
            <Download className="w-3.5 h-3.5" />
            <span>Export Pack</span>
          </button>

          <button
            onClick={handleOpenAddModal}
            className="flex items-center gap-1.5 px-3.5 py-1.5 bg-[#6366F1] hover:bg-[#4F46E5] text-white rounded-[7px] text-[13px] font-medium transition cursor-pointer shadow-sm"
          >
            <Plus className="w-4 h-4" />
            <span>New Shortcut</span>
          </button>
        </div>
      </div>

      {showCallout && (
        <CalloutCard
          headline="Type less. Say more."
          body="Create custom text shortcuts (e.g. /email -> user@example.com) or dynamic tokens ({date}, {time}). KeyStroke replaces them instantly as you type."
          chips={[{ label: "/email" }, { label: "/zoom" }, { label: "{date}" }]}
          ctaLabel="Add shortcut"
          onCtaClick={handleOpenAddModal}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* Filter & Search Bar */}
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-1 bg-[#1F1F23] p-1 rounded-[8px] border border-[rgba(255,255,255,0.08)] text-[12px] font-medium">
          {(["all", "static", "dynamic", "ai"] as const).map((t) => (
            <button
              key={t}
              onClick={() => setActiveSubTab(t)}
              className={`px-3 py-1 rounded-[6px] capitalize transition cursor-pointer ${
                activeSubTab === t
                  ? "bg-[#28282D] text-[#EDEDED] shadow-sm"
                  : "text-[#8F8F96] hover:text-[#EDEDED]"
              }`}
            >
              {t}
            </button>
          ))}
        </div>

        <div className="relative flex-1 max-w-[240px]">
          <Search className="w-3.5 h-3.5 absolute left-3 top-2.5 text-[#8F8F96]" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search snippets…"
            className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] pl-8 pr-3 py-1.5 text-[12px] text-[#EDEDED] placeholder-[#5C5C62] focus:outline-none focus:border-[#6366F1]"
          />
        </div>
      </div>

      {/* Snippets List */}
      {isLoading ? (
        <TableRowSkeleton count={4} />
      ) : filteredVariables.length > 0 ? (
        <div className="divide-y divide-[rgba(255,255,255,0.08)] border-t border-b border-[rgba(255,255,255,0.08)]">
          {filteredVariables.map((v) => (
            <div
              key={v.key}
              className="h-[52px] px-3 flex items-center justify-between hover:bg-[rgba(255,255,255,0.03)] transition rounded-[8px] group"
            >
              <div className="flex items-center gap-3">
                <span className="px-2 py-0.5 bg-[rgba(99,102,241,0.12)] border border-[rgba(99,102,241,0.2)] rounded-[5px] font-mono text-[12px] font-semibold text-[#6366F1]">
                  /{v.key}
                </span>
                <span className="text-[13px] text-[#EDEDED] truncate max-w-[320px]">
                  {v.var_type === "static" ? v.value : v.var_type === "dynamic" ? "Dynamic {date/time}" : v.ai_prompt}
                </span>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={() => handleOpenEditModal(v)}
                  className="p-1.5 text-[#8F8F96] hover:text-[#EDEDED] transition cursor-pointer"
                >
                  <Pencil className="w-3.5 h-3.5" />
                </button>
                <button
                  onClick={() => {
                    onDelete(v.key);
                    if (onShowToast) onShowToast("Deleted Snippet", `Deleted /${v.key}`, "info");
                  }}
                  className="p-1.5 text-[#8F8F96] hover:text-[#EF4444] transition cursor-pointer"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <EmptyState
          icon={Search}
          title="No snippets found"
          description="Create text expansions or dynamic tokens."
          actionLabel="Create Snippet"
          onAction={handleOpenAddModal}
        />
      )}

      {/* Add / Edit Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center animate-fade-in">
          <form
            onSubmit={handleSaveModal}
            className="w-[460px] bg-[#161618] border border-[rgba(255,255,255,0.12)] rounded-[14px] p-6 shadow-2xl space-y-4 animate-pop-in text-left"
          >
            <div className="flex items-center justify-between">
              <h2 className="text-[16px] font-semibold text-[#EDEDED]">
                {editingVar ? "Edit Snippet" : "New Text Expansion Snippet"}
              </h2>
              <button
                type="button"
                onClick={() => setIsModalOpen(false)}
                className="text-[#8F8F96] hover:text-[#EDEDED]"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            <div>
              <label className="block text-[12px] font-medium text-[#EDEDED] mb-1">Trigger Shortcut</label>
              <div className="flex items-center bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] px-3 py-2">
                <span className="font-mono text-[13px] text-[#6366F1] font-semibold mr-1">/</span>
                <input
                  type="text"
                  value={keyInput}
                  onChange={(e) => setKeyInput(e.target.value)}
                  placeholder="email"
                  className="w-full bg-transparent text-[13px] font-mono text-[#EDEDED] focus:outline-none"
                />
              </div>
            </div>

            <div>
              <label className="block text-[12px] font-medium text-[#EDEDED] mb-1">Expansion Content</label>
              <textarea
                rows={3}
                value={valueInput}
                onChange={(e) => setValueInput(e.target.value)}
                placeholder="user@example.com"
                className="w-full bg-[#1F1F23] border border-[rgba(255,255,255,0.08)] rounded-[8px] px-3.5 py-2 text-[13px] font-sans text-[#EDEDED] focus:outline-none resize-none"
              />
            </div>

            <div className="flex justify-end gap-2 pt-2">
              <button
                type="button"
                onClick={() => setIsModalOpen(false)}
                className="px-4 py-2 bg-[#28282D] hover:bg-[#333338] text-[#EDEDED] rounded-[7px] text-[13px] font-medium transition cursor-pointer"
              >
                Cancel
              </button>
              <button
                type="submit"
                className="px-4 py-2 bg-[#6366F1] hover:bg-[#4F46E5] text-white rounded-[7px] text-[13px] font-medium transition cursor-pointer shadow-sm"
              >
                Save Snippet
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
};
