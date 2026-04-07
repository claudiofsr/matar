use crate::error::{MatarError, MatarResult};
use std::process::Command;

/// Executes a system command and captures its standard output.
///
/// Returns:
/// - `Ok(Some(String))`: Command succeeded and produced output.
/// - `Ok(None)`        : Command succeeded but found nothing (e.g., pgrep exit code 1).
/// - `Err(MatarError)` : Command failed to launch or returned an actual error.
pub fn capture_output(cmd: &str, args: &[&str]) -> MatarResult<Option<String>> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| MatarError::CommandExecution(cmd.to_string(), e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if output.status.success() {
        // Functional approach: Return None if string is empty, otherwise Some(stdout)
        Ok(if stdout.is_empty() {
            None
        } else {
            Some(stdout)
        })
    } else {
        let code = output.status.code().unwrap_or(-1);

        // In Linux, pgrep/pidof return exit code 1 specifically to signal
        // "No processes matched the criteria". This is not a failure of the tool.
        if (cmd == "pgrep" || cmd == "pidof") && code == 1 {
            return Ok(None);
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(MatarError::CommandExecution(
            cmd.to_string(),
            format!("Exit code {}: {}", code, stderr.trim()),
        ))
    }
}
