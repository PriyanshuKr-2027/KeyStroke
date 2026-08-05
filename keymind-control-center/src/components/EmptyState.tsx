import React from "react";
import { LucideIcon } from "lucide-react";

interface EmptyStateProps {
  icon?: LucideIcon;
  title: string;
  description: string;
  actionLabel?: string;
  onAction?: () => void;
  secondaryActionLabel?: string;
  onSecondaryAction?: () => void;
  className?: string;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  icon: Icon,
  title,
  description,
  actionLabel,
  onAction,
  secondaryActionLabel,
  onSecondaryAction,
  className = "",
}) => {
  return (
    <div
      className={`py-12 px-4 text-center border border-dashed border-[#D1D5DB] rounded-[16px] bg-[#FAFAFA] space-y-4 my-2 animate-fade-in ${className}`}
    >
      {Icon && (
        <div className="w-12 h-12 mx-auto rounded-full bg-[#FFFFFF] border border-[#EBEBEB] shadow-sm flex items-center justify-center text-[#6B6B6B]">
          <Icon className="w-6 h-6 stroke-[1.5]" />
        </div>
      )}

      <div className="space-y-1 max-w-md mx-auto">
        <h3 className="text-[16px] font-semibold text-[#111111] font-sans tracking-tight">
          {title}
        </h3>
        <p className="text-[13px] text-[#6B6B6B] leading-relaxed font-sans">
          {description}
        </p>
      </div>

      {(actionLabel || secondaryActionLabel) && (
        <div className="flex items-center justify-center gap-3 pt-1">
          {actionLabel && onAction && (
            <button
              type="button"
              onClick={onAction}
              className="px-4 py-2 bg-[#111111] hover:bg-[#333333] text-[#FFFFFF] text-[13px] font-medium rounded-[8px] transition cursor-pointer"
            >
              {actionLabel}
            </button>
          )}

          {secondaryActionLabel && onSecondaryAction && (
            <button
              type="button"
              onClick={onSecondaryAction}
              className="px-4 py-2 bg-[#FFFFFF] border border-[#EBEBEB] hover:bg-[#F5F5F5] text-[#111111] text-[13px] font-medium rounded-[8px] transition cursor-pointer"
            >
              {secondaryActionLabel}
            </button>
          )}
        </div>
      )}
    </div>
  );
};
