use std::collections::HashSet;

use crate::commands::detect::detect_service_ports;
use crate::commands::listeners_sorted;

/// Result of inspecting the project's backing services.
pub enum CheckOutcome {
    /// No database/cache/broker was found to check.
    NoServices,
    /// Every detected service is listening.
    AllUp,
    /// At least one detected service is down.
    SomeDown,
}

/// `portsmith check` — verify the project's backing services are running.
///
/// Exits non-zero if any expected service is down, so it composes with shells:
/// `portsmith check && npm run dev`.
pub fn run() {
    match report() {
        CheckOutcome::NoServices => {
            println!("No backing services detected in .env files.");
            println!(
                "(check looks for connection URLs like DATABASE_URL and datastore *_PORT vars.)"
            );
        }
        CheckOutcome::AllUp => {}
        CheckOutcome::SomeDown => {
            eprintln!("\nSome services are not running. Start them before booting your app.");
            std::process::exit(1);
        }
    }
}

/// Prints the status of every detected backing service and returns the outcome.
/// Never kills anything and never exits — callers decide what to do.
pub fn report() -> CheckOutcome {
    let mut services = detect_service_ports();
    if services.is_empty() {
        return CheckOutcome::NoServices;
    }

    let Some(listeners) = listeners_sorted() else {
        return CheckOutcome::NoServices;
    };
    let active: HashSet<u16> = listeners.iter().map(|l| l.socket.port()).collect();

    // One line per port; the same service can appear as both a URL and a *_PORT.
    services.sort_by_key(|s| s.port);
    services.dedup_by_key(|s| s.port);

    println!("Services:");
    let mut all_up = true;
    for service in &services {
        if active.contains(&service.port) {
            println!(
                "  ✅ {} ({}) is running   [{}]",
                service.name, service.port, service.source
            );
        } else {
            println!(
                "  ⚠️  {} ({}) is NOT running   [{}]",
                service.name, service.port, service.source
            );
            all_up = false;
        }
    }

    if all_up {
        CheckOutcome::AllUp
    } else {
        CheckOutcome::SomeDown
    }
}
