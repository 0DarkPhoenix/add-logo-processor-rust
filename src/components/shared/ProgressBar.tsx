import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { ProgressInfo } from "../../types/ProgressInfo";
import { Progress } from "../ui/progress";

interface ProgressBarProps {
    isProcessing: boolean;
}

export default function ProgressBar({ isProcessing }: ProgressBarProps) {
    const [progressInfo, setProgressInfo] = useState<ProgressInfo | null>(null);
    const [isVisible, setIsVisible] = useState(false);

    useEffect(() => {
        let intervalId: NodeJS.Timeout;
        let hideTimeout: NodeJS.Timeout;

        const fetchProgress = async () => {
            try {
                const result = await invoke<ProgressInfo | null>("get_progress_info");
                console.log("result", result);
                if (result) {
                    setProgressInfo(result);
                    setIsVisible(true);
                } else if (progressInfo && !isProcessing) {
                    // Process is complete, hide after 2 seconds
                    hideTimeout = setTimeout(() => {
                        setProgressInfo(null);
                        setIsVisible(false);
                    }, 2000);
                }
            } catch (error) {
                console.error("Failed to fetch progress info:", error);
            }
        };

        if (isProcessing || isVisible) {
            // Start polling at 60 Hz when processing or still visible
            intervalId = setInterval(fetchProgress, 1000 / 60);
            // Initial fetch
            fetchProgress();
        } else {
            // Reset state when not processing and not visible
            setProgressInfo(null);
            setIsVisible(false);
        }

        return () => {
            if (intervalId) {
                clearInterval(intervalId);
            }
            if (hideTimeout) {
                clearTimeout(hideTimeout);
            }
        };
    }, [isProcessing, isVisible]); // Remove progressInfo from dependencies

    const formatTime = (seconds: number): string => {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        const secs = Math.floor(seconds % 60);

        if (hours > 0) {
            return `${hours}:${minutes.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
        }
        return `${minutes}:${secs.toString().padStart(2, "0")}`;
    };

    if (!(isVisible && progressInfo)) {
        return null;
    }

    return (
        <div className="mt-1">
            <div className='flex justify-around items-center text-sm'>
                <span className='font-medium'>{progressInfo.status}</span>
                <span>{progressInfo.itemsPerSecond.toFixed(1)} items/sec</span>
                <div className='flex gap-4'>
                    <span>Elapsed: {formatTime(progressInfo.elapsedTime)}</span>
                    {progressInfo.estimatedRemaining && (
                        <span>Remaining: {formatTime(progressInfo.estimatedRemaining)}</span>
                    )}
                </div>
                <span className='text-muted-foreground'>
                    {progressInfo.current} / {progressInfo.total} (
                    {progressInfo.percentage.toFixed(1)}%)
                </span>
            </div>

            <Progress value={Math.round(progressInfo.percentage *100)/100} className='w-full' />
        </div>
    );
}