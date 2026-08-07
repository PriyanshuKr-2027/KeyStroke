import React, { useState } from "react";
import { X } from "lucide-react";

export interface ExampleChip {
  trigger?: string;
  arrow?: string;
  label: string;
}

interface CalloutCardProps {
  headline: string;
  body: string;
  chips?: ExampleChip[];
  ctaLabel?: string;
  onCtaClick?: () => void;
  onDismiss?: () => void;
  className?: string;
}

export const CalloutCard: React.FC<CalloutCardProps> = ({
  headline,
  body,
  chips,
  ctaLabel,
  onCtaClick,
  onDismiss,
  className = "",
}) => {
  const [dismissed, setDismissed] = useState(false);

  if (dismissed) return null;

  const handleDismiss = () => {
    setDismissed(true);
    if (onDismiss) onDismiss();
  };

  return (
    <div
      className={`relative bg-[var(--accent-light)] border border-[var(--border)] rounded-[12px] p-6 text-[var(--text-primary)] transition-all duration-200 shadow-sm ${className}`}
    >
      {/* Dismiss Button */}
      <button
        onClick={handleDismiss}
        className="absolute top-4 right-4 text-[var(--text-tertiary)] hover:text-[var(--text-primary)] p-1 transition cursor-pointer"
        aria-label="Dismiss feature introduction card"
      >
        <X className="w-4 h-4" />
      </button>

      {/* Headline */}
      <h3 className="font-serif text-[24px] leading-[1.25] font-semibold text-[var(--text-primary)] pr-8 mb-2">
        {headline}
      </h3>

      {/* Body */}
      <p className="font-sans text-[14px] text-[var(--text-secondary)] leading-[1.5] max-w-[620px] mb-4">
        {body}
      </p>

      {/* Chips */}
      {chips && chips.length > 0 && (
        <div className="flex flex-wrap items-center gap-2 mb-4">
          {chips.map((chip, idx) => (
            <div
              key={idx}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 bg-[var(--surface)] border border-[var(--border)] rounded-lg text-[13px] text-[var(--text-primary)] shadow-xs"
            >
              {chip.trigger && (
                <span className="font-mono font-medium text-[var(--text-primary)]">{chip.trigger}</span>
              )}
              {chip.arrow && <span className="text-[var(--text-tertiary)]">{chip.arrow}</span>}
              <span className="font-sans text-[var(--text-secondary)]">{chip.label}</span>
            </div>
          ))}
        </div>
      )}

      {/* Primary CTA */}
      {ctaLabel && (
        <button
          onClick={onCtaClick}
          className="inline-flex items-center justify-center px-4 py-2 bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white text-[13px] font-medium rounded-[8px] transition cursor-pointer active:scale-[0.99] shadow-sm"
        >
          {ctaLabel}
        </button>
      )}
    </div>
  );
};
