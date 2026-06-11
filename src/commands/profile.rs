use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::commands::listeners_sorted;

/// File written in the current directory to remember a project's ports.
pub const PROFILE_FILE: &str = ".portsmith.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub ports: Vec<u16>,
}

impl Profile {
    /// Loads the profile from the current directory, if one exists.
    pub fn load() -> Option<Profile> {
        let contents = fs::read_to_string(PROFILE_FILE).ok()?;
        match serde_json::from_str(&contents) {
            Ok(profile) => Some(profile),
            Err(e) => {
                eprintln!("Failed to parse {PROFILE_FILE}: {e}");
                None
            }
        }
    }
}

/// `portsmith profile save [ports...]` — record the project's ports.
///
/// With explicit ports, saves exactly those. With no arguments, captures every
/// port currently listening on the machine.
pub fn save(ports: Vec<u16>) {
    let mut ports = if ports.is_empty() {
        let Some(listeners) = listeners_sorted() else {
            return;
        };
        listeners.iter().map(|l| l.socket.port()).collect()
    } else {
        ports
    };
    ports.sort_unstable();
    ports.dedup();

    if ports.is_empty() {
        println!("No ports to save.");
        return;
    }

    let profile = Profile { ports };
    let json = match serde_json::to_string_pretty(&profile) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to serialize profile: {e}");
            return;
        }
    };

    if let Err(e) = fs::write(PROFILE_FILE, json) {
        eprintln!("Failed to write {PROFILE_FILE}: {e}");
        return;
    }

    println!(
        "Saved {} port(s) to {}: {:?}",
        profile.ports.len(),
        Path::new(PROFILE_FILE).display(),
        profile.ports
    );
}
