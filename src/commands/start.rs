use crate::commands::check;
use crate::commands::detect::detect_project_ports;
use crate::commands::profile::Profile;
use crate::commands::{kill_pid, listeners_sorted};

/// `portsmith start` — get the project ready in one step.
///
/// Figures out which ports the project needs (a saved profile if there is one,
/// otherwise auto-detection), frees any that are currently taken, then reports
/// whether the backing services are up. This is the only command most people
/// need day to day.
pub fn run() {
    let ports = resolve_ports();
    if ports.is_empty() {
        crate::ui::warn_box(&[
            "No ports found for this project",
            "",
            "Looked through .env files and package.json but found",
            "nothing to free. That's normal outside a Node project.",
            "",
            "Try one of these:",
            "  • run portsmith from your project's root directory",
            "  • pin a port yourself:   portsmith save <port>",
        ]);
        return;
    }

    let Some(listeners) = listeners_sorted() else {
        return;
    };
    let self_pid = std::process::id();

    println!("Freeing project ports...");
    for port in ports {
        // Port 0 means "any free port"; it never identifies a real conflict.
        if port == 0 {
            continue;
        }

        let conflicts: Vec<_> = listeners
            .iter()
            .filter(|l| l.socket.port() == port && l.process.pid != self_pid)
            .collect();

        if conflicts.is_empty() {
            println!("  {port}\talready free");
            continue;
        }

        for listener in conflicts {
            let pid = listener.process.pid;
            let name = &listener.process.name;
            if kill_pid(pid) {
                println!("  {port}\tfreed (was {name}, PID {pid})");
            } else {
                println!("  {port}\tstill in use — couldn't kill {name} (PID {pid})");
            }
        }
    }

    // Backing services are reported for context but never killed.
    check::report();

    println!("Ready.");
}

/// Resolves the project's ports: a saved profile takes precedence (an explicit
/// override), otherwise they're auto-detected from the project files.
fn resolve_ports() -> Vec<u16> {
    let mut ports = if let Some(profile) = Profile::load() {
        println!("Using ports from .portsmith.json");
        profile.ports
    } else {
        let detected = detect_project_ports();
        for d in &detected {
            println!("Detected port {} from {}", d.port, d.source);
        }
        detected.into_iter().map(|d| d.port).collect()
    };

    ports.sort_unstable();
    ports.dedup();
    ports
}
