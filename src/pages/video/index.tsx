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
import { videoFormSchema } from "../../schema/videoForm";
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
		values: isInitialized && videoSettings ? videoSettings : undefined,
	});

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
		// Merge form data with existing videoSettings to preserve fields not in the form
		const mergedSettings: VideoSettings = {
			...videoSettings,
			...data,
		} as VideoSettings;

		setIsProcessing(true);
		try {
			await invoke("process_videos", { videoSettings: mergedSettings });
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
							videoSettings={videoSettings}
							updateVideoSettings={updateVideoSettings}
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
