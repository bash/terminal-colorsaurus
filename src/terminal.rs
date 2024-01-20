use std::env;

#[derive(Debug)]
pub(crate) enum TerminalKind {
    Supported,
    Unsupported,
    Unknown,
}

impl TerminalKind {
    pub(crate) fn from_env() -> Self {
        if let Ok(term) = env::var("TERM") {
            // tmux next-3.4 (which for some reason ships on Fedora supports querying for the colors)
            if term == "contour" || term == "foot" || term == "tmux" || term.starts_with("tmux-") {
                return TerminalKind::Supported;
            } else if term == "linux" {
                return TerminalKind::Unsupported;
            } else if term == "xterm" || term.starts_with("xterm-") {
                // A lot of terminals claim that they're xterm-some of which
                // do not support querying for colors. Let's do some investigating:
                return TerminalKind::from_env_for_xterm();
            }
        }

        TerminalKind::Unknown
    }

    fn from_env_for_xterm() -> Self {
        if let Ok(term_program) = env::var("TERM_PROGRAM") {
            const SUPPORTED: &[&str] = &[
                "Apple_Terminal",
                "iTerm.app",
                "vscode",
                "kgx", // (GNOME) Console
                "Hyper",
            ];
            const UNSUPPORTED: &[&str] = &[
                // While mintty supports the OSC 10 and 11 escape sequences, terminal-trx does not support enabling raw mode on it.
                "mintty",
                "Jetbrains.Fleet",
            ];

            if SUPPORTED.contains(&term_program.as_str()) {
                return TerminalKind::Supported;
            } else if UNSUPPORTED.contains(&term_program.as_str()) {
                return TerminalKind::Unsupported;
            }
        }

        if let Ok(term_emulator) = env::var("TERMINAL_EMULATOR") {
            if term_emulator == "JetBrains-JediTerm" {
                return TerminalKind::Supported;
            }
        }

        TerminalKind::Unknown
    }
}
