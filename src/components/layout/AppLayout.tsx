import { Camera, Video } from "lucide-react";
import type * as React from "react";
import { Button } from "../ui/button";

interface AppLayoutProps {
	children: React.ReactNode;
	currentPage?: "home" | "image" | "video";
	onPageChange?: (page: "home" | "image" | "video") => void;
}

export function AppLayout({ children, currentPage, onPageChange }: AppLayoutProps) {
	return (
		<div className='bg-background flex flex-col'>
			{onPageChange && (
				<nav className=' px-6 py-3 flex-shrink-0'>
					<div className='flex justify-center gap-4'>
						<Button
							variant={currentPage === "image" ? "purpleOutline" : "outline"}
							size='lg'
							onClick={() => onPageChange("image")}
							className='rounded-full'
						>
							<Camera className='mr-1 h-8 w-8' />
							Images
						</Button>
						<Button
							variant={currentPage === "video" ? "purpleOutline" : "outline"}
							size='lg'
							onClick={() => onPageChange("video")}
							className='rounded-full'
						>
							<Video className='mr-1 h-8 w-8' />
							Videos
						</Button>
					</div>
				</nav>
			)}
			<main className='flex-1 px-6 flex justify-center items-center'>
				<div className='container mx-auto h-full w-full'>{children}</div>
			</main>
		</div>
	);
}
