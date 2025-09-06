use std::io::{self, Write};
use std::time::Duration;

#[derive(Debug)]
pub struct TerminalProgressBar {
    width: usize,
    show_percentage: bool,
    show_rate: bool,
    show_eta: bool,
    show_elapsed: bool,
}

impl TerminalProgressBar {
    pub fn new() -> Self {
        Self {
            width: 50,
            show_percentage: true,
            show_rate: true,
            show_eta: true,
            show_elapsed: true,
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn show_rate(mut self, show: bool) -> Self {
        self.show_rate = show;
        self
    }

    pub fn show_eta(mut self, show: bool) -> Self {
        self.show_eta = show;
        self
    }

    pub fn show_elapsed(mut self, show: bool) -> Self {
        self.show_elapsed = show;
        self
    }

    pub fn display(
        &self,
        current: usize,
        total: usize,
        status: &str,
        elapsed: Duration,
        rate: f64,
        eta: Option<Duration>,
    ) {
        // Move cursor to bottom and clear the line
        print!("\r");

        let percentage = if total > 0 {
            (current as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let filled_width = if total > 0 {
            ((current as f64 / total as f64) * self.width as f64) as usize
        } else {
            0
        };

        // Build the progress bar
        let mut bar = String::new();
        bar.push('[');

        for i in 0..self.width {
            if i < filled_width {
                bar.push('█');
            } else if i == filled_width && current < total {
                bar.push('▌');
            } else {
                bar.push(' ');
            }
        }
        bar.push(']');

        // Build the info string
        let mut info_parts = Vec::new();

        if self.show_percentage {
            info_parts.push(format!("{:.1}%", percentage));
        }

        info_parts.push(format!("{}/{}", current, total));

        if self.show_elapsed {
            info_parts.push(format!("elapsed: {}", Self::format_duration(elapsed)));
        }

        if self.show_rate && rate > 0.0 {
            info_parts.push(format!("{:.1} items/s", rate));
        }

        if self.show_eta {
            if let Some(eta_duration) = eta {
                info_parts.push(format!("ETA: {}", Self::format_duration(eta_duration)));
            }
        }

        let info_string = info_parts.join(" | ");

        // Print the complete progress line
        print!("{}: {} {}", status, bar, info_string);

        // Flush to ensure immediate display
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self, status: &str) {
        print!("\n\r{}: Complete!\n", status);
        io::stdout().flush().unwrap();
    }

    pub fn clear_line(&self) {
        print!("\r\x1b[K");
        io::stdout().flush().unwrap();
    }

    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl Default for TerminalProgressBar {
    fn default() -> Self {
        Self::new()
    }
}
