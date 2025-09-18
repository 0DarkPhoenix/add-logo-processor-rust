import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { ImageResizeDimensionsCard } from "@/components/image-components/ImageProcessingOptionsCard";
import { LogoConfiguratorCard } from "@/components/shared/LogoConfiguratorCard";
import ProgressBar from "@/components/shared/ProgressBar";
import { Button } from "@/components/ui/button";
import type { AppConfig } from "@/types/AppConfig";
import { AppLayout } from "../../components/layout/AppLayout";
import { DirectorySelectionCard } from "../../components/shared/DirectorySelectionCard";
import { Form } from "../../components/ui/form";
import { imageFormats, imageFormSchema, logoCorners } from "../../schema/imageForm";
import type { ImageSettings } from "../../types/ImageSettings";

export default function ImageProcessingPage() {
	const [isProcessing, setIsProcessing] = useState(false);

	const form = useForm<ImageSettings>({
		resolver: zodResolver(imageFormSchema),
		defaultValues: {
			inputDirectory: "",
			outputDirectory: "",
			searchChildFolders: false,
			keepChildFoldersStructureInOutputDirectory: false,
			minPixelCount: 1,
			addLogo: false,
			logoPath: null,
			logoScale: 10,
			logoXOffsetScale: 0,
			logoYOffsetScale: 0,
			logoCorner: logoCorners[0],
			shouldConvertFormat: false,
			format: imageFormats[0],
			clearFilesInputDirectory: false,
			clearFilesOutputDirectory: false,
			overwriteExistingFilesOutputDirectory: false,
		},
	});

	// Load config on component mount
	useEffect(() => {
		const loadConfig = async () => {
			try {
				const config: AppConfig = await invoke("load_config");
				// Reset form with config values, keeping the current form structure
				form.reset({
					...form.getValues(),
					...config.imageSettings,
				});
			} catch (error) {
				console.error("Failed to load config:", error);
			}
		};

		loadConfig();
	}, [form]);

	const onSubmit = async (data: ImageSettings) => {
		setIsProcessing(true);
		try {
			await invoke("process_images", { imageSettings: data });
		} catch (error) {
			console.error("Processing failed:", error);
		} finally {
			setIsProcessing(false);
		}
	};

	return (
		<AppLayout>
			<div className='flex-1 flex align-center gap-6 min-w-0'>
				<Form {...form}>
					<form
						onSubmit={form.handleSubmit(onSubmit)}
						className='flex justify-center gap-6 h-full'
					>
						<DirectorySelectionCard />

						<ImageResizeDimensionsCard />

						<LogoConfiguratorCard />

						<Button type='submit' variant='default' disabled={isProcessing}>
							{isProcessing ? "Processing..." : "Process Images"}
						</Button>
					</form>
				</Form>
			</div>
			<ProgressBar isProcessing={isProcessing} />
		</AppLayout>
	);
}
