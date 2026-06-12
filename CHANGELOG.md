# Changelog

All notable changes to **portsmith** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- `portsmith watch` — live-updating view of port activity
- Parse `next.config.js` / `vite.config.ts` for ports
- `--json` output for scripting
- Prebuilt binaries and a one-line installer for every platform
- Homebrew tap and `npm` wrapper

## [1.0.0] - 2026-06-12

First stable release.

### Added

- `portsmith list [port]` — list every listening port with its protocol, address,
  PID, and process name, or filter to a single port.
- `portsmith kill <port>` — free a port by stopping the process listening on it.
- `portsmith profile save [ports...]` — record a project's ports to `.portsmith.json`,
  either explicitly or auto-detected.
- `portsmith start` — load the saved profile and free any conflicting ports so the
  project can boot cleanly.
- Node-aware port detection from `package.json` scripts (`--port`, `-p`, inline
  `PORT=` env vars) and `.env*` files.
- Datastore-aware safety: detection skips `DATABASE_PORT`, `REDIS_PORT`, `MYSQL_*`,
  `MONGO_*`, and similar keys so `start` never kills a dependency.
- Styled, colorized `--help` output with an ASCII banner and usage examples that
  respects `NO_COLOR` and non-TTY output.
- Cross-platform support for macOS, Linux, and Windows.

[Unreleased]: https://github.com/shubhang-d/portman/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/shubhang-d/portman/releases/tag/v1.0.0
