use std::fs;
use std::path::Path;

/// A port discovered in the project, with a human-readable source for display.
pub struct DetectedPort {
    pub port: u16,
    pub source: String,
}

/// `.env` files scanned in every directory, in priority order.
const ENV_FILES: [&str; 4] = [
    ".env",
    ".env.local",
    ".env.development",
    ".env.development.local",
];

/// Directory names we never descend into — dependency trees, build output, and
/// VCS metadata that would add noise (and a lot of false positives).
const SKIP_DIRS: [&str; 8] = [
    "node_modules",
    "target",
    "dist",
    "build",
    "out",
    "coverage",
    "vendor",
    ".git",
];

/// How deep to recurse. Deep enough for monorepos (`apps/web`, `packages/api`),
/// shallow enough to stay fast and avoid pathological trees.
const MAX_DEPTH: usize = 6;

/// Recursively scans the project for ports it is configured to use.
///
/// Walks the current directory and its subdirectories, reading the Node sources
/// (`.env*` files and `package.json` scripts) in each, while skipping dependency
/// and build directories. Returns ports in discovery order; callers dedupe.
pub fn detect_project_ports() -> Vec<DetectedPort> {
    let mut found = Vec::new();
    walk_dirs(Path::new("."), 0, &mut |dir| {
        for env_file in ENV_FILES {
            let path = dir.join(env_file);
            if let Ok(contents) = fs::read_to_string(&path) {
                for port in ports_from_env(&contents) {
                    found.push(DetectedPort {
                        port,
                        source: display_path(&path),
                    });
                }
            }
        }

        let package_json = dir.join("package.json");
        if let Ok(contents) = fs::read_to_string(&package_json) {
            found.extend(ports_from_package_json(&contents, &display_path(&package_json)));
        }
    });
    found
}

/// Recursively visits `dir` and its subdirectories (up to [`MAX_DEPTH`]), calling
/// `visit` for each, while skipping dependency, build, and hidden/VCS directories.
fn walk_dirs(dir: &Path, depth: usize, visit: &mut dyn FnMut(&Path)) {
    visit(dir);

    if depth >= MAX_DEPTH {
        return;
    }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        // Skip hidden directories (.git, .next, .turbo, …) and known noise dirs.
        if name.starts_with('.') || SKIP_DIRS.contains(&name) {
            continue;
        }
        walk_dirs(&path, depth + 1, visit);
    }
}

/// A backing service (database, cache, broker) the project expects to connect to.
pub struct DetectedService {
    pub port: u16,
    pub name: String,
    pub source: String,
}

/// Connection-URL schemes and the port their service listens on by default.
const URL_SCHEMES: [(&str, &str, u16); 8] = [
    ("postgresql", "postgres", 5432),
    ("postgres", "postgres", 5432),
    ("mysql", "mysql", 3306),
    ("mariadb", "mariadb", 3306),
    ("redis", "redis", 6379),
    ("rediss", "redis", 6379),
    ("mongodb", "mongodb", 27017),
    ("amqp", "rabbitmq", 5672),
];

/// Recursively detects the backing services a project connects to, by reading
/// `.env*` files for datastore `*_PORT` variables and connection URLs
/// (`DATABASE_URL`, `REDIS_URL`, …). These are services to *verify*, never kill.
pub fn detect_service_ports() -> Vec<DetectedService> {
    let mut found = Vec::new();
    walk_dirs(Path::new("."), 0, &mut |dir| {
        for env_file in ENV_FILES {
            let path = dir.join(env_file);
            if let Ok(contents) = fs::read_to_string(&path) {
                found.extend(services_from_env(&contents, &display_path(&path)));
            }
        }
    });
    found
}

/// Pulls service ports out of `.env` content: datastore `*_PORT` variables and
/// recognized connection URLs.
fn services_from_env(contents: &str, source: &str) -> Vec<DetectedService> {
    let mut services = Vec::new();
    for line in contents.lines() {
        let line = line.trim().strip_prefix("export ").unwrap_or(line.trim());
        if line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().split_whitespace().next().unwrap_or("");
        let value = value.trim_matches(['"', '\'']);

        // `DATABASE_PORT=5432`, `REDIS_PORT=6379`, …
        if key_mentions_port(key) && is_datastore_key(key) {
            if let Some(port) = parse_port(value) {
                services.push(DetectedService {
                    port,
                    name: service_label(key),
                    source: source.to_string(),
                });
            }
        // `DATABASE_URL=postgresql://…:5432/db`, `REDIS_URL=redis://…:6379`
        } else if let Some((name, port)) = port_from_connection_url(value) {
            services.push(DetectedService {
                port,
                name,
                source: source.to_string(),
            });
        }
    }
    services
}

/// Parses a connection URL into its `(service_name, port)`, using the scheme's
/// default port when the URL omits one. Returns `None` for unknown schemes.
fn port_from_connection_url(value: &str) -> Option<(String, u16)> {
    let (scheme, rest) = value.split_once("://")?;
    let (_, name, default_port) = URL_SCHEMES
        .iter()
        .find(|(s, _, _)| scheme.eq_ignore_ascii_case(s))?;

    // Authority is everything before the path/query, after any `user:pass@`.
    let authority = rest.split(['/', '?']).next().unwrap_or(rest);
    let host_port = authority.rsplit('@').next().unwrap_or(authority);
    let port = host_port
        .rsplit_once(':')
        .and_then(|(_, p)| parse_port(p))
        .unwrap_or(*default_port);

    Some((name.to_string(), port))
}

/// Turns a datastore env key into a friendly service name, e.g. `DATABASE_PORT`
/// → `database`, `REDIS_PORT` → `redis`.
fn service_label(key: &str) -> String {
    let key = key.to_ascii_lowercase();
    let trimmed = key.strip_suffix("_port").unwrap_or(&key);
    if trimmed.is_empty() {
        "service".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Renders a scanned path for display, dropping the leading `./` current-dir
/// component so sources read like `apps/web/package.json`.
fn display_path(path: &Path) -> String {
    path.strip_prefix(".").unwrap_or(path).display().to_string()
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

/// Well-known frameworks and the port their dev server uses by default, ordered
/// from most specific to most generic so meta-frameworks win over plain Vite.
const FRAMEWORKS: [(&str, &str, u16); 10] = [
    ("next", "Next.js", 3000),
    ("nuxt", "Nuxt", 3000),
    ("@angular/cli", "Angular", 4200),
    ("react-scripts", "Create React App", 3000),
    ("@vue/cli-service", "Vue CLI", 8080),
    ("gatsby", "Gatsby", 8000),
    ("@remix-run/dev", "Remix", 3000),
    ("astro", "Astro", 4321),
    ("@sveltejs/kit", "SvelteKit", 5173),
    ("vite", "Vite", 5173),
];

/// Extracts ports from a `package.json`.
///
/// First scans every `scripts` value for explicit ports (`next dev -p 3000`,
/// `vite --port=5173`, `cross-env PORT=4000 …`). If the scripts pin no port, it
/// falls back to the default port of whichever framework the project depends on
/// — so a plain `"dev": "next dev"` still yields 3000.
fn ports_from_package_json(contents: &str, source: &str) -> Vec<DetectedPort> {
    let mut found = Vec::new();
    let Ok(json) = serde_json::from_str::<serde_json::Value>(contents) else {
        return found;
    };

    let mut explicit = Vec::new();
    if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
        for command in scripts.values().filter_map(|v| v.as_str()) {
            explicit.extend(ports_from_command(command));
        }
    }

    if explicit.is_empty() {
        // No port pinned in the scripts — infer it from the framework in use.
        if let Some((label, port)) = framework_default(&json) {
            found.push(DetectedPort {
                port,
                source: format!("{label} default ({source})"),
            });
        }
    } else {
        for port in explicit {
            found.push(DetectedPort {
                port,
                source: source.to_string(),
            });
        }
    }

    found
}

/// Returns the `(label, default_port)` of the first known framework listed in the
/// project's `dependencies` or `devDependencies`, or `None` if none match.
fn framework_default(json: &serde_json::Value) -> Option<(&'static str, u16)> {
    let has_dep = |name: &str| {
        ["dependencies", "devDependencies"].iter().any(|section| {
            json.get(section)
                .and_then(|d| d.as_object())
                .is_some_and(|deps| deps.contains_key(name))
        })
    };
    FRAMEWORKS
        .iter()
        .find(|(pkg, _, _)| has_dep(pkg))
        .map(|(_, label, port)| (*label, *port))
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
