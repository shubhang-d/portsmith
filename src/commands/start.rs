use crate::commands::profile::Profile;
use crate::commands::{kill_pid, listeners_sorted};

/// `portsmith start` — load the saved profile and free any conflicting ports.
pub fn run() {
    let Some(profile) = Profile::load() else {
        eprintln!("No profile found. Run `portsmith profile save` first.");
        return;
    };

    let Some(listeners) = listeners_sorted() else {
        return;
    };

    for port in profile.ports {
        let conflicts: Vec<_> = listeners
            .iter()
            .filter(|l| l.socket.port() == port)
            .collect();

        if conflicts.is_empty() {
            println!("Port {port} is already free.");
            continue;
        }

        for listener in conflicts {
            let pid = listener.process.pid;
            let name = &listener.process.name;
            if kill_pid(pid) {
                println!("Freed port {port}: killed {name} (PID {pid}).");
            } else {
                eprintln!("Failed to free port {port}: could not kill {name} (PID {pid}).");
            }
        }
    }
}
