import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import type { ProgressInfo } from "../../types/ProgressInfo";
import { Progress } from "../ui/progress";

interface ProgressBarProps {
	isProcessing: boolean;
}

export default function ProgressBar({ isProcessing }: ProgressBarProps) {
	const [progressInfo, setProgressInfo] = useState<ProgressInfo | null>(null);
	const [isVisible, setIsVisible] = useState(false);
	const [isCompleted, setIsCompleted] = useState(false);
	const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

	useEffect(() => {
		let intervalId: NodeJS.Timeout;

		const fetchProgress = async () => {
			try {
				const result = await invoke<ProgressInfo | null>("get_progress_info");
				if (result) {
					setProgressInfo(result);
					setIsVisible(true);

					// Check if progress reaches 100%
					if (result.percentage >= 100 && !isCompleted) {
						setIsCompleted(true);
						if (intervalId) {
							clearInterval(intervalId);
						}

						// Clear any existing timeout
						if (hideTimeoutRef.current) {
							clearTimeout(hideTimeoutRef.current);
						}

						// Set timeout to hide after 2 seconds
						hideTimeoutRef.current = setTimeout(() => {
							setIsVisible(false);
							setProgressInfo(null);
							setIsCompleted(false);
						}, 5000);
					}
				} else {
					// No progress info available, stop polling
					if (intervalId) {
						clearInterval(intervalId);
					}
					setIsVisible(false);
					setProgressInfo(null);
					setIsCompleted(false);
				}
			} catch (error) {
				console.error("Failed to fetch progress info:", error);
			}
		};

		if (isProcessing || (isVisible && !isCompleted)) {
			// Start polling when processing or still visible and not completed
			intervalId = setInterval(fetchProgress, 1000 / 60);
			// Initial fetch
			fetchProgress();
		} else if (!(isProcessing || isVisible)) {
			// Reset state when not processing and not visible
			setProgressInfo(null);
			setIsCompleted(false);
		}

		return () => {
			if (intervalId) {
				clearInterval(intervalId);
			}
		};
	}, [isProcessing, isVisible, isCompleted]);

	// Cleanup timeout on unmount
	useEffect(() => {
		return () => {
			if (hideTimeoutRef.current) {
				clearTimeout(hideTimeoutRef.current);
			}
		};
	}, []);

	if (!(isVisible && progressInfo)) {
		return null;
	}

	const formatTime = (seconds: number) => {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const remainingSeconds = seconds % 60;

		const parts = [];
		if (hours > 0) {
			parts.push(`${hours}h`);
		}
		if (minutes > 0) {
			parts.push(`${minutes}m`);
		}
		parts.push(`${remainingSeconds.toFixed(3)}s`);

		return parts.join(" ");
	};

	return (
		<div className='mt-1'>
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

			<Progress value={Math.round(progressInfo.percentage * 100) / 100} className='w-full' />
		</div>
	);
}
