import { invoke } from "@tauri-apps/api/core";
import { createContext, type ReactNode, useContext, useEffect, useState } from "react";
import type { AppConfig } from "@/types/AppConfig";
import type { ImageSettings } from "@/types/ImageSettings";
import type { VideoSettings } from "@/types/VideoSettings";

interface SettingsContextType {
	imageSettings: ImageSettings | null;
	videoSettings: VideoSettings | null;
	supportedImageFormats: string[];
	supportedVideoFormats: string[];
	supportedVideoCodecs: string[];
	isInitialized: boolean;
	updateImageSettings: (settings: ImageSettings) => void;
	updateVideoSettings: (settings: VideoSettings) => void;
}

const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export function SettingsProvider({ children }: { children: ReactNode }) {
	const [imageSettings, setImageSettings] = useState<ImageSettings | null>(null);
	const [videoSettings, setVideoSettings] = useState<VideoSettings | null>(null);
	const [supportedImageFormats, setSupportedImageFormats] = useState<string[]>([]);
	const [supportedVideoFormats, setSupportedVideoFormats] = useState<string[]>([]);
	const [supportedVideoCodecs, setSupportedVideoCodecs] = useState<string[]>([]);
	const [isInitialized, setIsInitialized] = useState(false);

	useEffect(() => {
		const loadConfig = async () => {
			try {
				const [config, imageFormats, videoFormats, videoCodecs]: [
					AppConfig,
					string[],
					string[],
					string[],
				] = await Promise.all([
					invoke<AppConfig>("load_config"),
					invoke<string[]>("get_supported_image_formats"),
					invoke<string[]>("get_supported_video_formats"),
					invoke<string[]>("get_supported_video_codecs"),
				]);

				setImageSettings(config.imageSettings);
				setVideoSettings(config.videoSettings);
				setSupportedImageFormats(imageFormats);
				setSupportedVideoFormats(videoFormats);
				setSupportedVideoCodecs(videoCodecs);
				setIsInitialized(true);
			} catch (error) {
				console.error("Failed to load config:", error);
			}
		};

		loadConfig();
	}, []);

	const updateImageSettings = (settings: ImageSettings) => {
		setImageSettings(settings);
	};

	const updateVideoSettings = (settings: VideoSettings) => {
		setVideoSettings(settings);
	};

	return (
		<SettingsContext.Provider
			value={{
				imageSettings,
				videoSettings,
				supportedImageFormats,
				supportedVideoFormats,
				supportedVideoCodecs,
				isInitialized,
				updateImageSettings,
				updateVideoSettings,
			}}
		>
			{children}
		</SettingsContext.Provider>
	);
}

export function useSettings() {
	const context = useContext(SettingsContext);
	if (context === undefined) {
		throw new Error("useSettings must be used within a SettingsProvider");
	}
	return context;
}
