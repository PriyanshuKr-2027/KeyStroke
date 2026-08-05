import React from "react";

export interface SettingsRowItem {
  id: string;
  label: string;
  subtitle?: string;
  type?: "toggle" | "button" | "custom" | "radio";
  checked?: boolean;
  onToggle?: (checked: boolean) => void;
  buttonLabel?: string;
  onButtonClick?: () => void;
  customControl?: React.ReactNode;
}

interface SettingsRowGroupProps {
  items: SettingsRowItem[];
  className?: string;
}

export const SettingsRowGroup: React.FC<SettingsRowGroupProps> = ({
  items,
  className = "",
}) => {
  return (
    <div
      className={`bg-[#F5F5F5] rounded-[10px] overflow-hidden divide-y divide-[#EBEBEB] ${className}`}
    >
      {items.map((row) => (
        <div
          key={row.id}
          className="min-h-[48px] px-4 py-2.5 flex items-center justify-between gap-4"
        >
          {/* Left Text */}
          <div>
            <p className="font-sans text-[14px] font-normal text-[#111111] leading-tight">
              {row.label}
            </p>
            {row.subtitle && (
              <p className="font-sans text-[13px] text-[#6B6B6B] mt-0.5 leading-tight">
                {row.subtitle}
              </p>
            )}
          </div>

          {/* Right Control */}
          <div>
            {row.type === "button" ? (
              <button
                onClick={row.onButtonClick}
                className="px-3 py-1.5 bg-[#F0F0F0] hover:bg-[#E5E5E5] text-[#111111] text-[13px] font-medium rounded-[8px] transition cursor-pointer"
              >
                {row.buttonLabel || "Change"}
              </button>
            ) : row.type === "custom" ? (
              row.customControl
            ) : (
              /* Toggle Switch 44x24px */
              <button
                type="button"
                onClick={() => row.onToggle && row.onToggle(!row.checked)}
                className={`w-[44px] h-[24px] rounded-full p-[2px] transition-colors duration-150 cursor-pointer flex items-center ${
                  row.checked ? "bg-[#22C55E]" : "bg-[#D1D5DB]"
                }`}
                aria-pressed={row.checked}
              >
                <div
                  className={`w-[20px] h-[20px] rounded-full bg-[#FFFFFF] shadow-sm transform transition-transform duration-150 ${
                    row.checked ? "translate-x-[20px]" : "translate-x-0"
                  }`}
                />
              </button>
            )}
          </div>
        </div>
      ))}
    </div>
  );
};
