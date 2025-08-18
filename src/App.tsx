import "./App.css";
import { useState } from "react";
import { AppLayout } from "./components/layout/AppLayout";
import { Button } from "./components/ui/button";
import ImageProcessingPage from "./pages/image/index";
import VideoProcessingPage from "./pages/video/index";

type Page = "home" | "image" | "video";

function App() {
	const [currentPage, setCurrentPage] = useState<Page>("home");

	const renderPage = () => {
		switch (currentPage) {
			case "image":
				return <ImageProcessingPage />;
			case "video":
				return <VideoProcessingPage />;
			default:
				return (
					<AppLayout>
						<div className="flex-1 flex items-center justify-center">
							<div className="text-center max-w-md">
								<h1 className="text-4xl font-bold mb-2">Add Logo Processor</h1>
								<p className="text-muted-foreground mb-8">
									Choose what type of media you want to process
								</p>

								<div className="flex flex-col gap-4">
									<Button
										size="lg"
										className="w-full h-16 text-lg"
										onClick={() => setCurrentPage("image")}
									>
										<div className="flex flex-col items-center gap-1">
											<span>📷</span>
											<span>Process Images</span>
										</div>
									</Button>

									<Button
										size="lg"
										variant="outline"
										className="w-full h-16 text-lg"
										onClick={() => setCurrentPage("video")}
									>
										<div className="flex flex-col items-center gap-1">
											<span>🎥</span>
											<span>Process Videos</span>
										</div>
									</Button>
								</div>

								<p className="text-xs text-muted-foreground mt-6">
									Resize media and add logos in bulk
								</p>
							</div>
						</div>
					</AppLayout>
				);
		}
	};

	return renderPage();
}

export default App;
