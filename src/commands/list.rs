use crate::commands::listeners_sorted;

/// `portsmith list [port]` — show listening ports with their owning processes.
///
/// With a `port`, shows only the process bound to that exact port.
pub fn run(port: Option<u16>) {
    let Some(mut listeners) = listeners_sorted() else {
        return;
    };

    if let Some(port) = port {
        listeners.retain(|l| l.socket.port() == port);
        if listeners.is_empty() {
            println!("Nothing is listening on port {port}.");
            return;
        }
    } else if listeners.is_empty() {
        println!("No listening ports found.");
        return;
    }

    println!(
        "{:<8} {:<24} {:<8} {:<20}",
        "PROTO", "ADDRESS", "PID", "PROCESS"
    );
    println!("{}", "-".repeat(62));

    for listener in listeners {
        println!(
            "{:<8} {:<24} {:<8} {:<20}",
            format!("{:?}", listener.protocol),
            listener.socket.to_string(),
            listener.process.pid,
            listener.process.name
        );
    }
}
