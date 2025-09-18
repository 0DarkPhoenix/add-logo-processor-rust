import * as ProgressPrimitive from "@radix-ui/react-progress";
import type * as React from "react";

import { cn } from "@/lib/utils";

interface ProgressProps extends React.ComponentProps<typeof ProgressPrimitive.Root> {
	value?: number;
}

function Progress({ className, value, ...props }: ProgressProps) {
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
                    }`}
			</style>

			<ProgressPrimitive.Indicator
				data-slot='progress-indicator'
				className={cn(
					"h-full w-full flex-1 transition-all relative overflow-hidden",
					"bg-gradient-to-r from-cyan-500 via-sky-500 to-indigo-500 rounded-l-full",
				)}
				style={{ transform: `translateX(-${100 - (value || 0)}%)` }}
			>
				<div
					className={cn(
						"absolute left-0 w-6 h-full blur-sm inset-y-0 progress-animation",
						"bg-white/100 shadow-lg",
					)}
				/>
			</ProgressPrimitive.Indicator>
		</ProgressPrimitive.Root>
	);

	return (
		<div className='w-full flex items-center justify-center gap-3'>
			{progressBarElement}
			<span className='text-sm'>{value || 0}%</span>
		</div>
	);
}

export { Progress };
