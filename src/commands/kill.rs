use crate::commands::{kill_pid, listeners_sorted};

/// `portsmith kill <port>` — free a port by killing whatever is listening on it.
pub fn run(port: u16) {
    let Some(listeners) = listeners_sorted() else {
        return;
    };

    let matches: Vec<_> = listeners
        .into_iter()
        .filter(|l| l.socket.port() == port)
        .collect();

    if matches.is_empty() {
        println!("Nothing is listening on port {port}.");
        return;
    }

    for listener in matches {
        let pid = listener.process.pid;
        let name = &listener.process.name;
        if kill_pid(pid) {
            println!("Freed port {port}: killed {name} (PID {pid}).");
        } else {
            eprintln!("Failed to kill {name} (PID {pid}). Try running with higher privileges.");
        }
    }
}
