mod commands;
mod ui;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

use crate::commands::{check, kill, list, profile, start};

/// Color scheme for the help output.
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default())
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default().effects(Effects::BOLD));

const BANNER: &str = "\
\x1b[1;36m
  ┌─┐┌─┐┬─┐┌┬┐┌─┐┌┬┐┬┌┬┐┬ ┬
  ├─┘│ │├┬┘ │ └─┐││││ │ ├─┤
  ┴  └─┘┴└─ ┴ └─┘┴ ┴┴ ┴ ┴ ┴\x1b[0m
  \x1b[2mtake control of your ports\x1b[0m";

const AFTER_HELP: &str = "\
\x1b[1;32mThe one to remember:\x1b[0m
  \x1b[36mportsmith start\x1b[0m             Free your project's ports & check services

\x1b[1;32mWhen you need finer control:\x1b[0m
  \x1b[36mportsmith list\x1b[0m              See every listening port
  \x1b[36mportsmith kill 3000\x1b[0m         Free a specific port
  \x1b[36mportsmith save 8080\x1b[0m         Pin a port when auto-detect can't find it
  \x1b[36mportsmith check\x1b[0m             Verify your database & cache are running
";

/// portsmith — a tiny cross-platform port manager.
#[derive(Parser, Debug)]
#[command(
    name = "portsmith",
    author,
    version,
    about = BANNER,
    long_about = None,
    styles = STYLES,
    after_help = AFTER_HELP,
    arg_required_else_help = true,
)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Detect the project's ports, free any conflicts, and check services.
    Start,
    /// List all listening ports and their processes.
    #[command(visible_alias = "ls")]
    List {
        /// Only show the process listening on this exact port.
        port: Option<u16>,
    },
    /// Free a port by killing the process listening on it.
    #[command(visible_alias = "free")]
    Kill {
        /// The port to free, e.g. 3000.
        port: u16,
    },
    /// Pin the project's ports to .portsmith.json (when auto-detect isn't enough).
    Save {
        /// Ports to save. If omitted, auto-detects from .env and package.json.
        ports: Vec<u16>,
    },
    /// Verify the project's backing services (database, cache) are running.
    Check,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Start => start::run(),
        Commands::List { port } => list::run(port),
        Commands::Kill { port } => kill::run(port),
        Commands::Save { ports } => profile::save(ports),
        Commands::Check => check::run(),
    }
}
