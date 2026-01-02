import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { ImageResizeDimensionsCard } from "@/components/image-components/ImageProcessingOptionsCard";
import { LogoConfiguratorCard } from "@/components/shared/LogoConfiguratorCard";
import ProgressBar from "@/components/shared/ProgressBar";
import { Button } from "@/components/ui/button";
import { useSettings } from "@/contexts/SettingsContext";
import { DirectorySelectionCard } from "../../components/shared/DirectorySelectionCard";
import { Form } from "../../components/ui/form";
import { imageFormSchema } from "../../schema/imageForm";
import type { ImageSettings } from "../../types/ImageSettings";

export default function ImageProcessingPage() {
	const [isProcessing, setIsProcessing] = useState(false);
	const { imageSettings, supportedImageFormats, isInitialized, updateImageSettings } =
		useSettings();

	const form = useForm<ImageSettings>({
		resolver: zodResolver(imageFormSchema),
		values: isInitialized && imageSettings ? imageSettings : undefined,
	});

	// Update context when form changes
	useEffect(() => {
		const subscription = form.watch((data) => {
			if (isInitialized) {
				updateImageSettings(data as ImageSettings);
			}
		});
		return () => subscription.unsubscribe();
	}, [form, isInitialized, updateImageSettings]);

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
			<div className='flex-1 flex align-center gap-6'>
				<Form {...form}>
					<form
						onSubmit={form.handleSubmit(onSubmit)}
						className='flex justify-center gap-6 h-full'
					>
						<DirectorySelectionCard />

						<ImageResizeDimensionsCard supportedImageFormats={supportedImageFormats} />

						<LogoConfiguratorCard />

						<Button
							type={isProcessing ? "button" : "submit"}
							variant={isProcessing ? "destructive" : "default"}
							disabled={false}
							onClick={isProcessing ? handleCancelProcessing : undefined}
						>
							{isProcessing ? "Cancel processing" : "Process Images"}
						</Button>
					</form>
				</Form>
			</div>
			<ProgressBar isProcessing={isProcessing} />
		</>
	);
}
