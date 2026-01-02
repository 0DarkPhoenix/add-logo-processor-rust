import type * as React from "react";
import { cn } from "@/lib/utils";

interface SwitchProps {
	checked?: boolean;
	onCheckedChange?: (checked: boolean) => void;
	disabled?: boolean;
	className?: string;
	id?: string;
	label?: string;
	labelPosition?: "left" | "right";
	labelClassName?: string;
}

function Switch({
	checked = false,
	onCheckedChange,
	disabled = false,
	className,
	id,
	label,
	labelPosition = "right",
	labelClassName,
	...props
}: SwitchProps) {
	const handleClick = () => {
		if (!disabled && onCheckedChange) {
			onCheckedChange(!checked);
		}
	};

	const handleKeyDown = (e: React.KeyboardEvent) => {
		if (e.key === " " || e.key === "Enter") {
			e.preventDefault();
			handleClick();
		}
	};

	// Use hardcoded colors that work in both light and dark mode
	const getBackgroundColor = () => {
		if (checked) {
			return "#396cd8"; // Blue when checked
		}
		return "#6d6d6d"; // Light gray when unchecked
	};

	const getThumbColor = () => {
		return "#eeeeee"; // Always white thumb
	};

	const switchElement = (
		<button
			type='button'
			role='switch'
			aria-checked={checked}
			disabled={disabled}
			onClick={handleClick}
			onKeyDown={handleKeyDown}
			id={id}
			className={cn("switch-container", className)}
			style={{
				all: "unset",
				display: "inline-flex",
				alignItems: "center",
				width: "44px",
				height: "24px",
				minWidth: "44px",
				maxWidth: "44px",
				flexShrink: 0,
				backgroundColor: getBackgroundColor(),
				borderRadius: "12px",
				position: "relative",
				cursor: disabled ? "not-allowed" : "pointer",
				opacity: disabled ? 0.5 : 1,
				transition: "background-color 0.2s ease",
				border: "2px solid transparent",
				boxSizing: "border-box",
			}}
			{...props}
		>
			<div
				className='switch-thumb'
				style={{
					width: "18px",
					height: "18px",
					backgroundColor: getThumbColor(),
					borderRadius: "50%",
					position: "absolute",
					left: "2px",
					top: "50%",
					transform: `translateY(-50%) translateX(${checked ? "100%" : "0px"})`,
					transition: "transform 0.2s ease",
					boxShadow: "0 2px 4px rgba(0, 0, 0, 0.2)",
				}}
			/>
		</button>
	);

	if (!label) {
		return switchElement;
	}

	return (
		<div
			className={cn(
				"flex items-center gap-2",
				labelPosition === "left" && "flex-row-reverse",
				disabled && "opacity-50 cursor-not-allowed",
			)}
		>
			{switchElement}
			<label
				htmlFor={id}
				className={cn(
					"text-sm font-medium leading-none cursor-pointer select-none",
					disabled && "cursor-not-allowed",
					labelClassName,
				)}
			>
				{label}
			</label>
		</div>
	);
}

export { Switch };
