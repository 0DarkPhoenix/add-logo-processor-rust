import { useId } from "react";
import { useFormContext } from "react-hook-form";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectSeparator,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select-favorite";
import { Switch } from "@/components/ui/switch";
import type { VideoSettings } from "@/types/VideoSettings";

interface VideoResizeDimensionsCardProps {
	supportedVideoFormats: string[];
	supportedVideoCodecs: string[];
	videoSettings: VideoSettings | null;
	updateVideoSettings: (settings: VideoSettings) => void;
}

export function VideoResizeDimensionsCard({
	supportedVideoFormats,
	supportedVideoCodecs,
	videoSettings,
	updateVideoSettings,
}: VideoResizeDimensionsCardProps) {
	const { setValue, watch } = useFormContext<VideoSettings>();
	const baseId = useId();

	const minPixelCount = watch("minPixelCount");
	const shouldConvertFormat = watch("shouldConvertFormat");
	const format = watch("format");
	const shouldConvertCodec = watch("shouldConvertCodec");
	const codec = watch("codec");

	const favoriteFormats = videoSettings?.formatFavoriteList || [];
	const favoriteCodecs = videoSettings?.codecFavoriteList || [];

	const handleToggleFavoriteFormat = (format: string) => {
		if (!videoSettings) {
			return;
		}

		let updatedFavorites: string[];

		if (videoSettings.formatFavoriteList.includes(format)) {
			// Remove from favorites
			updatedFavorites = videoSettings.formatFavoriteList.filter((f) => f !== format);
		} else {
			// Add to favorites
			updatedFavorites = [...videoSettings.formatFavoriteList, format];
		}

		updateVideoSettings({
			...videoSettings,
			formatFavoriteList: updatedFavorites,
		});
	};

	const handleToggleFavoriteCodec = (codec: string) => {
		if (!videoSettings) {
			return;
		}

		let updatedFavorites: string[];

		if (videoSettings.codecFavoriteList.includes(codec)) {
			// Remove from favorites
			updatedFavorites = videoSettings.codecFavoriteList.filter((f) => f !== codec);
		} else {
			// Add to favorites
			updatedFavorites = [...videoSettings.codecFavoriteList, codec];
		}

		updateVideoSettings({
			...videoSettings,
			codecFavoriteList: updatedFavorites,
		});
	};

	return (
		<Card>
			<CardHeader>
				<CardTitle>Resize Dimensions</CardTitle>
			</CardHeader>
			<CardContent className='space-y-6'>
				<div>
					<Label htmlFor={`${baseId}-minPixelCount`} className='text-sm font-medium'>
						Minimum Pixel Count
					</Label>
					<Input
						id={`${baseId}-minPixelCount`}
						type='number'
						min='1'
						placeholder='Enter minimum pixel count...'
						value={minPixelCount}
						onChange={(e) => setValue("minPixelCount", Number(e.target.value))}
						className='mt-1'
					/>
				</div>

				{/* Format Conversion Section */}
				<div className='space-y-4'>
					<div className='flex items-center space-x-2'>
						<Switch
							id={`${baseId}-shouldConvertFormat`}
							checked={shouldConvertFormat}
							onCheckedChange={(checked) => setValue("shouldConvertFormat", checked)}
							label='Convert format'
						/>
					</div>

					<div>
						<Label htmlFor={`${baseId}-format`} className='text-sm font-medium'>
							Output Format
						</Label>
						<Select
							value={format}
							onValueChange={(value) => {
								// Guard against empty string on initial render
								if (value) {
									setValue("format", value);
								}
							}}
							disabled={!shouldConvertFormat}
						>
							<SelectTrigger id={`${baseId}-format`} className='mt-1'>
								<SelectValue placeholder='Select output format...' />
							</SelectTrigger>
							<SelectContent>
								{/* Render favorites first */}
								{favoriteFormats.map((format) => (
									<SelectItem
										key={`favorite-${format}`}
										value={format}
										isFavorite={true}
										onFavorite={handleToggleFavoriteFormat}
									>
										{format.toUpperCase()}
									</SelectItem>
								))}

								{/* Optional separator */}
								{favoriteFormats.length > 0 && <SelectSeparator />}

								{/* Regular items */}
								{supportedVideoFormats
									.filter((format) => !favoriteFormats.includes(format))
									.map((format) => (
										<SelectItem
											key={`item-${format}`}
											value={format}
											isFavorite={favoriteFormats.includes(format)}
											onFavorite={handleToggleFavoriteFormat}
										>
											{format.toUpperCase()}
										</SelectItem>
									))}
							</SelectContent>
						</Select>
					</div>
				</div>

				{/* Codec Conversion Section */}
				<div className='space-y-4'>
					<div className='flex items-center space-x-2'>
						<Switch
							id={`${baseId}-shouldConvertCodec`}
							checked={shouldConvertCodec}
							onCheckedChange={(checked) => setValue("shouldConvertCodec", checked)}
							label='Convert codec'
						/>
					</div>

					<div>
						<Label htmlFor={`${baseId}-codec`} className='text-sm font-medium'>
							Output Codec
						</Label>
						<Select
							value={codec}
							onValueChange={(value) => {
								// Guard against empty string on initial render
								if (value) {
									setValue("codec", value);
								}
							}}
							disabled={!shouldConvertCodec}
						>
							<SelectTrigger id={`${baseId}-codec`} className='mt-1'>
								<SelectValue placeholder='Select output codec...' />
							</SelectTrigger>
							<SelectContent>
								{/* Render favorites first */}
								{favoriteCodecs.map((format) => (
									<SelectItem
										key={`favorite-${format}`}
										value={format}
										isFavorite={true}
										onFavorite={handleToggleFavoriteFormat}
									>
										{format.toUpperCase()}
									</SelectItem>
								))}

								{/* Optional separator */}
								{favoriteCodecs.length > 0 && <SelectSeparator />}

								{/* Regular items */}
								{supportedVideoCodecs
									.filter((codec) => !favoriteCodecs.includes(codec))
									.map((codec) => (
										<SelectItem
											key={`item-${codec}`}
											value={codec}
											isFavorite={favoriteCodecs.includes(codec)}
											onFavorite={handleToggleFavoriteCodec}
										>
											{codec.toUpperCase()}
										</SelectItem>
									))}
							</SelectContent>
						</Select>
					</div>
				</div>
			</CardContent>
		</Card>
	);
}
