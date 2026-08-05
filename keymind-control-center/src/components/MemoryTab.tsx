import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { CalloutCard } from "./CalloutCard";
import { TableRowSkeleton } from "./Skeleton";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { Pencil, Trash2, X, Plus, BookOpen, Users, User } from "lucide-react";

interface DictionaryItem {
  id: string;
  word: string;
  expansion?: string;
  category?: "personal" | "shared";
}

interface MemoryTabProps {
  onPinPhrase?: (id: string) => void;
  onDeletePhrase?: (id: string) => void;
  onIgnorePhrase?: (id: string) => void;
  onClearAllPhrases?: () => void;
  onAddWord?: (word: string) => void;
  onDeleteWord?: (id: string) => void;
  onToggleLearning?: (enabled: boolean) => void;
  isLoading?: boolean;
  isError?: boolean;
  errorMessage?: string;
  onRetry?: () => void;
}

export const MemoryTab: React.FC<MemoryTabProps> = ({
  onAddWord,
  onDeleteWord,
  isLoading = false,
  isError = false,
  errorMessage = "",
  onRetry,
}) => {
  const [activeSubTab, setActiveSubTab] = useState<"all" | "personal" | "shared">("all");
  const [showCallout, setShowCallout] = useState(true);

  const [dictionaryItems, setDictionaryItems] = useState<DictionaryItem[]>([]);
  const [localLoading, setLocalLoading] = useState(true);
  const [localError, setLocalError] = useState(false);

  const fetchDictionaryData = () => {
    setLocalLoading(true);
    setLocalError(false);

    Promise.all([
      invoke<{ id: string; word: string; date_added: string }[]>("get_personal_words").catch(() => []),
      invoke<{ id: string; phrase: string; frequency: number; is_pinned: boolean }[]>("get_learned_phrases").catch(() => [])
    ])
      .then(([words, phrases]) => {
        const items: DictionaryItem[] = [
          ...(words || []).map((w) => ({ id: w.id, word: w.word, category: "personal" as const })),
          ...(phrases || []).map((p) => ({ id: p.id, word: p.phrase, category: "shared" as const }))
        ];
        setDictionaryItems(items);
      })
      .catch((err) => {
        console.error("Error fetching dictionary entries:", err);
        setLocalError(true);
      })
      .finally(() => {
        setLocalLoading(false);
      });
  };

  useEffect(() => {
    fetchDictionaryData();
  }, []);

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingItem, setEditingItem] = useState<DictionaryItem | null>(null);
  const [wordInput, setWordInput] = useState("");
  const [expansionInput, setExpansionInput] = useState("");

  const filteredItems = dictionaryItems.filter((item) => {
    if (activeSubTab === "personal") return item.category === "personal";
    if (activeSubTab === "shared") return item.category === "shared";
    return true;
  });

  const handleOpenAddModal = () => {
    setEditingItem(null);
    setWordInput("");
    setExpansionInput("");
    setIsModalOpen(true);
  };

  const handleOpenEditModal = (item: DictionaryItem) => {
    setEditingItem(item);
    setWordInput(item.word);
    setExpansionInput(item.expansion || "");
    setIsModalOpen(true);
  };

  const handleSaveModal = (e: React.FormEvent) => {
    e.preventDefault();
    if (!wordInput.trim()) return;

    const cleanWord = wordInput.trim();
    if (editingItem) {
      setDictionaryItems((prev) =>
        prev.map((i) =>
          i.id === editingItem.id
            ? { ...i, word: cleanWord, expansion: expansionInput.trim() || undefined }
            : i
        )
      );
    } else {
      invoke("add_personal_word", { word: cleanWord })
        .then(() => {
          const newItem: DictionaryItem = {
            id: String(Date.now()),
            word: cleanWord,
            expansion: expansionInput.trim() || undefined,
            category: "personal",
          };
          setDictionaryItems((prev) => [...prev, newItem]);
          if (onAddWord) onAddWord(cleanWord);
        })
        .catch((err) => console.error("add_personal_word error:", err));
    }

    setIsModalOpen(false);
  };

  const handleDelete = (id: string) => {
    invoke("delete_personal_word", { id })
      .then(() => {
        setDictionaryItems((prev) => prev.filter((i) => i.id !== id));
        if (onDeleteWord) onDeleteWord(id);
      })
      .catch(() => {
        invoke("delete_learned_phrase", { id })
          .then(() => {
            setDictionaryItems((prev) => prev.filter((i) => i.id !== id));
          })
          .catch((err) => console.error("delete_personal_word/phrase error:", err));
      });
  };

  if (isError || localError) {
    return (
      <div className="max-w-[760px] mx-auto pt-6 pb-10">
        <ErrorState
          title="Failed to load Dictionary"
          message={errorMessage || "Could not retrieve dictionary entries from the local storage backend."}
          onRetry={onRetry || fetchDictionaryData}
        />
      </div>
    );
  }

  const getEmptyStateDetails = () => {
    if (activeSubTab === "personal") {
      return {
        icon: User,
        title: "No personal words added yet",
        description: "Add technical terms, code symbols, client names, or jargon to ensure KeyMind never flags them as typos.",
        actionLabel: "Add your first word",
      };
    }
    if (activeSubTab === "shared") {
      return {
        icon: Users,
        title: "No shared team phrases yet",
        description: "Shared dictionary entries sync across your team's KeyMind clients to enforce common spelling standards.",
        actionLabel: "Add team phrase",
      };
    }
    return {
      icon: BookOpen,
      title: "Your dictionary is empty",
      description: "KeyMind uses your custom dictionary alongside SymSpell. Add terms or abbreviations to get started.",
      actionLabel: "Add new word",
    };
  };

  const emptyInfo = getEmptyStateDetails();

  return (
    <div className="space-y-6 animate-fade-in max-w-[760px] mx-auto pb-10">
      {/* Header Row */}
      <div className="flex items-center justify-between">
        <h1 className="font-sans text-[22px] font-semibold text-[#111111]">
          Dictionary
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
          { id: "personal", label: "Personal" },
          { id: "shared", label: "Shared with team" },
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
          headline="KeyMind learns the way you speak."
          body="Add personal terms, technical jargon, client names, or abbreviations. KeyMind will never flag them as typos."
          chips={[
            { label: "SymSpell" },
            { label: "SQLite" },
            { trigger: "btw", arrow: "→", label: "by the way" },
            { label: "Priyanshu" },
          ]}
          ctaLabel="Add new word"
          onCtaClick={handleOpenAddModal}
          onDismiss={() => setShowCallout(false)}
        />
      )}

      {/* List Header */}
      <div className="text-[11px] font-sans font-semibold tracking-wider text-[#AAAAAA] uppercase pt-2">
        PERSONAL DICTIONARY
      </div>

      {/* Dictionary Rows List */}
      {isLoading || localLoading ? (
        <TableRowSkeleton count={4} />
      ) : filteredItems.length > 0 ? (
        <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
          {filteredItems.map((item) => (
            <div
              key={item.id}
              className="h-[48px] px-1 flex items-center justify-between hover:bg-[#FAFAFA] transition-colors group"
            >
              <div className="flex items-center gap-2 text-[14px]">
                {item.expansion ? (
                  <>
                    <span className="font-mono text-[#111111]">{item.word}</span>
                    <span className="text-[#AAAAAA]">→</span>
                    <span className="font-sans text-[#6B6B6B]">{item.expansion}</span>
                  </>
                ) : (
                  <span className="font-sans text-[#111111]">{item.word}</span>
                )}
              </div>

              <div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onClick={() => handleOpenEditModal(item)}
                  className="text-[#AAAAAA] hover:text-[#111111] p-1 cursor-pointer transition"
                  title="Edit word"
                >
                  <Pencil className="w-4 h-4" />
                </button>
                <button
                  onClick={() => handleDelete(item.id)}
                  className="text-[#AAAAAA] hover:text-[#EF4444] p-1 cursor-pointer transition"
                  title="Delete word"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>
            </div>
          ))}
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

      {/* Add / Edit Word Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/30 flex items-center justify-center p-4 z-50 animate-fade-in">
          <div className="bg-[#FFFFFF] rounded-[16px] max-w-md w-full p-6 space-y-5 shadow-2xl border border-[#EBEBEB]">
            <div className="flex items-center justify-between border-b border-[#EBEBEB] pb-3">
              <h3 className="text-[18px] font-semibold text-[#111111]">
                {editingItem ? "Edit Dictionary Entry" : "Add New Word"}
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
                  Word or Abbreviation Trigger
                </label>
                <input
                  type="text"
                  required
                  value={wordInput}
                  onChange={(e) => setWordInput(e.target.value)}
                  placeholder="e.g. SymSpell or btw"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[14px] text-[#111111] focus:outline-none focus:ring-1 focus:ring-[#111111]"
                />
              </div>

              <div>
                <label className="block text-[13px] font-medium text-[#111111] mb-1">
                  Optional Expansion (Leave blank for whitelist word)
                </label>
                <input
                  type="text"
                  value={expansionInput}
                  onChange={(e) => setExpansionInput(e.target.value)}
                  placeholder="e.g. by the way"
                  className="w-full bg-[#F5F5F5] border border-[#EBEBEB] rounded-[8px] px-3.5 py-2 text-[14px] text-[#111111] focus:outline-none focus:ring-1 focus:ring-[#111111]"
                />
              </div>

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
                  Save Entry
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
