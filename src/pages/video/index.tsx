import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { LogoConfiguratorCard } from "@/components/shared/LogoConfiguratorCard";
import ProgressBar from "@/components/shared/ProgressBar";
import { Button } from "@/components/ui/button";
import { VideoResizeDimensionsCard } from "@/components/video-components/VideoProcessingOptionsCard";
import type { AppConfig } from "@/types/AppConfig";
import { DirectorySelectionCard } from "../../components/shared/DirectorySelectionCard";
import { Form } from "../../components/ui/form";
import { logoCorners, videoFormats, videoFormSchema } from "../../schema/videoForm";
import type { VideoSettings } from "../../types/VideoSettings";

export default function VideoProcessingPage() {
	const [isProcessing, setIsProcessing] = useState(false);

	const form = useForm<VideoSettings>({
		resolver: zodResolver(videoFormSchema),
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
			format: videoFormats[0],
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
					...config.videoSettings,
				});
			} catch (error) {
				console.error("Failed to load config:", error);
			}
		};

		loadConfig();
	}, [form]);

	const onSubmit = async (data: VideoSettings) => {
		setIsProcessing(true);
		try {
			await invoke("process_videos", { videoSettings: data });
		} catch (error) {
			console.error("Processing failed:", error);
		} finally {
			setIsProcessing(false);
		}
	};

	return (
		<>
			<div className='flex-1 flex align-center gap-6 min-w-0'>
				<Form {...form}>
					<form
						onSubmit={form.handleSubmit(onSubmit)}
						className='flex justify-center gap-6 h-full'
					>
						<DirectorySelectionCard />

						<VideoResizeDimensionsCard />

						<LogoConfiguratorCard />

						<Button type='submit' variant='default' disabled={isProcessing}>
							{isProcessing ? "Processing..." : "Process Videos"}
						</Button>
					</form>
				</Form>
			</div>
			<ProgressBar isProcessing={isProcessing} />
		</>
	);
}
