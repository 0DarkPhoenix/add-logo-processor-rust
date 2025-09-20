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
	disabled = false,
	...props
}: FileInputProps) {
	const [selectedPath, setSelectedPath] = useState<string>(value);

	// Update internal state when value prop changes
	useEffect(() => {
		setSelectedPath(value);
	}, [value]);

	const handleBrowse = async () => {
		if (disabled) {
			return;
		}

		try {
			// Only use defaultPath for directories, not files
			let defaultPathToUse: string | undefined;
			if (directory && selectedPath) {
				defaultPathToUse = selectedPath;
			} else if (!directory && selectedPath) {
				// For files, extract just the directory
				const pathParts = selectedPath.split(/[/\\]/);
				if (pathParts.length > 1) {
					pathParts.pop(); // Remove filename
					defaultPathToUse = pathParts.join("\\");
				}
			}

			const result = await open({
				directory,
				multiple,
				defaultPath: defaultPathToUse,
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
		<div className='flex w-full items-end'>
			<input
				type='text'
				value={selectedPath}
				onChange={(e) => {
					const newPath = e.target.value;
					setSelectedPath(newPath);
					onChange?.(newPath);
				}}
				placeholder={placeholder}
				data-slot='input'
				title={selectedPath || placeholder}
				disabled={disabled}
				className={cn(
					"file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input h-9 flex-1 min-w-0 rounded-l-md border border-r-0 bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm leading-none",
					"focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
					"aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
					className,
				)}
				{...props}
			/>
			<button
				type='button'
				onClick={handleBrowse}
				disabled={disabled}
				className={cn(
					"border-input bg-muted hover:bg-muted/80 text-muted-foreground h-9 flex items-center justify-center rounded-r-md border px-3 text-sm font-medium transition-colors shrink-0 leading-none",
					"focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] focus-visible:outline-none",
					"disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
				)}
			>
				<svg
					xmlns='http://www.w3.org/2000/svg'
					height='20px'
					viewBox='0 -960 960 960'
					width='20px'
					fill='currentColor'
					aria-hidden='true'
				>
					<title>Folder open icon</title>
					<path d='M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h240l80 80h320q33 0 56.5 23.5T880-640H447l-80-80H160v480l96-320h684L837-217q-8 26-29.5 41.5T760-160H160Zm84-80h516l72-240H316l-72 240Zm0 0 72-240-72 240Zm-84-400v-80 80Z' />
				</svg>
			</button>
		</div>
	);
}

export { FileInput };
