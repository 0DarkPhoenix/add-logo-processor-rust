use log::{info, warn};
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::handlers::progress_handler::ProgressManager;

lazy_static::lazy_static! {
    pub static ref PROCESS_MANAGER: Arc<Mutex<ProcessManager>> = Arc::new(Mutex::new(ProcessManager::new()));
}

pub struct ProcessManager {
    pub process_ids: HashMap<u64, u32>,
    next_id: u64,
    cancel_flag: Arc<AtomicBool>,
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            process_ids: HashMap::new(),
            next_id: 0,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Register a new process by its system PID and return its unique ID
    pub fn register_process_by_pid(pid: u32) -> u64 {
        let mut manager = PROCESS_MANAGER.lock().unwrap();
        let id = manager.next_id;
        manager.next_id += 1;
        manager.process_ids.insert(id, pid);
        info!(
            "Registered process with ID {} (PID: {}). Total active: {}",
            id,
            pid,
            manager.process_ids.len()
        );
        id
    }

    /// Remove a completed process by its unique ID
    pub fn unregister_process(id: u64) {
        let mut manager = PROCESS_MANAGER.lock().unwrap();
        if let Some(pid) = manager.process_ids.remove(&id) {
            info!(
                "Unregistered process with ID {} (PID: {}). Remaining: {}",
                id,
                pid,
                manager.process_ids.len()
            );
        } else {
            warn!(
                "Attempted to unregister non-existent process with ID {}",
                id
            );
        }
    }

    /// Request cancellation of all operations
    pub fn request_cancel() {
        let manager = PROCESS_MANAGER.lock().unwrap();
        manager.cancel_flag.store(true, Ordering::Relaxed);
        info!("Cancellation requested for all operations");
    }

    /// Check if cancellation has been requested
    pub fn is_cancelled() -> bool {
        let manager = PROCESS_MANAGER.lock().unwrap();
        manager.cancel_flag.load(Ordering::Relaxed)
    }

    /// Kill all active processes immediately using OS-level termination
    pub fn kill_all_processes() -> Result<(), Box<dyn Error>> {
        let mut manager = PROCESS_MANAGER.lock().unwrap();

        // Request cancellation
        manager.cancel_flag.store(true, Ordering::Relaxed);

        let process_count = manager.process_ids.len();
        if process_count == 0 {
            info!("No active processes to kill");
            return Ok(());
        }

        info!("Forcefully killing {} active processes", process_count);

        let mut errors = Vec::new();
        let mut killed_count = 0;

        // Kill all processes using OS-specific methods
        for (id, pid) in manager.process_ids.iter() {
            match Self::kill_process_by_pid(*pid) {
                Ok(_) => {
                    info!("Successfully killed process {} (PID: {})", id, pid);
                    killed_count += 1;
                }
                Err(e) => {
                    warn!("Failed to kill process {} (PID: {}): {}", id, pid, e);
                    errors.push(format!("Process {} (PID: {}): {}", id, pid, e));
                }
            }
        }

        // Clear the process list
        manager.process_ids.clear();

        if !errors.is_empty() {
            warn!(
                "Some processes had issues during cleanup: {}",
                errors.join("; ")
            );
        }

        info!(
            "Process cleanup complete. Killed: {}, Total processed: {}",
            killed_count, process_count
        );

        Ok(())
    }

    /// Clear all processes without killing (use after normal completion)
    pub fn clear() {
        let mut manager = PROCESS_MANAGER.lock().unwrap();
        manager.process_ids.clear();
        // Reset the cancel flag when clearing
        manager.cancel_flag.store(false, Ordering::Relaxed);
        info!("Process manager cleared and cancel flag reset");
    }

    /// Get the count of active processes
    pub fn active_process_count() -> usize {
        let manager = PROCESS_MANAGER.lock().unwrap();
        manager.process_ids.len()
    }

    /// Kill a process by its system PID using OS-specific methods
    #[cfg(target_os = "windows")]
    fn kill_process_by_pid(pid: u32) -> Result<(), Box<dyn Error>> {
        use std::process::Command;

        // Use taskkill with /F (force) flag on Windows
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "taskkill failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into())
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn kill_process_by_pid(pid: u32) -> Result<(), Box<dyn Error>> {
        use nix::sys::signal::{self, Signal};
        use nix::unistd::Pid;

        // Send SIGKILL on Unix-like systems
        signal::kill(Pid::from_raw(pid as i32), Signal::SIGKILL)?;
        Ok(())
    }
}

/// Custom error type for cancellation
#[derive(Debug)]
pub struct CancellationError;

impl std::fmt::Display for CancellationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation cancelled by user")
    }
}

impl Error for CancellationError {}

/// Check for cancellation and return an error if cancelled
pub fn check_cancelled() -> Result<(), Box<dyn Error + Send + Sync>> {
    if ProcessManager::is_cancelled() {
        ProgressManager::set_status("Operation cancelled".to_string());
        return Err(CancellationError.into());
    }
    Ok(())
}
