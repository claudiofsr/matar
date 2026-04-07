mod cli;
mod error;
mod process;
mod sys_cmd;

use crate::error::MatarResult;
use std::{thread, time::Duration};

// Importing Tabled utilities
use tabled::{Table, settings::Style};

fn main() {
    if let Err(error) = run() {
        eprintln!("\n[!] Operation failed: {}", error);
        std::process::exit(1);
    }
}

fn run() -> MatarResult<()> {
    let args = cli::parse_args();
    let target = &args.target;

    println!("--- Analyzing processes for: '{}' ---", target);

    // 1. Identification
    let mut targets = process::find_target_pids(target)?;
    targets.sort_by_key(|p| p.pid);

    if targets.is_empty() {
        println!("No active processes found for '{}'.", target);
        return Ok(());
    }

    // 2. Display Table using the 'tabled' crate
    let mut table = Table::new(&targets);
    table.with(Style::rounded());

    println!("{}", table);

    // 3. Initial Termination
    let killed = process::terminate_all(&targets);
    println!("\nSIGKILL signal sent to {} process(es).", killed);

    if args.fast {
        println!("Fast mode enabled. Skipping verification.");
        return Ok(());
    }

    // 4. Verification Pass (Deep Clean)
    println!("Waiting for kernel confirmation (1s)...");
    thread::sleep(Duration::from_secs(1));

    let remaining = process::find_target_pids(target)?;

    if !remaining.is_empty() {
        println!(
            "Remnants detected: {}. Attempting final cleaning...",
            remaining.len()
        );
        process::terminate_all(&remaining);

        thread::sleep(Duration::from_millis(500));

        if !process::find_target_pids(target)?.is_empty() {
            println!("WARNING: Some processes could not be terminated (likely Zombies or IOWait).");
        } else {
            println!("SUCCESS: Deep cleaning completed.");
        }
    } else {
        println!(
            "SUCCESS: All processes associated with '{}' were eliminated.",
            target
        );
    }

    Ok(())
}
