import type * as React from "react";

interface AppLayoutProps {
	children: React.ReactNode;
}

export function AppLayout({ children }: AppLayoutProps) {
	return (
		<div className='min-h-screen bg-background flex flex-col'>
			<header className='border-b border-border px-6 py-4 flex-shrink-0'>
				<h1 className='text-2xl font-bold'>Add Logo Processor</h1>
			</header>
			<main className='flex-1 px-6 py-8 overflow-auto'>
				<div className='container mx-auto h-full'>{children}</div>
			</main>
		</div>
	);
}
