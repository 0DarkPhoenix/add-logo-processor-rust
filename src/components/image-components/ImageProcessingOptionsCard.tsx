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
import type { ImageSettings } from "@/types/ImageSettings";

export function ImageResizeDimensionsCard({
	supportedImageFormats,
}: {
	supportedImageFormats: string[];
}) {
	const { setValue, watch } = useFormContext<ImageSettings>();
	const baseId = useId();

	const minPixelCount = watch("minPixelCount");
	const shouldConvertFormat = watch("shouldConvertFormat");
	const format = watch("format");

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
								<SelectValue placeholder='Select format...' />
							</SelectTrigger>
							<SelectContent>
								{supportedImageFormats.map((format) => (
									<SelectItem key={format} value={format}>
										{format.toUpperCase()}
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
