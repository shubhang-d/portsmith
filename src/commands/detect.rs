use std::fs;

/// A port discovered in the project, with a human-readable source for display.
pub struct DetectedPort {
    pub port: u16,
    pub source: String,
}

/// Scans the current directory for ports the project is configured to use.
///
/// Looks at the common Node sources: `.env*` files and `package.json` scripts.
/// Returns ports in discovery order; callers are expected to dedupe.
pub fn detect_project_ports() -> Vec<DetectedPort> {
    let mut found = Vec::new();

    for env_file in [
        ".env",
        ".env.local",
        ".env.development",
        ".env.development.local",
    ] {
        if let Ok(contents) = fs::read_to_string(env_file) {
            for port in ports_from_env(&contents) {
                found.push(DetectedPort {
                    port,
                    source: env_file.to_string(),
                });
            }
        }
    }

    if let Ok(contents) = fs::read_to_string("package.json") {
        for port in ports_from_package_json(&contents) {
            found.push(DetectedPort {
                port,
                source: "package.json".to_string(),
            });
        }
    }

    found
}

/// Extracts ports from `.env` style content: any `KEY=VALUE` line whose key
/// mentions PORT and whose value is a number, e.g. `PORT=3000`, `VITE_PORT=5173`.
fn ports_from_env(contents: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    for line in contents.lines() {
        let line = line.trim().strip_prefix("export ").unwrap_or(line.trim());
        if line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if !key_mentions_port(key) || is_datastore_key(key) {
            continue;
        }
        // Take the first token and strip surrounding quotes / inline comments.
        let value = value.trim().split_whitespace().next().unwrap_or("");
        let value = value.trim_matches(['"', '\'']);
        if let Some(port) = parse_port(value) {
            ports.push(port);
        }
    }
    ports
}

/// Extracts ports from a `package.json` by scanning every `scripts` value for
/// port flags and inline env vars, e.g. `next dev -p 3000`, `vite --port=5173`,
/// `cross-env PORT=4000 node server.js`.
fn ports_from_package_json(contents: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    let Ok(json) = serde_json::from_str::<serde_json::Value>(contents) else {
        return ports;
    };
    let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) else {
        return ports;
    };
    for command in scripts.values().filter_map(|v| v.as_str()) {
        ports.extend(ports_from_command(command));
    }
    ports
}

/// Scans a shell command string for ports specified as flags (`--port`, `-p`)
/// or inline env assignments (`PORT=3000`).
fn ports_from_command(command: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    let tokens: Vec<&str> = command.split_whitespace().collect();
    for (i, token) in tokens.iter().enumerate() {
        if let Some((key, value)) = token.split_once('=') {
            // `--port=3000`, `-p=3000`, `PORT=3000`
            if is_port_flag(key) || key_mentions_port(key) {
                if let Some(port) = parse_port(value) {
                    ports.push(port);
                }
            }
        } else if is_port_flag(token) {
            // `--port 3000`, `-p 3000` — value is the next token
            if let Some(port) = tokens.get(i + 1).and_then(|v| parse_port(v)) {
                ports.push(port);
            }
        }
    }
    ports
}

/// True for the CLI port flags used by Node dev servers.
fn is_port_flag(token: &str) -> bool {
    matches!(token, "--port" | "-p" | "-P")
}

/// True if an env-var key names a port, e.g. `PORT`, `API_PORT`, `VITE_PORT`.
fn key_mentions_port(key: &str) -> bool {
    key.to_ascii_uppercase().contains("PORT")
}

/// True if the key refers to a datastore/service you connect *to* rather than a
/// dev server you run — those should never be saved, since `start` would kill
/// the database/cache the project depends on.
fn is_datastore_key(key: &str) -> bool {
    const DATASTORES: [&str; 8] = [
        "DB", "DATABASE", "POSTGRES", "MYSQL", "MONGO", "REDIS", "SQL", "RABBIT",
    ];
    let key = key.to_ascii_uppercase();
    DATASTORES.iter().any(|name| key.contains(name))
}

/// Parses a string into a valid, non-zero port number.
fn parse_port(value: &str) -> Option<u16> {
    match value.parse::<u16>() {
        Ok(port) if port != 0 => Some(port),
        _ => None,
    }
}
