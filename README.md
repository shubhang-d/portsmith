<div align="center">

```
┌─┐┌─┐┬─┐┌┬┐┌─┐┌┬┐┬┌┬┐┬ ┬
├─┘│ │├┬┘ │ └─┐││││ │ ├─┤
┴  └─┘┴└─ ┴ └─┘┴ ┴┴ ┴ ┴ ┴
```

# portsmith

**Take control of your ports.** A tiny, fast, cross-platform CLI to see what's using your ports, free them, and remember the ones your project needs.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-CE412B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platforms](https://img.shields.io/badge/platforms-macOS_·_Linux_·_Windows-4c1?logo=apple&logoColor=white)](#-cross-platform)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#-contributing)

</div>

---

## Why portsmith?

You know the dance. `EADDRINUSE: address already in use :::3000`. You reach for the muscle-memory incantation:

```bash
kill -9 $(lsof -ti:3000)
```

…which works on your Mac, fails on your teammate's Windows box, and tells you nothing about *what* was actually holding the port. **portsmith** replaces that with a single command that works everywhere:

```bash
portsmith kill 3000
```

No more googling `lsof` flags. No more "works on my machine." One binary, every OS.

## ✨ Features

- 🔎 **See everything** — list every listening port with its protocol, address, PID, and process name.
- 🎯 **Free a port instantly** — `portsmith kill 3000` finds and stops whatever's squatting on it.
- 💾 **Per-project profiles** — save the ports your project needs to a `.portsmith.json` file.
- 🚀 **One-command clean boot** — `portsmith start` frees every saved port so your dev servers come up clean.
- 🧠 **Node-aware** — auto-detects ports from `package.json` scripts and `.env` files. It even knows *not* to touch your database.
- 🌍 **Truly cross-platform** — native backends for macOS, Linux, and Windows. The same command in the same `package.json` works for your whole team.
- ⚡ **Tiny & fast** — a single self-contained binary written in Rust. No runtime, no dependencies to install.

## 📦 Installation

### With Cargo (Rust toolchain)

```bash
cargo install portsmith
```

### From source

```bash
git clone https://github.com/shubhang-d/portman.git
cd portman
cargo install --path .
```

This drops a `portsmith` binary into `~/.cargo/bin`, which is already on your `PATH`. Verify it:

```bash
portsmith --version
```

## 🚀 Quick start

```bash
# What's running on my machine?
portsmith list

# What's on port 3000 specifically?
portsmith list 3000

# Free it.
portsmith kill 3000

# Remember this project's ports (auto-detected from package.json + .env)
portsmith profile save

# Later: free all of them in one shot before you boot
portsmith start
```

## 📖 Commands

### `portsmith list [port]`

List all listening ports, or just the one you care about.

```console
$ portsmith list
PROTO    ADDRESS                  PID      PROCESS
--------------------------------------------------------------
TCP      [::]:3000                87460    node
TCP      127.0.0.1:5037           2920     adb
UDP      0.0.0.0:5353             84539    Google Chrome Helper
TCP      127.0.0.1:5554           2960     qemu-system-aarch64

$ portsmith list 3000
PROTO    ADDRESS                  PID      PROCESS
--------------------------------------------------------------
TCP      [::]:3000                87460    node
```

### `portsmith kill <port>`

Free a port by stopping whatever process is listening on it.

```console
$ portsmith kill 3000
Freed port 3000: killed node (PID 87460).

$ portsmith kill 9999
Nothing is listening on port 9999.
```

### `portsmith profile save [ports...]`

Record the ports your project uses into `.portsmith.json`. Pass them explicitly, or let portsmith detect them:

```console
$ portsmith profile save
Detected port 3000 from package.json
Detected port 4000 from package.json
Detected port 5173 from .env
Saved 3 port(s) to .portsmith.json: [3000, 4000, 5173]

$ portsmith profile save 3000 8080
Saved 2 port(s) to .portsmith.json: [3000, 8080]
```

### `portsmith start`

Load the saved profile and free any port that's currently taken, so your project can boot conflict-free.

```console
$ portsmith start
Freed port 3000: killed node (PID 87460).
Port 4000 is already free.
Freed port 5173: killed node (PID 91022).
```

## 🟢 Node.js integration

portsmith pairs perfectly with Node projects. Drop it into your `package.json` and never fight a port again:

```json
{
  "scripts": {
    "predev": "portsmith kill 3000",
    "dev": "next dev"
  }
}
```

Running `npm run dev` now clears port 3000 first — on **every** operating system, unlike `lsof`/`kill` one-liners that break on Windows.

For multi-service projects, save a profile once and let `start` do the work:

```json
{
  "scripts": {
    "predev": "portsmith start",
    "dev": "concurrently \"next dev\" \"node api.js\""
  }
}
```

### How port detection works

When you run `portsmith profile save` with no arguments, it scans:

| Source | Recognizes |
| --- | --- |
| `package.json` scripts | `next dev -p 3000`, `vite --port=5173`, `cross-env PORT=4000 node server.js` |
| `.env`, `.env.local`, `.env.development`, `.env.development.local` | `PORT=3000`, `VITE_PORT="5173"`, `export API_PORT=8080` |

> **Your database is safe.** Detection deliberately skips datastore variables like `DATABASE_PORT`, `REDIS_PORT`, `MYSQL_*`, and `MONGO_*` — those are services you connect *to*, not dev servers that conflict, so portsmith will never kill them.

## 🗂️ The profile file

Profiles are plain, reviewable JSON written to `.portsmith.json` in your project root:

```json
{
  "ports": [3000, 4000, 5173]
}
```

Commit it to share a project's ports with your team, or edit it by hand to prune anything you don't want. Add it to `.gitignore` if you'd rather keep it local.

## 🌍 Cross-platform

Everything works identically across operating systems — no platform-specific code, no shell tricks.

| Capability | macOS | Linux | Windows |
| --- | :---: | :---: | :---: |
| List ports → process | ✅ | ✅ | ✅ |
| Kill process on a port | ✅ | ✅ | ✅ |
| Save / load profiles | ✅ | ✅ | ✅ |
| Node port auto-detection | ✅ | ✅ | ✅ |

> **Note:** stopping a process owned by another user or the system requires elevated privileges — use `sudo` on macOS/Linux or an Administrator terminal on Windows. portsmith fails gracefully with a clear message rather than crashing.

## 🛣️ Roadmap

- [ ] `portsmith watch` — live-updating view of port activity
- [ ] Parse `next.config.js` / `vite.config.ts` for ports
- [ ] `--json` output for scripting
- [ ] Prebuilt binaries & one-line installer for every platform
- [ ] Homebrew tap and `npm` wrapper

## 🤝 Contributing

Contributions are welcome! Open an issue to discuss an idea, or send a PR.

```bash
git clone https://github.com/shubhang-d/portman.git
cd portman
cargo build
cargo run -- list
```

## 📄 License

Released under the [MIT License](LICENSE).

---

<div align="center">
<sub>Built with 🦀 Rust — because your ports deserve a smith, not a séance.</sub>
</div>
