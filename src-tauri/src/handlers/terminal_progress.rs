use std::io::{self, Write};
use std::time::Duration;

#[derive(Debug)]
pub struct TerminalProgressBar {
    width: usize,
    show_percentage: bool,
    show_rate: bool,
    show_eta: bool,
    show_elapsed: bool,
    is_initialized: bool,
}

impl TerminalProgressBar {
    pub fn new() -> Self {
        Self {
            width: 50,
            show_percentage: true,
            show_rate: true,
            show_eta: true,
            show_elapsed: true,
            is_initialized: false,
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

    pub fn init(&mut self) {
        if !self.is_initialized {
            // Save cursor position
            print!("\x1b[s");
            // Print an empty line for the progress bar
            println!();
            // Restore cursor position
            print!("\x1b[u");
            self.is_initialized = true;
            io::stdout().flush().unwrap();
        }
    }

    pub fn display(
        &mut self,
        current: usize,
        total: usize,
        status: &str,
        elapsed: Duration,
        rate: f64,
        eta: Option<Duration>,
    ) {
        // Initialize if not done yet
        self.init();

        // Save current cursor position
        print!("\x1b[s");

        // Move to the top line (line 1)
        print!("\x1b[1;1H");

        // Clear the entire line
        print!("\x1b[2K");

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

        // Print the complete progress line at the top
        print!("{}: {} {}", status, bar, info_string);

        // Restore cursor position
        print!("\x1b[u");

        // Flush to ensure immediate display
        io::stdout().flush().unwrap();
    }

    pub fn finish(&mut self, status: &str) {
        if self.is_initialized {
            // Save current cursor position
            print!("\x1b[s");

            // Move to the top line
            print!("\x1b[1;1H");

            // Clear the line and print completion message
            print!("\x1b[2K{}: Complete!", status);

            // Restore cursor position
            print!("\x1b[u");

            io::stdout().flush().unwrap();
            self.is_initialized = false;
        }
    }

    pub fn clear_line(&mut self) {
        if self.is_initialized {
            // Save current cursor position
            print!("\x1b[s");

            // Move to the top line and clear it
            print!("\x1b[1;1H\x1b[2K");

            // Restore cursor position
            print!("\x1b[u");

            io::stdout().flush().unwrap();
            self.is_initialized = false;
        }
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
