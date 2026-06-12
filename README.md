<div align="center">

```
┌─┐┌─┐┬─┐┌┬┐┌─┐┌┬┐┬┌┬┐┬ ┬
├─┘│ │├┬┘ │ └─┐││││ │ ├─┤
┴  └─┘┴└─ ┴ └─┘┴ ┴┴ ┴ ┴ ┴
```

# portsmith

**Take control of your ports.** A tiny, cross-platform CLI that frees the ports your project needs — automatically.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-CE412B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platforms](https://img.shields.io/badge/platforms-macOS_·_Linux_·_Windows-4c1?logo=apple&logoColor=white)](#cross-platform)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## The problem

`EADDRINUSE: address already in use :::3000`. The usual fix —

```bash
kill -9 $(lsof -ti:3000)
```

— works on your Mac, breaks on your teammate's Windows, and tells you nothing about what was on the port. portsmith does it in one command, everywhere.

## Install

```bash
cargo install portsmith
```

Or from source: `git clone https://github.com/shubhang-d/portman && cd portman && cargo install --path .`

## The one command to know

```bash
portsmith start
```

Run it in your project. It figures out which ports your project uses, frees the busy ones, and checks your database — all at once. No setup.

```console
$ portsmith start
Detected port 3000 from Next.js default (package.json)
Freeing project ports...
  3000	freed (was node, PID 87460)
Services:
  ✅ postgres (5432) is running   [.env]
Ready.
```

## All commands

| Command | What it does |
| --- | --- |
| `portsmith start` | **The main one.** Detect your ports, free the busy ones, check services. |
| `portsmith list` &nbsp;_(ls)_ | Show everything that's listening. Add a port to filter: `list 3000`. |
| `portsmith kill <port>` &nbsp;_(free)_ | Free one specific port. |
| `portsmith save [ports]` | Pin ports manually, for when auto-detect can't find them. |
| `portsmith check` | Verify your database/cache are running (never kills anything). |

## Use it in package.json

Drop one line in and never fight a port again — on every OS:

```json
{
  "scripts": {
    "predev": "portsmith start",
    "dev": "next dev"
  }
}
```

## How it finds your ports

When portsmith needs your project's ports, it reads your `.env*` files and `package.json` scripts, and falls back to the framework's default port (so a plain `next dev` still means 3000). It works in monorepos, skips `node_modules`, and **never touches database ports** like `DATABASE_URL` or `REDIS_PORT` — those are services you connect to, not conflicts to clear.

<details>
<summary>Framework defaults it knows</summary>

| Framework | Port | Framework | Port |
| --- | --- | --- | --- |
| Next.js | 3000 | Astro | 4321 |
| Nuxt | 3000 | SvelteKit | 5173 |
| Create React App | 3000 | Vite | 5173 |
| Remix | 3000 | Angular | 4200 |
| Vue CLI | 8080 | Gatsby | 8000 |

</details>

## Cross-platform

Works identically on **macOS, Linux, and Windows** — one binary, no shell tricks. Stopping a process owned by another user needs elevated privileges (`sudo`, or an Administrator terminal on Windows); portsmith says so clearly instead of failing silently.

## License

[MIT](LICENSE)

---

<div align="center">
<sub>Built with 🦀 Rust — because your ports deserve a smith, not a séance.</sub>
</div>
