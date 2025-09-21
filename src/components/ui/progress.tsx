import * as ProgressPrimitive from "@radix-ui/react-progress";
import type * as React from "react";

import { cn } from "@/lib/utils";

interface ProgressProps extends React.ComponentProps<typeof ProgressPrimitive.Root> {
	value?: number;
}

function Progress({ className, value, ...props }: ProgressProps) {
	const progressValue = value || 0;
	const isComplete = progressValue >= 100;
	const isAtZero = progressValue === 0;

	const progressBarElement = (
		<ProgressPrimitive.Root
			data-slot='progress'
			className={cn(
				"bg-gray-800 relative h-2 w-full overflow-hidden rounded-full",
				className,
			)}
			{...props}
		>
			<style>
				{`@keyframes progress-shine {
                        0% {
                            left: -2rem;
                        }
                        100% {
                            left: calc(100% + 2rem);
                        }
                    }
                    .progress-animation {
                        animation: progress-shine 1s ease-in-out infinite;
                    }
                    .progress-animation-paused {
                        animation-play-state: paused;
                    }`}
			</style>

			{/* Full-width shine animation when at 0% */}
			{isAtZero && (
				<div
					className={cn(
						"absolute left-0 w-6 h-full blur-sm inset-y-0 progress-animation",
						"bg-white/100 shadow-lg",
					)}
				/>
			)}

			<ProgressPrimitive.Indicator
				data-slot='progress-indicator'
				className={cn(
					"h-full w-full flex-1 transition-all relative overflow-hidden",
					"bg-gradient-to-r from-cyan-500 via-sky-500 to-indigo-500 rounded-l-full",
				)}
				style={{ transform: `translateX(-${100 - progressValue}%)` }}
			>
				{/* Shine animation within the progress indicator (stops at 100%) */}
				{!isAtZero && (
					<div
						className={cn(
							"absolute left-0 w-6 h-full blur-sm inset-y-0",
							"bg-white/100 shadow-lg",
							isComplete ? "" : "progress-animation",
						)}
					/>
				)}
			</ProgressPrimitive.Indicator>
		</ProgressPrimitive.Root>
	);

	return (
		<div className='w-full flex items-center justify-center gap-3'>
			{progressBarElement}
			<span className='text-sm'>{progressValue}%</span>
		</div>
	);
}

export { Progress };
