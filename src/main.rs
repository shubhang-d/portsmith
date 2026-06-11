mod commands;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

use crate::commands::{kill, list, profile, start};

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
\x1b[1;32mExamples:\x1b[0m
  \x1b[36mportsmith list\x1b[0m              See every listening port
  \x1b[36mportsmith list 3000\x1b[0m         See what's running on port 3000
  \x1b[36mportsmith kill 3000\x1b[0m         Free port 3000
  \x1b[36mportsmith profile save\x1b[0m      Remember this project's ports
  \x1b[36mportsmith start\x1b[0m             Free the saved ports so your app can boot
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
    /// List all listening ports and their processes.
    List {
        /// Only show the process listening on this exact port.
        port: Option<u16>,
    },
    /// Free a port by killing the process listening on it.
    Kill {
        /// The port to free, e.g. 3000.
        port: u16,
    },
    /// Manage saved port profiles.
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Load the saved profile and free any conflicting ports.
    Start,
}

#[derive(Subcommand, Debug)]
enum ProfileAction {
    /// Save the project's ports to .portsmith.json.
    Save {
        /// Ports to save. If omitted, auto-detects from .env and package.json.
        ports: Vec<u16>,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::List { port } => list::run(port),
        Commands::Kill { port } => kill::run(port),
        Commands::Profile { action } => match action {
            ProfileAction::Save { ports } => profile::save(ports),
        },
        Commands::Start => start::run(),
    }
}
