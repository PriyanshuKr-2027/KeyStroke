import React from "react";

interface SkeletonProps {
  className?: string;
  width?: string;
  height?: string;
  rounded?: string;
}

export const Skeleton: React.FC<SkeletonProps> = ({
  className = "",
  width,
  height,
  rounded = "rounded-[8px]",
}) => {
  return (
    <div
      className={`animate-pulse bg-[#EBEBEB] ${rounded} ${className}`}
      style={{
        width: width,
        height: height,
      }}
    />
  );
};

export const TableRowSkeleton: React.FC<{ count?: number }> = ({ count = 4 }) => {
  return (
    <div className="divide-y divide-[#EBEBEB] border-t border-b border-[#EBEBEB]">
      {Array.from({ length: count }).map((_, i) => (
        <div
          key={i}
          className="h-[48px] px-1 flex items-center justify-between animate-pulse"
        >
          <div className="flex items-center gap-4">
            <Skeleton width="60px" height="14px" />
            <Skeleton width="180px" height="14px" />
            <Skeleton width="120px" height="14px" />
          </div>
          <Skeleton width="48px" height="14px" />
        </div>
      ))}
    </div>
  );
};

export const StatCardSkeleton: React.FC = () => {
  return (
    <div className="flex items-center gap-3">
      <Skeleton width="90px" height="16px" />
      <Skeleton width="12px" height="16px" />
      <Skeleton width="110px" height="16px" />
      <Skeleton width="12px" height="16px" />
      <Skeleton width="100px" height="16px" />
    </div>
  );
};

export const PaletteSkeleton: React.FC = () => {
  return (
    <div className="p-3.5 space-y-2.5 animate-pulse">
      <Skeleton width="100%" height="14px" />
      <Skeleton width="85%" height="14px" />
      <Skeleton width="60%" height="14px" />
    </div>
  );
};
