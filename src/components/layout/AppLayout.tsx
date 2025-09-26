import type * as React from "react";
import { Button } from "../ui/button";

interface AppLayoutProps {
	children: React.ReactNode;
	currentPage?: "home" | "image" | "video";
	onPageChange?: (page: "home" | "image" | "video") => void;
}

export function AppLayout({ children, currentPage, onPageChange }: AppLayoutProps) {
	return (
		<div className='min-h-screen bg-background flex flex-col'>
			{onPageChange && (
				<nav className=' px-6 py-3 flex-shrink-0'>
					<div className='flex justify-center gap-8'>
						<Button
							variant={currentPage === "image" ? "default" : "outline"}
							size='sm'
							onClick={() => onPageChange("image")}
							className='rounded-full'
						>
							📷 Images
						</Button>
						<Button
							variant={currentPage === "video" ? "default" : "outline"}
							size='sm'
							onClick={() => onPageChange("video")}
							className='rounded-full'
						>
							🎥 Videos
						</Button>
					</div>
				</nav>
			)}
			<main className='flex-1 px-6 overflow-auto'>
				<div className='container mx-auto h-full'>{children}</div>
			</main>
		</div>
	);
}
