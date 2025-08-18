import { useId } from "react";
import { useFormContext } from "react-hook-form";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { FileInput } from "@/components/ui/path-input";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import { imageFormats } from "@/schema/imageForm";
import type { ImageSettings } from "@/types/ImageSettings";

export function LogoConfiguratorCard() {
	const { setValue, watch } = useFormContext<ImageSettings>();
	const baseId = useId();

	const addLogo = watch("addLogo");
	const logoPath = watch("logoPath");
	const logoScale = watch("logoScale");
	const logoXOffsetScale = watch("logoXOffsetScale");
	const logoYOffsetScale = watch("logoYOffsetScale");
	const logoCorner = watch("logoCorner");

	return (
		<Card>
			<CardHeader>
				<CardTitle>Logo Settings</CardTitle>
			</CardHeader>
			<CardContent className='space-y-6'>
				{/* Enable Logo Section */}
				<div className='flex items-center space-x-2'>
					<Switch
						id={`${baseId}-addLogo`}
						checked={addLogo}
						onCheckedChange={(checked) => setValue("addLogo", checked)}
					/>
					<Label htmlFor={`${baseId}-addLogo`} className='text-sm'>
						Add logo to images
					</Label>
				</div>

				{/* Logo File Selection */}
				<div>
					<Label htmlFor={`${baseId}-logoPath`} className='text-sm font-medium'>
						Logo File
					</Label>
					<FileInput
						id={`${baseId}-logoPath`}
						placeholder='Select logo file...'
						accept={imageFormats.join(", ")}
						value={logoPath || ""}
						onChange={(path) => setValue("logoPath", path)}
						className='mt-1'
						disabled={!addLogo}
					/>
				</div>

				{/* Logo Position */}
				<div>
					<Label className='text-sm font-medium'>Logo Position</Label>
					<RadioGroup
						value={logoCorner}
						onValueChange={(value) =>
							setValue("logoCorner", value as ImageSettings["logoCorner"])
						}
						disabled={!addLogo}
						className='mt-2'
					>
						<div className='grid grid-cols-2 gap-4'>
							{/* Top Row */}
							<div className='flex items-center space-x-2'>
								<RadioGroupItem
									value='topLeft'
									id={`${baseId}-topLeft`}
									disabled={!addLogo}
								/>
								<Label htmlFor={`${baseId}-topLeft`} className='text-sm'>
									Top Left
								</Label>
							</div>
							<div className='flex items-center space-x-2'>
								<RadioGroupItem
									value='topRight'
									id={`${baseId}-topRight`}
									disabled={!addLogo}
								/>
								<Label htmlFor={`${baseId}-topRight`} className='text-sm'>
									Top Right
								</Label>
							</div>

							{/* Bottom Row */}
							<div className='flex items-center space-x-2'>
								<RadioGroupItem
									value='bottomLeft'
									id={`${baseId}-bottomLeft`}
									disabled={!addLogo}
								/>
								<Label htmlFor={`${baseId}-bottomLeft`} className='text-sm'>
									Bottom Left
								</Label>
							</div>
							<div className='flex items-center space-x-2'>
								<RadioGroupItem
									value='bottomRight'
									id={`${baseId}-bottomRight`}
									disabled={!addLogo}
								/>
								<Label htmlFor={`${baseId}-bottomRight`} className='text-sm'>
									Bottom Right
								</Label>
							</div>
						</div>
					</RadioGroup>
				</div>

				{/* Logo Scale and Offset Settings */}
				<div className='space-y-4'>
					<div>
						<Label htmlFor={`${baseId}-logoScale`} className='text-sm font-medium'>
							Logo Scale (%)
						</Label>
						<Input
							id={`${baseId}-logoScale`}
							type='number'
							min='1'
							max='100'
							placeholder='Enter logo scale percentage...'
							value={logoScale}
							onChange={(e) => setValue("logoScale", Number(e.target.value))}
							className='mt-1'
							disabled={!addLogo}
						/>
					</div>

					<div className='grid grid-cols-2 gap-4'>
						<div>
							<Label
								htmlFor={`${baseId}-logoXOffsetScale`}
								className='text-sm font-medium'
							>
								X Offset (%)
							</Label>
							<Input
								id={`${baseId}-logoXOffsetScale`}
								type='number'
								placeholder='X offset...'
								value={logoXOffsetScale}
								onChange={(e) =>
									setValue("logoXOffsetScale", Number(e.target.value))
								}
								className='mt-1'
								disabled={!addLogo}
							/>
						</div>
						<div>
							<Label
								htmlFor={`${baseId}-logoYOffsetScale`}
								className='text-sm font-medium'
							>
								Y Offset (%)
							</Label>
							<Input
								id={`${baseId}-logoYOffsetScale`}
								type='number'
								placeholder='Y offset...'
								value={logoYOffsetScale}
								onChange={(e) =>
									setValue("logoYOffsetScale", Number(e.target.value))
								}
								className='mt-1'
								disabled={!addLogo}
							/>
						</div>
					</div>
				</div>
			</CardContent>
		</Card>
	);
}
