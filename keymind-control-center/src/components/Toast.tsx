import React, { useEffect } from "react";
import { CheckCircle2, AlertCircle, Info } from "lucide-react";

export interface ToastMessage {
  id: string;
  type: "success" | "error" | "info";
  title: string;
  message?: string;
}

interface ToastProps {
  toasts: ToastMessage[];
  onDismiss: (id: string) => void;
}

export const ToastContainer: React.FC<ToastProps> = ({ toasts, onDismiss }) => {
  return (
    <div className="fixed bottom-5 right-5 z-50 flex flex-col gap-2 pointer-events-none">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onDismiss={onDismiss} />
      ))}
    </div>
  );
};

const ToastItem: React.FC<{ toast: ToastMessage; onDismiss: (id: string) => void }> = ({
  toast,
  onDismiss,
}) => {
  useEffect(() => {
    const timer = setTimeout(() => {
      onDismiss(toast.id);
    }, 3200);
    return () => clearTimeout(timer);
  }, [toast.id, onDismiss]);

  const icons = {
    success: <CheckCircle2 className="w-4 h-4 text-[#22C55E]" />,
    error: <AlertCircle className="w-4 h-4 text-[#EF4444]" />,
    info: <Info className="w-4 h-4 text-[#6366F1]" />,
  };

  return (
    <div className="pointer-events-auto bg-[#1A1A1E] border border-[rgba(255,255,255,0.12)] text-[#EDEDED] px-4 py-3 rounded-[10px] shadow-2xl flex items-center gap-3 max-w-[360px] animate-slide-up font-sans">
      <div className="shrink-0">{icons[toast.type]}</div>
      <div className="flex-1 min-w-0">
        <p className="text-[13px] font-medium leading-none text-[#EDEDED]">{toast.title}</p>
        {toast.message && (
          <p className="text-[12px] text-[#8F8F96] mt-1 truncate">{toast.message}</p>
        )}
      </div>
    </div>
  );
};
