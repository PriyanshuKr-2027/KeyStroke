import React from "react";
import { AlertTriangle, RefreshCw } from "lucide-react";

interface ErrorStateProps {
  title?: string;
  message: string;
  onRetry?: () => void;
  retryLabel?: string;
  compact?: boolean;
  className?: string;
}

export const ErrorState: React.FC<ErrorStateProps> = ({
  title = "Connection Error",
  message,
  onRetry,
  retryLabel = "Try Again",
  compact = false,
  className = "",
}) => {
  if (compact) {
    return (
      <div
        className={`p-3 bg-[#FEF2F2] border border-[#FCA5A5] rounded-[10px] text-[#991B1B] text-[13px] flex items-center justify-between gap-3 ${className}`}
      >
        <div className="flex items-center gap-2 truncate">
          <AlertTriangle className="w-4 h-4 shrink-0 text-[#EF4444]" />
          <span className="truncate">{message}</span>
        </div>
        {onRetry && (
          <button
            type="button"
            onClick={onRetry}
            className="shrink-0 px-2.5 py-1 bg-[#FFFFFF] hover:bg-[#FEE2E2] border border-[#FCA5A5] text-[#991B1B] text-[12px] font-medium rounded-[6px] transition flex items-center gap-1 cursor-pointer"
          >
            <RefreshCw className="w-3 h-3" /> {retryLabel}
          </button>
        )}
      </div>
    );
  }

  return (
    <div
      className={`py-10 px-6 text-center border border-[#FCA5A5] rounded-[16px] bg-[#FEF2F2] space-y-4 my-2 animate-fade-in ${className}`}
    >
      <div className="w-12 h-12 mx-auto rounded-full bg-[#FFFFFF] border border-[#FCA5A5] shadow-sm flex items-center justify-center text-[#EF4444]">
        <AlertTriangle className="w-6 h-6 stroke-[1.5]" />
      </div>

      <div className="space-y-1 max-w-md mx-auto">
        <h3 className="text-[16px] font-semibold text-[#991B1B] font-sans tracking-tight">
          {title}
        </h3>
        <p className="text-[13px] text-[#7F1D1D] leading-relaxed font-sans">
          {message}
        </p>
      </div>

      {onRetry && (
        <div className="pt-1">
          <button
            type="button"
            onClick={onRetry}
            className="inline-flex items-center gap-1.5 px-4 py-2 bg-[#DC2626] hover:bg-[#B91C1C] text-[#FFFFFF] text-[13px] font-medium rounded-[8px] transition cursor-pointer shadow-sm"
          >
            <RefreshCw className="w-3.5 h-3.5" /> {retryLabel}
          </button>
        </div>
      )}
    </div>
  );
};
