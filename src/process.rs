use crate::error::{MatarError, MatarResult};
use crate::sys_cmd::capture_output;
use nix::sys::signal::{self, Signal};
use nix::unistd::{Pid, Uid, getppid};
use std::collections::HashMap;
use std::fs;

// Import Tabled trait for automatic table generation
use tabled::Tabled;

/// Process information structure.
/// The #[tabled] attributes define the headers used by the 'tabled' crate.
#[derive(Debug, Clone, Tabled)]
pub struct ProcessInfo {
    #[tabled(rename = "PID")]
    pub pid: Pid,

    #[tabled(rename = "BINARY PATH")]
    pub path: String,

    #[tabled(rename = "COMMAND LINE")]
    pub command: String,
}

/// Regex-style trick to prevent a process from finding itself.
/// Example: "matar" becomes "[m]atar".
/// When 'ps | grep [m]atar' runs, the grep pattern doesn't match the string "[m]atar".
fn wrap_pattern(target: &str) -> String {
    let mut chars = target.chars();
    match chars.next() {
        Some(first) if first.is_alphabetic() => {
            format!("[{}]{}", first, chars.as_str())
        }
        _ => target.to_string(),
    }
}

fn get_process_details(pid: u32) -> MatarResult<(String, String)> {
    let path = fs::read_link(format!("/proc/{}/exe", pid))
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| MatarError::ProcessMetadata(pid, format!("Path unreadable: {}", e)))?;

    let command = fs::read_to_string(format!("/proc/{}/cmdline", pid))
        .map(|cmd| cmd.replace('\0', " ").trim().to_string())
        .map_err(|e| MatarError::ProcessMetadata(pid, format!("Cmdline unreadable: {}", e)))?;

    Ok((path, command))
}

/// Finds PIDs using functional chaining.
/// Didactic Note: We use .map() and .unwrap_or_default() to handle the Option
/// without needing explicit 'if let' or 'match' blocks.
pub fn find_target_pids(target_name: &str) -> MatarResult<Vec<ProcessInfo>> {
    let my_pid = std::process::id();
    let my_ppid = getppid().as_raw() as u32;
    let my_uid = Uid::current().to_string();
    let pattern = wrap_pattern(target_name);

    let mut found_map: HashMap<u32, (String, String)> = HashMap::new();

    // Strategy 1: pgrep
    let pgrep_pids: Vec<u32> = capture_output("pgrep", &["-u", &my_uid, "-f", &pattern])?
        .map(|out| {
            out.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .unwrap_or_default();

    for pid in pgrep_pids {
        let details = get_process_details(pid)
            .unwrap_or_else(|_| ("<Inaccessible>".into(), "<Unknown>".into()));
        found_map.insert(pid, details);
    }

    // Strategy 2: ps
    let ps_pids: Vec<u32> = capture_output("ps", &["-u", &my_uid, "-o", "pid=", "--no-headers"])?
        .map(|out| {
            out.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .unwrap_or_default();

    for pid in ps_pids {
        if !found_map.contains_key(&pid)
            && let Ok((path, cmd)) = get_process_details(pid)
            && (path.contains(target_name) || cmd.contains(target_name))
        {
            found_map.insert(pid, (path, cmd));
        }
    }

    let result = found_map
        .into_iter()
        .filter(|&(pid, _)| pid != my_pid && pid != my_ppid && pid > 1000)
        .map(|(pid, (path, command))| ProcessInfo {
            pid: Pid::from_raw(pid as i32),
            path,
            command,
        })
        .collect();

    Ok(result)
}

pub fn terminate_all(processes: &[ProcessInfo]) -> usize {
    processes
        .iter()
        .filter(|p| signal::kill(p.pid, Signal::SIGKILL).is_ok())
        .count()
}
