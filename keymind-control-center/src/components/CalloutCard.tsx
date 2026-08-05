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
      className={`relative bg-[#FAFAE8] rounded-[12px] p-6 text-[#111111] transition-all duration-200 ${className}`}
    >
      {/* Dismiss Button */}
      <button
        onClick={handleDismiss}
        className="absolute top-4 right-4 text-[#AAAAAA] hover:text-[#111111] p-1 transition cursor-pointer"
        aria-label="Dismiss feature introduction card"
      >
        <X className="w-4 h-4" />
      </button>

      {/* Serif Headline */}
      <h3 className="font-serif text-[28px] leading-[1.25] font-normal text-[#111111] pr-8 mb-2">
        {headline}
      </h3>

      {/* Body */}
      <p className="font-sans text-[14px] text-[#6B6B6B] leading-[1.5] max-w-[620px] mb-4">
        {body}
      </p>

      {/* Chips */}
      {chips && chips.length > 0 && (
        <div className="flex flex-wrap items-center gap-2 mb-5">
          {chips.map((chip, idx) => (
            <div
              key={idx}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 bg-[#FFFFFF] border border-[#D0D0D0] rounded-lg text-[13px] text-[#111111]"
            >
              {chip.trigger && (
                <span className="font-mono text-[#111111] font-medium">{chip.trigger}</span>
              )}
              {chip.arrow && <span className="text-[#AAAAAA]">{chip.arrow}</span>}
              <span className="font-sans text-[#6B6B6B]">{chip.label}</span>
            </div>
          ))}
        </div>
      )}

      {/* Primary CTA */}
      {ctaLabel && (
        <button
          onClick={onCtaClick}
          className="inline-flex items-center justify-center px-4 py-2 bg-[#111111] hover:bg-[#333333] text-[#FFFFFF] text-[14px] font-medium rounded-[8px] transition cursor-pointer active:scale-[0.99]"
        >
          {ctaLabel}
        </button>
      )}
    </div>
  );
};
