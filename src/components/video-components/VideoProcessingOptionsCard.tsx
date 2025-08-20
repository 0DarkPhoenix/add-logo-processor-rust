import { useId } from "react";
import { useFormContext } from "react-hook-form";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { videoCodecs, videoFormats } from "@/schema/videoForm";
import type { VideoSettings } from "@/types/VideoSettings";

export function VideoResizeDimensionsCard() {
	const { setValue, watch } = useFormContext<VideoSettings>();
	const baseId = useId();

	const minPixelCount = watch("minPixelCount");
	const shouldConvertFormat = watch("shouldConvertFormat");
	const format = watch("format");
	const shouldConvertCodec = watch("shouldConvertCodec");
	const codec = watch("codec");

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
							onValueChange={(value) =>
								setValue("format", value as VideoSettings["format"])
							}
							disabled={!shouldConvertFormat}
						>
							<SelectTrigger id={`${baseId}-format`} className='mt-1'>
								<SelectValue placeholder='Select output format...' />
							</SelectTrigger>
							<SelectContent>
								{videoFormats.map((format) => (
									<SelectItem key={format} value={format}>
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
							onValueChange={(value) =>
								setValue("codec", value as VideoSettings["codec"])
							}
							disabled={!shouldConvertCodec}
						>
							<SelectTrigger id={`${baseId}-codec`} className='mt-1'>
								<SelectValue placeholder='Select output codec...' />
							</SelectTrigger>
							<SelectContent>
								{videoCodecs.map((codec) => (
									<SelectItem key={codec} value={codec}>
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
