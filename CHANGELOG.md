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

## [2.0.0] - 2026-06-12

A simpler CLI built around a single command. **This release changes the command
surface** — see Changed/Removed before upgrading.

### Changed

- **`portsmith start` is now the all-in-one command.** It auto-detects the
  project's ports, frees any that are taken, and reports backing-service status.
  Previously it required a saved profile and only freed those ports. A
  `.portsmith.json` is still honored as an override when present.
- **`portsmith profile save` is now `portsmith save`** — the saving step is a
  top-level command instead of a nested subcommand.

### Removed

- **The `profile` subcommand.** Use `portsmith save` to pin ports and
  `portsmith start` to use them.

### Added

- `portsmith check` — verify the project's backing services (database, cache,
  broker) are running, detected from `.env` connection URLs (`DATABASE_URL`,
  `REDIS_URL`, …) and datastore `*_PORT` vars. Reports status without killing
  anything and exits non-zero if any service is down.
- Command aliases: `ls` for `list`, `free` for `kill`.
- Recursive, monorepo-aware port detection that walks subdirectories (`apps/web`,
  `packages/api`) while skipping `node_modules`, build output, and hidden/VCS dirs.
- Framework default ports: when scripts pin no port, the dev-server default is
  inferred from the project's dependencies (Next.js 3000, Vite 5173, Angular
  4200, Astro 4321, SvelteKit, Nuxt, CRA, Remix, Vue CLI, Gatsby). Explicit
  ports always take precedence.
- A boxed, colorized warning when no ports are found, with actionable next steps;
  respects `NO_COLOR` and non-TTY output.

## [1.0.0] - 2026-06-12

First published release.

### Added

- `portsmith list [port]` — list every listening port with its protocol, address,
  PID, and process name, or filter to a single port.
- `portsmith kill <port>` — free a port by stopping the process listening on it.
- `portsmith profile save [ports...]` — record a project's ports to
  `.portsmith.json`, either explicitly or auto-detected.
- `portsmith start` — load the saved profile and free any conflicting ports.
- Node-aware port detection from `package.json` scripts (`--port`, `-p`, inline
  `PORT=` env vars) and `.env*` files in the current directory.
- Datastore-aware safety: detection skips `DATABASE_PORT`, `REDIS_PORT`, `MYSQL_*`,
  `MONGO_*`, and similar keys so `start` never kills a dependency.
- Styled, colorized `--help` output with an ASCII banner that respects `NO_COLOR`
  and non-TTY output.
- Cross-platform support for macOS, Linux, and Windows.

[Unreleased]: https://github.com/shubhang-d/portman/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/shubhang-d/portman/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/shubhang-d/portman/releases/tag/v1.0.0
