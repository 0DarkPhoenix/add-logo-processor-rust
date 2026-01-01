import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import { useForm } from "react-hook-form";
import { LogoConfiguratorCard } from "@/components/shared/LogoConfiguratorCard";
import ProgressBar from "@/components/shared/ProgressBar";
import { Button } from "@/components/ui/button";
import { VideoResizeDimensionsCard } from "@/components/video-components/VideoProcessingOptionsCard";
import { useSettings } from "@/contexts/SettingsContext";
import { DirectorySelectionCard } from "../../components/shared/DirectorySelectionCard";
import { Form } from "../../components/ui/form";
import { logoCorners, videoFormSchema } from "../../schema/videoForm";
import type { VideoSettings } from "../../types/VideoSettings";

export default function VideoProcessingPage() {
	const [isProcessing, setIsProcessing] = useState(false);
	const {
		videoSettings,
		supportedVideoFormats,
		supportedVideoCodecs,
		isInitialized,
		updateVideoSettings,
	} = useSettings();

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
			format: "",
			clearFilesInputDirectory: false,
			clearFilesOutputDirectory: false,
			overwriteExistingFilesOutputDirectory: false,
			shouldConvertCodec: false,
			codec: "",
		},
	});

	// Initialize form values once settings are loaded
	useEffect(() => {
		if (isInitialized && videoSettings) {
			form.reset(videoSettings);
		}
	}, [isInitialized, videoSettings, form.reset]);

	// Update context when form changes
	useEffect(() => {
		const subscription = form.watch((data) => {
			if (isInitialized) {
				updateVideoSettings(data as VideoSettings);
			}
		});
		return () => subscription.unsubscribe();
	}, [form, isInitialized, updateVideoSettings]);

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

	const handleCancelProcessing = async () => {
		try {
			await invoke("cancel_process");
			setIsProcessing(false);
		} catch (error) {
			console.error("Failed to cancel processing:", error);
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

						<VideoResizeDimensionsCard
							supportedVideoFormats={supportedVideoFormats}
							supportedVideoCodecs={supportedVideoCodecs}
						/>

						<LogoConfiguratorCard />

						<Button
							type={isProcessing ? "button" : "submit"}
							variant={isProcessing ? "destructive" : "default"}
							disabled={false}
							onClick={isProcessing ? handleCancelProcessing : undefined}
						>
							{isProcessing ? "Cancel processing" : "Process Videos"}
						</Button>
					</form>
				</Form>
			</div>
			<ProgressBar isProcessing={isProcessing} />
		</>
	);
}
