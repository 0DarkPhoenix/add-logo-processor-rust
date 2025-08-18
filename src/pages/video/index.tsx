import { AppLayout } from "../../components/layout/AppLayout";
import { Button } from "../../components/ui/button";

export default function VideoProcessingPage() {
	return (
		<AppLayout>
			<div className="flex-1 flex flex-col gap-6 min-w-0">
				<div className="bg-card rounded-lg border p-6 flex-1 flex items-center justify-center">
					<div className="text-center">
						<h2 className="text-2xl font-semibold mb-4">Video Processing</h2>
						<p className="text-muted-foreground mb-6">
							Video processing functionality coming soon...
						</p>
						<Button variant="outline" onClick={() => window.history.back()}>
							Go Back
						</Button>
					</div>
				</div>
			</div>
		</AppLayout>
	);
}
