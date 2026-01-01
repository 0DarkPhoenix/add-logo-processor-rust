import * as z from "zod";
import type { ImageSettings } from "../types/ImageSettings";

type LogoCorner = ImageSettings["logoCorner"];
export const logoCorners: LogoCorner[] = [
	"topLeft",
	"topRight",
	"bottomLeft",
	"bottomRight",
] as const;

export const imageFormSchema = z
	.object({
		inputDirectory: z.string({
			error: "Selecting an input directory is required.",
		}),
		outputDirectory: z.string({
			error: "Selecting an output directory is required.",
		}),
		searchChildFolders: z.boolean(),
		keepChildFoldersStructureInOutputDirectory: z.boolean(),
		minPixelCount: z.number().min(1, "Minimum pixel count can't be lower than 1"),
		addLogo: z.boolean(),
		logoPath: z.string().nullable(),
		logoScale: z
			.number()
			.min(1, "Logo scale can't be lower than 1")
			.max(100, "Logo scale can't be higher than 100"),
		logoXOffsetScale: z.number().max(100, "Logo X offset scale can't be higher than 100"),
		logoYOffsetScale: z.number().max(100, "Logo Y offset scale can't be higher than 100"),
		logoCorner: z.enum(logoCorners),
		shouldConvertFormat: z.boolean(),
		format: z.string({ error: "Selecting a format is required." }),
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
			error: "Logo path is required when adding a logo",
			path: ["logoPath"],
		},
	);
