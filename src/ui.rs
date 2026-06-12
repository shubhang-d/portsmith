use std::io::IsTerminal;

/// Whether to emit ANSI styling to stderr. Respects `NO_COLOR`, honors
/// `CLICOLOR_FORCE`, and otherwise only colors when stderr is a real terminal.
fn color_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }
    std::io::stderr().is_terminal()
}

/// Prints a boxed warning to stderr. The first entry is the bold heading; the
/// rest are body lines (use `""` for a blank spacer). The box is drawn in yellow
/// when color is enabled, and degrades to a clean ASCII-aligned box otherwise.
pub fn warn_box(lines: &[&str]) {
    let color = color_enabled();
    let yellow = if color { "\x1b[33m" } else { "" };
    let heading = if color { "\x1b[1;33m" } else { "" };
    let reset = if color { "\x1b[0m" } else { "" };

    const PAD: usize = 2;
    let widest = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let inner = widest + PAD * 2;
    let rule = "─".repeat(inner);

    eprintln!();
    eprintln!("  {yellow}╭{rule}╮{reset}");
    for (i, line) in lines.iter().enumerate() {
        let trailing = inner - PAD - line.chars().count();
        let text_style = if i == 0 { heading } else { yellow };
        eprintln!(
            "  {yellow}│{reset}{text_style}{lpad}{line}{rpad}{reset}{yellow}│{reset}",
            lpad = " ".repeat(PAD),
            rpad = " ".repeat(trailing),
        );
    }
    eprintln!("  {yellow}╰{rule}╯{reset}");
    eprintln!();
}
