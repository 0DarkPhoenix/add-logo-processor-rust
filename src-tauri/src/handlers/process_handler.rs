use log::{info, warn};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

lazy_static::lazy_static! {
    pub static ref PROCESS_MANAGER: Arc<Mutex<ProcessManager>> = Arc::new(Mutex::new(ProcessManager::new()));
}

pub struct ProcessManager {
    pub process_ids: HashMap<u64, u32>, // Map our ID to system PID
    next_id: u64,
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            process_ids: HashMap::new(),
            next_id: 0,
        }
    }

    /// Register a new FFmpeg process by its system PID and return its unique ID
    pub fn register_process_by_pid(pid: u32) -> u64 {
        let mut manager = PROCESS_MANAGER.lock().unwrap();
        let id = manager.next_id;
        manager.next_id += 1;
        manager.process_ids.insert(id, pid);
        info!(
            "Registered FFmpeg process with ID {} (PID: {}). Total active: {}",
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
                "Unregistered FFmpeg process with ID {} (PID: {}). Remaining: {}",
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

    /// Kill all active FFmpeg processes immediately using OS-level termination
    pub fn kill_all_processes() -> Result<(), Box<dyn Error>> {
        let mut manager = PROCESS_MANAGER.lock().unwrap();

        let process_count = manager.process_ids.len();
        if process_count == 0 {
            info!("No active FFmpeg processes to kill");
            return Ok(());
        }

        info!(
            "Forcefully killing {} active FFmpeg processes",
            process_count
        );

        let mut errors = Vec::new();
        let mut killed_count = 0;

        // Kill all processes using OS-specific methods
        for (id, pid) in manager.process_ids.iter() {
            match Self::kill_process_by_pid(*pid) {
                Ok(_) => {
                    info!("Successfully killed FFmpeg process {} (PID: {})", id, pid);
                    killed_count += 1;
                }
                Err(e) => {
                    warn!("Failed to kill FFmpeg process {} (PID: {}): {}", id, pid, e);
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
