import React from 'react';

interface PaletteResultProps {
  resultText: string;
  onInsert: () => void;
  onCopy: () => void;
}

export const PaletteResultView: React.FC<PaletteResultProps> = ({
  resultText,
  onInsert,
  onCopy,
}) => {
  return (
    <div className="border-t border-white/10 p-3 bg-white/[0.02]">
      <div className="max-h-[140px] overflow-y-auto pr-1 text-xs text-gray-200 font-mono leading-relaxed select-text whitespace-pre-wrap">
        {resultText}
      </div>

      <div className="flex items-center justify-between mt-3 pt-2 border-t border-white/5">
        <div className="flex items-center gap-3 text-[11px]">
          <button
            onClick={onInsert}
            className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-blue-600 hover:bg-blue-500 text-white font-medium transition-colors"
          >
            <span>Insert</span>
            <span className="badge-key">↵</span>
          </button>

          <button
            onClick={onCopy}
            className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-white/10 hover:bg-white/20 text-gray-300 font-medium transition-colors"
          >
            <span>Copy</span>
            <span className="badge-key">Tab</span>
          </button>
        </div>

        <div className="text-[10px] text-gray-500 font-mono">
          <span className="badge-key">Esc</span> Cancel
        </div>
      </div>
    </div>
  );
};
