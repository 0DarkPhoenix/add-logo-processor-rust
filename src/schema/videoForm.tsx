import * as z from "zod";

export const videoFormSchema = z
	.object({
		inputDirectory: z.string({
			error: "Selecting an input directory is required.",
		}),
		outputDirectory: z.string({
			error: "Selecting an output directory is required.",
		}),
		searchChildFolders: z.boolean(),
		keepChildFoldersStructureInOutputDirectory: z.boolean(),
		minPixelCount: z.number().min(1, "Minimum pixel count must be at least 1"),
		addLogo: z.boolean(),
		logoPath: z.string().nullable(),
		logoScale: z
			.number()
			.min(1, "Logo scale must be at least 1")
			.max(100, "Logo scale can't be higher than 100"),
		logoXOffsetScale: z
			.number()
			.max(100, "Logo X offset scale can't be higher than 100"),
		logoYOffsetScale: z
			.number()
			.max(100, "Logo Y offset scale can't be higher than 100"),
		logoCorner: z.enum(["TopLeft", "TopRight", "BottomLeft", "BottomRight"]),
		shouldConvertFormat: z.boolean(),
		format: z.enum(["mp4", "mov", "mkv"]),
		clearFilesInputDirectory: z.boolean(),
		clearFilesOutputDirectory: z.boolean(),
		overwriteExistingFilesOutputDirectory: z.boolean(),
	})
	.refine(
		(data) => {
			if (data.addLogo && (!data.logoPath || data.logoPath.trim() === "")) {
				return false;
			}
			return true;
		},
		{
			message: "Logo path is required when adding a logo",
			path: ["logoPath"],
		},
	);
