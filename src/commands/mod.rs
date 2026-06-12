pub mod check;
pub mod detect;
pub mod kill;
pub mod list;
pub mod profile;
pub mod start;

use listeners::Listener;
use sysinfo::{Pid, ProcessesToUpdate, System};

/// Returns every listener currently bound on the machine, sorted by port.
///
/// Returns `None` (after printing an error) if the OS query fails, usually
/// because the command needs higher privileges.
pub fn listeners_sorted() -> Option<Vec<Listener>> {
    match listeners::get_all() {
        Ok(set) => {
            let mut v: Vec<Listener> = set.into_iter().collect();
            v.sort_by_key(|l| l.socket.port());
            Some(v)
        }
        Err(_) => {
            eprintln!(
                "Failed to retrieve system listening ports. Try running with higher privileges."
            );
            None
        }
    }
}

/// Kills the process with the given PID. Returns `true` if the kill succeeded.
pub fn kill_pid(pid: u32) -> bool {
    let pid = Pid::from_u32(pid);
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    match sys.process(pid) {
        Some(process) => process.kill(),
        None => false,
    }
}
