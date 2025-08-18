import { useId } from "react";
import { useFormContext } from "react-hook-form";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { FileInput } from "@/components/ui/path-input";
import { Switch } from "@/components/ui/switch";
import type { ImageSettings } from "@/types/ImageSettings";

export function DirectorySelectionCard() {
	const { setValue, watch } = useFormContext<ImageSettings>();
	const baseId = useId();

	const inputDirectory = watch("inputDirectory");
	const outputDirectory = watch("outputDirectory");
	const searchChildFolders = watch("searchChildFolders");
	const clearFilesInputDirectory = watch("clearFilesInputDirectory");
	const keepChildFoldersStructureInOutputDirectory = watch(
		"keepChildFoldersStructureInOutputDirectory",
	);
	const overwriteExistingFilesOutputDirectory = watch("overwriteExistingFilesOutputDirectory");
	const clearFilesOutputDirectory = watch("clearFilesOutputDirectory");

	return (
		<Card>
			<CardHeader>
				<CardTitle>Directory Settings</CardTitle>
			</CardHeader>
			<CardContent className='space-y-6'>
				{/* Input Directory Section */}
				<div className='space-y-4'>
					<div className='flex items-center gap-4'>
						<div className='flex flex-col gap-3'>
							<div className='flex items-center space-x-2'>
								<Switch
									id={`${baseId}-searchChildFolders`}
									checked={searchChildFolders}
									onCheckedChange={(checked) =>
										setValue("searchChildFolders", checked)
									}
								/>
								<Label htmlFor={`${baseId}-searchChildFolders`} className='text-sm'>
									Search child folders
								</Label>
							</div>
							<div className='flex items-center space-x-2'>
								<Switch
									id={`${baseId}-clearFilesInputDirectory`}
									checked={clearFilesInputDirectory}
									onCheckedChange={(checked) =>
										setValue("clearFilesInputDirectory", checked)
									}
								/>
								<Label
									htmlFor={`${baseId}-clearFilesInputDirectory`}
									className='text-sm'
								>
									Clear input directory
								</Label>
							</div>
						</div>
						<div className='flex-1'>
							<Label
								htmlFor={`${baseId}-inputDirectory`}
								className='text-sm font-medium'
							>
								Input Directory
							</Label>
							<FileInput
								id={`${baseId}-inputDirectory`}
								directory
								placeholder='Select input directory...'
								value={inputDirectory}
								onChange={(path) => setValue("inputDirectory", path || "")}
								className='mt-1'
							/>
						</div>
					</div>
				</div>

				{/* Output Directory Section */}
				<div className='space-y-4'>
					<div className='flex items-center gap-4'>
						<div className='flex flex-col gap-3'>
							<div className='flex items-center space-x-2'>
								<Switch
									id={`${baseId}-keepChildFoldersStructureInOutputDirectory`}
									checked={keepChildFoldersStructureInOutputDirectory}
									onCheckedChange={(checked) =>
										setValue(
											"keepChildFoldersStructureInOutputDirectory",
											checked,
										)
									}
								/>
								<Label
									htmlFor={`${baseId}-keepChildFoldersStructureInOutputDirectory`}
									className='text-sm'
								>
									Keep folder structure
								</Label>
							</div>
							<div className='flex items-center space-x-2'>
								<Switch
									id={`${baseId}-overwriteExistingFilesOutputDirectory`}
									checked={overwriteExistingFilesOutputDirectory}
									onCheckedChange={(checked) =>
										setValue("overwriteExistingFilesOutputDirectory", checked)
									}
								/>
								<Label
									htmlFor={`${baseId}-overwriteExistingFilesOutputDirectory`}
									className='text-sm'
								>
									Overwrite existing files
								</Label>
							</div>
							<div className='flex items-center space-x-2'>
								<Switch
									id={`${baseId}-clearFilesOutputDirectory`}
									checked={clearFilesOutputDirectory}
									onCheckedChange={(checked) =>
										setValue("clearFilesOutputDirectory", checked)
									}
								/>
								<Label
									htmlFor={`${baseId}-clearFilesOutputDirectory`}
									className='text-sm'
								>
									Clear output directory
								</Label>
							</div>
						</div>
						<div className='flex-1'>
							<Label
								htmlFor={`${baseId}-outputDirectory`}
								className='text-sm font-medium'
							>
								Output Directory
							</Label>
							<FileInput
								id={`${baseId}-outputDirectory`}
								directory
								placeholder='Select output directory...'
								value={outputDirectory}
								onChange={(path) => setValue("outputDirectory", path || "")}
								className='mt-1'
							/>
						</div>
					</div>
				</div>
			</CardContent>
		</Card>
	);
}
