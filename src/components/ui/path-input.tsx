import { open } from "@tauri-apps/plugin-dialog";
import type * as React from "react";
import { useEffect, useState } from "react";
import { cn } from "@/lib/utils";

interface FileInputProps extends Omit<React.ComponentProps<"input">, "type" | "onChange"> {
	onChange?: (path: string | null) => void;
	placeholder?: string;
	accept?: string;
	directory?: boolean;
	multiple?: boolean;
	value?: string;
}

function FileInput({
	className,
	onChange,
	placeholder = "Select a file...",
	accept,
	directory = false,
	multiple = false,
	value = "",
	...props
}: FileInputProps) {
	const [selectedPath, setSelectedPath] = useState<string>(value);

	// Update internal state when value prop changes
	useEffect(() => {
		setSelectedPath(value);
	}, [value]);

	const handleBrowse = async () => {
		try {
			const result = await open({
				directory,
				multiple,
				filters: accept
					? [
							{
								name: "Files",
								extensions: accept
									.split(",")
									.map((ext) => ext.trim().replace(".", "")),
							},
						]
					: undefined,
			});

			if (result) {
				const path = Array.isArray(result) ? result[0] : result;
				setSelectedPath(path);
				onChange?.(path);
			}
		} catch (error) {
			console.error("Error opening file dialog:", error);
		}
	};

	return (
		<div className='flex w-full'>
			<input
				type='text'
				readOnly
				value={selectedPath}
				placeholder={placeholder}
				data-slot='input'
				className={cn(
					"file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-l-md border border-r-0 bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
					"focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
					"aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
					className,
				)}
				{...props}
			/>
			<button
				type='button'
				onClick={handleBrowse}
				className={cn(
					"border-input bg-muted hover:bg-muted/80 text-muted-foreground flex h-9 items-center rounded-r-md border px-3 text-sm font-medium transition-colors",
					"focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] focus-visible:outline-none",
				)}
			>
				Browse
			</button>
		</div>
	);
}

export { FileInput };
