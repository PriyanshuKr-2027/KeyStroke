import React, { useState } from "react";
import { Variable, VariableType } from "../types";
import {
  Search,
  Plus,
  Trash2,
  Play,
  Download,
  Sparkles,
  Braces,
  X,
  Zap,
} from "lucide-react";

interface VariablesTabProps {
  variables: Variable[];
  onUpsert: (variable: Variable) => void;
  onDelete: (key: string) => void;
  onTest: (key: string) => Promise<string>;
}

export const VariablesTab: React.FC<VariablesTabProps> = ({
  variables,
  onUpsert,
  onDelete,
  onTest,
}) => {
  const [search, setSearch] = useState("");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [testResult, setTestResult] = useState<{ key: string; result: string } | null>(null);

  // Form state
  const [keyInput, setKeyInput] = useState("");
  const [varType, setVarType] = useState<VariableType>("static");
  const [valueInput, setValueInput] = useState("");
  const [aiPromptInput, setAiPromptInput] = useState("");
  const [descriptionInput, setDescriptionInput] = useState("");

  const filteredVars = variables.filter(
    (v) =>
      v.key.toLowerCase().includes(search.toLowerCase()) ||
      (v.description && v.description.toLowerCase().includes(search.toLowerCase()))
  );

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    if (!keyInput.trim()) return;

    let cleanKey = keyInput.trim();
    if (cleanKey.startsWith("/")) {
      cleanKey = cleanKey.substring(1);
    }

    const newVar: Variable = {
      key: cleanKey,
      var_type: varType,
      value: varType === "static" ? valueInput : undefined,
      ai_prompt: varType === "ai" ? aiPromptInput : undefined,
      description: descriptionInput || undefined,
      use_count: 0,
    };

    onUpsert(newVar);
    setIsModalOpen(false);
    resetForm();
  };

  const resetForm = () => {
    setKeyInput("");
    setVarType("static");
    setValueInput("");
    setAiPromptInput("");
    setDescriptionInput("");
  };

  const handleTestClick = async (key: string) => {
    const res = await onTest(key);
    setTestResult({ key, result: res });
  };

  const handleExportJson = () => {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(variables, null, 2));
    const downloadAnchor = document.createElement("a");
    downloadAnchor.setAttribute("href", dataStr);
    downloadAnchor.setAttribute("download", "keymind_variables.json");
    document.body.appendChild(downloadAnchor);
    downloadAnchor.click();
    downloadAnchor.remove();
  };

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header Bar & Search */}
      <div className="flex items-center justify-between gap-4">
        <div>
          <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
            <div className="p-2.5 bg-[#DA7756]/15 rounded-xl text-[#DA7756] border border-[#DA7756]/20">
              <Braces className="w-5 h-5" />
            </div>
            Variables & In-Line Snippets
          </h2>
          <p className="text-xs text-[#A1A0AB] mt-1">
            Type <kbd className="px-1.5 py-0.5 rounded bg-zinc-800 text-[#DA7756] font-mono text-[11px]">/trigger</kbd> + <kbd className="px-1.5 py-0.5 rounded bg-zinc-800 text-[#A1A0AB] font-mono text-[11px]">Space</kbd> anywhere in Windows/macOS to expand snippets.
          </p>
        </div>

        <div className="flex items-center gap-3">
          <button
            onClick={handleExportJson}
            className="flex items-center gap-2 px-4 py-2 bg-[#1B1A22] border border-[#DA7756]/20 hover:border-[#DA7756]/50 rounded-xl text-xs font-semibold text-[#FAF8F5] transition shadow-sm cursor-pointer"
          >
            <Download className="w-3.5 h-3.5 text-[#A1A0AB]" />
            Export JSON
          </button>

          <button
            onClick={() => setIsModalOpen(true)}
            className="flex items-center gap-2 px-4 py-2 bg-[#DA7756] hover:bg-[#C86544] rounded-xl text-xs font-semibold text-white shadow-lg shadow-[#DA7756]/25 transition cursor-pointer"
          >
            <Plus className="w-4 h-4" />
            New Variable
          </button>
        </div>
      </div>

      {/* Search Input Bar */}
      <div className="relative">
        <Search className="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-[#A1A0AB]" />
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search variables by trigger key or description..."
          className="w-full glass-panel rounded-2xl pl-11 pr-4 py-3 text-sm text-[#FAF8F5] placeholder-[#71707C] focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50 focus:border-[#DA7756]/50 transition"
        />
      </div>

      {/* Variables Table */}
      <div className="glass-panel rounded-2xl overflow-hidden shadow-2xl">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-[#DA7756]/15 bg-[#16151B]/80 text-xs font-bold uppercase tracking-wider text-[#A1A0AB]">
              <th className="py-4 px-6">Trigger Key</th>
              <th className="py-4 px-6">Type</th>
              <th className="py-4 px-6">Expansion Preview</th>
              <th className="py-4 px-6">Uses</th>
              <th className="py-4 px-6 text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-[#DA7756]/10 text-sm">
            {filteredVars.map((v) => (
              <tr key={String(v.key)} className="hover:bg-white/[0.03] transition-all duration-200">
                <td className="py-4 px-6 font-mono font-bold text-[#DA7756]">
                  <span className="px-2.5 py-1 bg-[#DA7756]/10 border border-[#DA7756]/20 rounded-xl">
                    /{v.key}
                  </span>
                </td>
                <td className="py-4 px-6">
                  <span
                    className={`inline-flex items-center gap-1.5 text-[11px] font-extrabold px-2.5 py-0.5 rounded-full border ${
                      v.var_type === "static"
                        ? "bg-sky-500/10 text-sky-300 border-sky-500/30"
                        : v.var_type === "dynamic"
                        ? "bg-emerald-500/10 text-emerald-300 border-emerald-500/30"
                        : "bg-[#DA7756]/15 text-[#DA7756] border border-[#DA7756]/30"
                    }`}
                  >
                    {v.var_type === "ai" && <Sparkles className="w-3 h-3" />}
                    {v.var_type.toUpperCase()}
                  </span>
                </td>
                <td className="py-4 px-6 text-xs text-[#A1A0AB] max-w-xs truncate font-mono">
                  {v.var_type === "static"
                    ? v.value || "—"
                    : v.var_type === "ai"
                    ? v.ai_prompt || "AI Prompt"
                    : "Computed dynamic value"}
                </td>
                <td className="py-4 px-6 text-xs font-bold text-[#FAF8F5] font-mono">{v.use_count}</td>
                <td className="py-4 px-6 text-right space-x-2">
                  <button
                    onClick={() => handleTestClick(String(v.key))}
                    className="p-2 rounded-xl bg-zinc-900 hover:bg-emerald-500/20 hover:text-emerald-400 text-[#A1A0AB] border border-white/[0.08] transition cursor-pointer"
                    title="Test Resolution"
                  >
                    <Play className="w-3.5 h-3.5" />
                  </button>
                  <button
                    onClick={() => onDelete(String(v.key))}
                    className="p-2 rounded-xl bg-zinc-900 hover:bg-rose-500/20 hover:text-rose-400 text-[#A1A0AB] border border-white/[0.08] transition cursor-pointer"
                    title="Delete Variable"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Test Output Modal */}
      {testResult && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-md flex items-center justify-center p-4 z-50 animate-fade-in">
          <div className="glass-panel rounded-3xl max-w-md w-full p-6 space-y-4 shadow-2xl border border-white/10">
            <div className="flex items-center justify-between border-b border-white/10 pb-3">
              <h3 className="text-sm font-bold text-white flex items-center gap-2">
                <Zap className="w-4 h-4 text-emerald-400" />
                Test Output: /{testResult.key}
              </h3>
              <button onClick={() => setTestResult(null)} className="text-[#A1A0AB] hover:text-white p-1">
                <X className="w-4 h-4" />
              </button>
            </div>
            <div className="p-4 bg-zinc-950/80 rounded-2xl border border-emerald-500/30 font-mono text-xs text-emerald-300 whitespace-pre-wrap leading-relaxed">
              {testResult.result}
            </div>
          </div>
        </div>
      )}

      {/* New Variable Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-md flex items-center justify-center p-4 z-50 animate-fade-in">
          <div className="glass-panel rounded-3xl max-w-md w-full p-7 space-y-6 shadow-2xl border border-white/10">
            <div className="flex items-center justify-between border-b border-white/10 pb-4">
              <h3 className="text-base font-extrabold text-white">Create New Variable</h3>
              <button onClick={() => setIsModalOpen(false)} className="text-[#A1A0AB] hover:text-white p-1">
                <X className="w-4 h-4" />
              </button>
            </div>

            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-xs font-bold text-zinc-300 mb-1.5">
                  Trigger Key (without leading /)
                </label>
                <input
                  type="text"
                  required
                  value={keyInput}
                  onChange={(e) => setKeyInput(e.target.value)}
                  placeholder="e.g. phone or myemail"
                  className="w-full bg-zinc-950/80 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
                />
              </div>

              <div>
                <label className="block text-xs font-bold text-zinc-300 mb-1.5">Variable Type</label>
                <select
                  value={varType}
                  onChange={(e) => setVarType(e.target.value as VariableType)}
                  className="w-full bg-zinc-950/80 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
                >
                  <option value="static">Static Text Snippet</option>
                  <option value="dynamic">Dynamic Computed (/date, /time, etc.)</option>
                  <option value="ai">AI Prompt Expansion (Groq / Cerebras)</option>
                </select>
              </div>

              {varType === "static" && (
                <div>
                  <label className="block text-xs font-bold text-zinc-300 mb-1.5">Text Value</label>
                  <textarea
                    required
                    rows={3}
                    value={valueInput}
                    onChange={(e) => setValueInput(e.target.value)}
                    placeholder="Enter fixed snippet expansion value..."
                    className="w-full bg-zinc-950/80 border border-white/10 rounded-xl p-3 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
                  />
                </div>
              )}

              {varType === "ai" && (
                <div>
                  <label className="block text-xs font-bold text-zinc-300 mb-1.5">AI System Prompt</label>
                  <textarea
                    required
                    rows={3}
                    value={aiPromptInput}
                    onChange={(e) => setAiPromptInput(e.target.value)}
                    placeholder="e.g. Write a polite email reply to the clipboard content..."
                    className="w-full bg-zinc-950/80 border border-white/10 rounded-xl p-3 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
                  />
                </div>
              )}

              <div className="flex justify-end gap-3 pt-3">
                <button
                  type="button"
                  onClick={() => setIsModalOpen(false)}
                  className="px-4 py-2 rounded-xl border border-white/10 text-xs font-semibold text-[#A1A0AB] hover:text-white"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-5 py-2.5 rounded-xl bg-[#DA7756] hover:bg-[#C86544] text-xs font-bold text-white shadow-lg shadow-[#DA7756]/25"
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
