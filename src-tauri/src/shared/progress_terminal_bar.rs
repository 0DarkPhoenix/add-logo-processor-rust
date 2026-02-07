use std::io::{self, Write};
use std::time::Duration;

use crate::ProgressInfo;
use crossterm::terminal;
#[derive(Debug)]
pub struct TerminalProgressBar {
    width: usize,
    show_percentage: bool,
    show_rate: bool,
    show_eta: bool,
    show_elapsed: bool,
    is_displayed: bool,
    last_progress_line: String,
    scroll_region_active: bool,
}

impl TerminalProgressBar {
    pub fn new() -> Self {
        Self {
            width: 50,
            show_percentage: true,
            show_rate: true,
            show_eta: true,
            show_elapsed: true,
            is_displayed: false,
            last_progress_line: String::new(),
            scroll_region_active: false,
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

    pub fn display(&mut self, progress_info: &ProgressInfo) {
        let ProgressInfo {
            current,
            total,
            percentage,
            ref unit,
            elapsed_time,
            estimated_remaining,
            items_per_second,
            ref status,
            alternative_current,
            alternative_total,
            ref alternative_unit,
            ..
        } = *progress_info;

        let is_complete = current >= total && total > 0;

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

        info_parts.push(format!("{}/{} {}", current, total, unit));

        if self.show_rate && items_per_second > 0.0 {
            info_parts.push(format!("{:.1} {}/s", items_per_second, unit));
        }

        if alternative_total > 0 {
            info_parts.push(format!(
                "{}/{} {}",
                alternative_current, alternative_total, alternative_unit
            ));
        }

        if self.show_elapsed {
            info_parts.push(format!("elapsed: {}", Self::format_duration(elapsed_time)));
        }

        if self.show_eta {
            if let Some(eta_duration) = estimated_remaining {
                info_parts.push(format!("ETA: {}", Self::format_duration(eta_duration)));
            }
        }

        let info_string = info_parts.join(" | ");
        let progress_line = format!("{}: {} {}", status, bar, info_string);

        if is_complete {
            // For completion, clear the persistent progress bar and print final message
            if self.is_displayed {
                self.clear_persistent_progress();
            }
            println!("{} - Complete!", progress_line);
            self.is_displayed = false;
            self.last_progress_line.clear();
        } else {
            // Store the current progress line
            self.last_progress_line = progress_line.clone();

            if !self.is_displayed {
                // First time showing progress - reserve space at bottom
                self.setup_persistent_progress();
                self.is_displayed = true;
            }

            // Update the progress line at the bottom
            self.update_persistent_progress(&progress_line);
        }

        // Flush to ensure immediate display
        io::stdout().flush().unwrap();
    }

    pub fn finish(&mut self, status: &str) {
        if self.is_displayed {
            self.clear_persistent_progress();
        }
        println!("{}: Complete!", status);
        self.is_displayed = false;
        self.last_progress_line.clear();
        io::stdout().flush().unwrap();
    }

    pub fn clear_line(&mut self) {
        if self.is_displayed {
            self.clear_persistent_progress();
            self.is_displayed = false;
            self.last_progress_line.clear();
        }
    }

    // Method to redraw the progress bar (can be called externally when needed)
    pub fn redraw(&self) {
        if self.is_displayed && !self.last_progress_line.is_empty() {
            self.update_persistent_progress(&self.last_progress_line);
            io::stdout().flush().unwrap();
        }
    }

    fn setup_persistent_progress(&mut self) {
        if let Ok((_, rows)) = terminal::size() {
            if rows > 1 {
                // Reserve last line for the progress bar
                print!("\x1b[1;{}r", rows - 1); // set scroll region (1..rows-1)
                self.scroll_region_active = true;

                // Clear the reserved line
                print!("\x1b[{};1H", rows); // move to last line
                print!("\x1b[K"); // clear line
                print!("\x1b[1;1H"); // move to top to keep normal output in region
                return;
            }
        }

        // Fallback (no scroll region)
        print!("\x1b[s");
        print!("\x1b[999;999H");
        println!();
        print!("\x1b[u");
    }

    fn update_persistent_progress(&self, progress_line: &str) {
        if let Ok((_, rows)) = terminal::size() {
            if rows > 0 {
                print!("\x1b[s"); // Save cursor
                print!("\x1b[{};1H", rows); // Move to last line
                print!("\x1b[K"); // Clear line
                print!("{}", progress_line);
                print!("\x1b[u"); // Restore cursor
                return;
            }
        }

        // Fallback
        print!("\x1b[s");
        print!("\x1b[999;1H");
        print!("\x1b[K");
        print!("{}", progress_line);
        print!("\x1b[u");
    }

    fn clear_persistent_progress(&mut self) {
        if let Ok((_, rows)) = terminal::size() {
            if rows > 0 {
                print!("\x1b[s");
                print!("\x1b[{};1H", rows);
                print!("\x1b[K");
                print!("\x1b[u");
            }
        }

        if self.scroll_region_active {
            print!("\x1b[r"); // Reset scroll region
            self.scroll_region_active = false;
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
