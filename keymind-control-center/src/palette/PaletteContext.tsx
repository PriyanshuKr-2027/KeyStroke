import React from 'react';

interface CapturedContext {
  before: string;
  after: string;
  app_name: string;
  has_text_field: boolean;
}

interface PaletteContextProps {
  context: CapturedContext | null;
}

export const PaletteContextView: React.FC<PaletteContextProps> = ({ context }) => {
  if (!context) return null;

  const previewBefore = context.before.slice(-40);
  const previewAfter = context.after.slice(0, 40);

  return (
    <div className="flex items-center justify-between px-3 py-1.5 border-b border-white/5 bg-white/[0.02]">
      <div className="flex items-center gap-2 overflow-hidden">
        <span className="context-pill flex items-center gap-1.5 shrink-0">
          <span className="w-1.5 h-1.5 rounded-full bg-blue-400"></span>
          {context.app_name || 'Active App'}
        </span>

        {(previewBefore || previewAfter) && (
          <span className="text-[11px] text-gray-400 font-mono truncate max-w-[360px]">
            {previewBefore && <span className="opacity-60">...{previewBefore}</span>}
            <span className="text-blue-400 font-semibold px-0.5">|</span>
            {previewAfter && <span className="opacity-60">{previewAfter}...</span>}
          </span>
        )}
      </div>

      <div className="text-[10px] text-gray-500 font-medium tracking-wide uppercase shrink-0">
        AI Copilot
      </div>
    </div>
  );
};
