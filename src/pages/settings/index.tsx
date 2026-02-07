import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function SettingsPage() {
	const handleShowConfigInFolder = async () => {
		try {
			await invoke("show_config_in_folder");
		} catch (error) {
			console.error("Failed to show config in folder:", error);
		}
	};

	const handleShowLogInFolder = async () => {
		try {
			await invoke("show_log_in_folder");
		} catch (error) {
			console.error("Failed to show log in folder:", error);
		}
	};

	return (
		<div className='flex-1 flex-col align-center gap-6'>
			<h1 className='text-2xl font-bold'>Settings Page</h1>
			<Card>
				<CardHeader>
					<CardTitle>Advanced</CardTitle>
				</CardHeader>
				<CardContent className='space-y-6'>
					<div>
						<Button onClick={handleShowConfigInFolder}>Show config in folder</Button>
					</div>
					<div>
						<Button onClick={handleShowLogInFolder}>Show log in folder</Button>
					</div>
				</CardContent>
			</Card>
		</div>
	);
}
