use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::commands::detect::detect_project_ports;
use crate::commands::listeners_sorted;

/// File written in the current directory to remember a project's ports.
pub const PROFILE_FILE: &str = ".portsmith.json";

/// Resolves the ports to save when none were given on the command line: first
/// tries to detect them from the project, otherwise captures live listeners.
fn detect_or_capture() -> Vec<u16> {
    let detected = detect_project_ports();
    if !detected.is_empty() {
        for d in &detected {
            println!("Detected port {} from {}", d.port, d.source);
        }
        return detected.into_iter().map(|d| d.port).collect();
    }

    println!("No project ports found in .env or package.json; capturing live ports instead.");
    match listeners_sorted() {
        Some(listeners) => listeners.iter().map(|l| l.socket.port()).collect(),
        None => Vec::new(),
    }
}

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
/// With explicit ports, saves exactly those. With no arguments, auto-detects the
/// project's ports from `.env*` files and `package.json` scripts; if none are
/// found, falls back to capturing every port currently listening.
pub fn save(ports: Vec<u16>) {
    let mut ports = if !ports.is_empty() {
        ports
    } else {
        detect_or_capture()
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
