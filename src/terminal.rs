use std::env;

pub(crate) enum TerminalKind {
    Supported,
    Unsupported,
    Passthrough(Passthrough),
    Unknown,
}

pub(crate) enum Passthrough {
    Tmux,
    Screen,
}

impl TerminalKind {
    pub(crate) fn from_env() -> Self {
        if let Some(term) = env::var_os("TERM") {
            if term == "contour" || term == "foot" {
                return TerminalKind::Supported;
            } else if term == "linux" {
                return TerminalKind::Unsupported;
            } else if term == "tmux" {
                return TerminalKind::Passthrough(Passthrough::Tmux);
            } else if term == "screen" {
                return TerminalKind::Passthrough(Passthrough::Screen);
            }
        }

        if let Some(term_program) = env::var_os("TERM_PROGRAM") {
            const SUPPORTED: &[&str] = &[
                "Apple_Terminal",
                "iTerm.app",
                "vscode",
                "kgx", // (GNOME) Console
                "Hyper",
                "mintty",
            ];

            if SUPPORTED.iter().any(|x| x == &term_program) {
                return TerminalKind::Supported;
            } else if term_program == "Jetbrains.Fleet" {
                return TerminalKind::Unsupported;
            }
        }

        if let Some(term_emulator) = env::var_os("TERMINAL_EMULATOR") {
            if term_emulator == "JetBrains-JediTerm" {
                return TerminalKind::Supported;
            }
        }

        TerminalKind::Unknown
    }
}
