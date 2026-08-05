import React, { useState } from "react";
import { Brain, Pin, Trash2, Plus, Sparkles, BookOpen } from "lucide-react";

interface MemoryTabProps {
  onPinPhrase: (id: string) => void;
  onDeletePhrase: (id: string) => void;
  onIgnorePhrase: (id: string) => void;
  onClearAllPhrases: () => void;
  onAddWord: (word: string) => void;
  onDeleteWord: (id: string) => void;
  onToggleLearning: (enabled: boolean) => void;
}

export const MemoryTab: React.FC<MemoryTabProps> = ({
  onPinPhrase,
  onDeletePhrase,
  onAddWord,
  onDeleteWord,
}) => {
  const [newWord, setNewWord] = useState("");
  const [personalWords, setPersonalWords] = useState([
    { id: "w1", word: "KeyMind", date_added: "2026-08-01" },
    { id: "w2", word: "Tauri", date_added: "2026-08-02" },
    { id: "w3", word: "SymSpell", date_added: "2026-08-03" },
  ]);

  const [learnedPhrases, setLearnedPhrases] = useState([
    { id: "1", phrase: "Quarterly financial results", frequency: 14, is_pinned: false, app: "Notes" },
    { id: "2", phrase: "Project status update", frequency: 8, is_pinned: true, app: "Slack" },
    { id: "3", phrase: "Please find attached document", frequency: 6, is_pinned: false, app: "Mail" },
  ]);

  const handleAddWordSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newWord.trim()) return;
    const wordObj = {
      id: String(Date.now()),
      word: newWord.trim(),
      date_added: new Date().toISOString().split("T")[0],
    };
    setPersonalWords((prev) => [...prev, wordObj]);
    onAddWord(newWord.trim());
    setNewWord("");
  };

  return (
    <div className="space-y-8 animate-fade-in">
      <div>
        <h2 className="text-2xl font-extrabold tracking-tight text-[#FAF8F5] flex items-center gap-3">
          <div className="p-2.5 bg-[#DA7756]/15 rounded-xl text-[#DA7756] border border-[#DA7756]/20">
            <Brain className="w-5 h-5" />
          </div>
          Memory & Personal Dictionary
        </h2>
        <p className="text-xs text-[#A1A0AB] mt-1">
          Manage learned multi-word typing phrases and custom whitelisted words.
        </p>
      </div>

      <div className="grid grid-cols-2 gap-6">
        {/* Personal Dictionary Panel */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
              <BookOpen className="w-4 h-4 text-emerald-400" />
              Personal Dictionary Whitelist
            </h3>
            <span className="text-xs text-[#71707C]">{personalWords.length} Words</span>
          </div>

          <form onSubmit={handleAddWordSubmit} className="flex gap-2">
            <input
              type="text"
              value={newWord}
              onChange={(e) => setNewWord(e.target.value)}
              placeholder="Add custom word or jargon..."
              className="flex-1 bg-zinc-950/80 border border-white/10 rounded-xl px-3.5 py-2 text-xs text-[#FAF8F5] placeholder-[#71707C] focus:outline-none focus:ring-2 focus:ring-[#DA7756]/50"
            />
            <button
              type="submit"
              className="px-3.5 py-2 bg-[#DA7756] hover:bg-[#C86544] rounded-xl text-xs font-bold text-white shadow-md shadow-[#DA7756]/20 transition cursor-pointer flex items-center gap-1"
            >
              <Plus className="w-3.5 h-3.5" /> Add
            </button>
          </form>

          <div className="flex flex-wrap gap-2 pt-2">
            {personalWords.map((w) => (
              <span
                key={w.id}
                className="inline-flex items-center gap-2 px-3 py-1.5 rounded-xl bg-zinc-900 border border-white/[0.08] text-xs font-semibold text-zinc-200 group hover:border-[#DA7756]/30 transition"
              >
                <span>{w.word}</span>
                <button
                  onClick={() => {
                    setPersonalWords((prev) => prev.filter((item) => item.id !== w.id));
                    onDeleteWord(w.id);
                  }}
                  className="text-zinc-500 hover:text-rose-400"
                >
                  <Trash2 className="w-3 h-3" />
                </button>
              </span>
            ))}
          </div>
        </div>

        {/* Learned Phrases Panel */}
        <div className="glass-panel p-6 rounded-3xl space-y-5">
          <div className="flex items-center justify-between border-b border-[#DA7756]/15 pb-4">
            <h3 className="text-sm font-bold text-[#FAF8F5] flex items-center gap-2">
              <Sparkles className="w-4 h-4 text-[#DA7756]" />
              Auto-Learned Frequent Phrases
            </h3>
            <span className="text-xs text-[#71707C]">Frequency Ranking</span>
          </div>

          <div className="space-y-3">
            {learnedPhrases.map((phrase) => (
              <div
                key={phrase.id}
                className="p-3.5 rounded-2xl bg-[#17161D]/80 border border-[#DA7756]/15 flex items-center justify-between text-xs"
              >
                <div>
                  <p className="font-bold text-[#FAF8F5]">{phrase.phrase}</p>
                  <span className="text-[10px] text-[#71707C]">Typed {phrase.frequency} times in {phrase.app}</span>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => {
                      setLearnedPhrases((prev) =>
                        prev.map((p) => (p.id === phrase.id ? { ...p, is_pinned: !p.is_pinned } : p))
                      );
                      onPinPhrase(phrase.id);
                    }}
                    className={`p-1.5 rounded-xl border transition ${
                      phrase.is_pinned
                        ? "bg-[#DA7756]/20 text-[#DA7756] border-[#DA7756]/30"
                        : "bg-zinc-800 text-[#71707C] hover:text-white border-transparent"
                    }`}
                  >
                    <Pin className="w-3.5 h-3.5" />
                  </button>

                  <button
                    onClick={() => {
                      setLearnedPhrases((prev) => prev.filter((p) => p.id !== phrase.id));
                      onDeletePhrase(phrase.id);
                    }}
                    className="p-1.5 rounded-xl bg-zinc-800 text-[#71707C] hover:text-rose-400 transition"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
