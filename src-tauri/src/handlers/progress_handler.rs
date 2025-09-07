use crate::handlers::terminal_progress::TerminalProgressBar;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub current: usize,
    pub total: usize,
    pub percentage: f64,
    pub elapsed_time: Duration,
    pub estimated_remaining: Option<Duration>,
    pub images_per_second: f64,
    pub status: String,
}

impl ProgressInfo {
    pub fn new(status: String, total: Option<usize>) -> Self {
        Self {
            current: 0,
            total: total.unwrap_or(0),
            percentage: 0.0,
            elapsed_time: Duration::from_secs(0),
            estimated_remaining: None,
            images_per_second: 0.0,
            status,
        }
    }
}

#[derive(Debug)]
pub struct ProgressTracker {
    info: Arc<Mutex<ProgressInfo>>,
    start_time: Instant,
    terminal_bar: Option<RefCell<TerminalProgressBar>>,
}

impl ProgressTracker {
    pub fn new(status: String, total: Option<usize>) -> Self {
        Self {
            info: Arc::new(Mutex::new(ProgressInfo::new(status, total))),
            start_time: Instant::now(),
            terminal_bar: None,
        }
    }

    pub fn with_terminal_display(mut self) -> Self {
        self.terminal_bar = Some(RefCell::new(TerminalProgressBar::new()));
        self
    }

    pub fn with_custom_terminal_bar(mut self, bar: TerminalProgressBar) -> Self {
        self.terminal_bar = Some(RefCell::new(bar));
        self
    }

    pub fn increment(&self) {
        let mut info = self.info.lock().unwrap();
        info.current += 1;
        self.update_calculations(&mut info);
        self.display_terminal_progress(&info);
    }

    pub fn set_current(&self, current: usize) {
        let mut info = self.info.lock().unwrap();
        info.current = current;
        self.update_calculations(&mut info);
        self.display_terminal_progress(&info);
    }

    pub fn set_status(&self, status: String) {
        let mut info = self.info.lock().unwrap();
        info.status = status;
        self.display_terminal_progress(&info);
    }

    pub fn set_total(&self, total: usize) {
        let mut info = self.info.lock().unwrap();
        info.total = total;
        self.display_terminal_progress(&info);
    }

    pub fn get_info(&self) -> ProgressInfo {
        self.info.lock().unwrap().clone()
    }

    pub fn is_complete(&self) -> bool {
        let info = self.info.lock().unwrap();
        info.current >= info.total
    }

    pub fn finish(&self) {
        if let Some(ref bar_cell) = self.terminal_bar {
            let info = self.info.lock().unwrap();
            bar_cell.borrow_mut().finish(&info.status);
        }
    }

    pub fn redraw_terminal_progress(&self) {
        if let Some(ref bar_cell) = self.terminal_bar {
            bar_cell.borrow().redraw();
        }
    }

    fn update_calculations(&self, info: &mut ProgressInfo) {
        info.elapsed_time = self.start_time.elapsed();
        info.percentage = if info.total > 0 {
            (info.current as f64 / info.total as f64) * 100.0
        } else {
            0.0
        };

        if info.elapsed_time.as_secs_f64() > 0.0 {
            info.images_per_second = info.current as f64 / info.elapsed_time.as_secs_f64();
        }

        if info.current > 0 && info.current < info.total && info.images_per_second > 0.0 {
            let remaining_images = info.total - info.current;
            let estimated_seconds = remaining_images as f64 / info.images_per_second;
            info.estimated_remaining = Some(Duration::from_secs_f64(estimated_seconds));
        } else {
            info.estimated_remaining = None;
        }
    }

    fn display_terminal_progress(&self, info: &ProgressInfo) {
        if let Some(ref bar_cell) = self.terminal_bar {
            bar_cell.borrow_mut().display(
                info.current,
                info.total,
                &info.status,
                info.elapsed_time,
                info.images_per_second,
                info.estimated_remaining,
            );
        }
    }
}

// Global progress manager
lazy_static::lazy_static! {
    static ref GLOBAL_PROGRESS: Arc<Mutex<Option<ProgressTracker>>> = Arc::new(Mutex::new(None));
}

pub struct ProgressManager;

impl ProgressManager {
    pub fn start_progress(status: String, total: Option<usize>) {
        let tracker = ProgressTracker::new(status, total);
        let mut global = GLOBAL_PROGRESS.lock().unwrap();
        *global = Some(tracker);
    }

    pub fn start_progress_with_terminal(status: String, total: Option<usize>) {
        let tracker = ProgressTracker::new(status, total).with_terminal_display();
        let mut global = GLOBAL_PROGRESS.lock().unwrap();
        *global = Some(tracker);
    }

    pub fn start_progress_with_custom_terminal(
        status: String,
        total: Option<usize>,
        bar: TerminalProgressBar,
    ) {
        let tracker = ProgressTracker::new(status, total).with_custom_terminal_bar(bar);
        let mut global = GLOBAL_PROGRESS.lock().unwrap();
        *global = Some(tracker);
    }

    pub fn increment_progress() {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.increment();
        }
    }

    pub fn set_progress(current: usize) {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.set_current(current);
        }
    }

    pub fn set_status(status: String) {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.set_status(status);
        }
    }

    pub fn set_total(total: usize) {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.set_total(total);
        }
    }

    pub fn get_progress() -> Option<ProgressInfo> {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        global.as_ref().map(|tracker| tracker.get_info())
    }

    pub fn is_complete() -> bool {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        global.as_ref().is_none_or(|tracker| tracker.is_complete())
    }

    pub fn finish_progress() {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.finish();
        }
    }

    pub fn clear_progress() {
        let mut global = GLOBAL_PROGRESS.lock().unwrap();
        *global = None;
    }

    pub fn redraw_progress() {
        let global = GLOBAL_PROGRESS.lock().unwrap();
        if let Some(tracker) = global.as_ref() {
            tracker.redraw_terminal_progress();
        }
    }
}
