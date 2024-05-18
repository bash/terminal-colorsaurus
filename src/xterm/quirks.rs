use std::env;
use std::io::{self, Write};
use std::sync::OnceLock;

pub(super) fn terminal_quirks_from_env() -> TerminalQuirks {
    // This OnceLock is not here for efficiency, it's here so that
    // we have consistent results in case a consumer uses `set_var`.
    static TERMINAL_QUIRK: OnceLock<TerminalQuirks> = OnceLock::new();
    *TERMINAL_QUIRK.get_or_init(terminal_quirk_from_env_eager)
}

fn terminal_quirk_from_env_eager() -> TerminalQuirks {
    use TerminalQuirks::*;
    match env::var("TERM") {
        Ok(term) if term == "dumb" => Barebones,
        Ok(term) if term == "rxvt-unicode" || term.starts_with("rxvt-unicode-") => Urxvt,
        Ok(_) | Err(_) => None,
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum TerminalQuirks {
    None,
    Barebones,
    Urxvt,
}

impl TerminalQuirks {
    pub(super) fn is_known_unsupported(&self) -> bool {
        // `TERM=dumb` indicates that the terminal supports very little features.
        // We don't want to send any escape sequences to those terminals.
        matches!(self, TerminalQuirks::Barebones)
    }

    pub(super) fn string_terminator(&self) -> &[u8] {
        const ST: &[u8] = b"\x1b\\";
        const BEL: u8 = 0x07;

        if let TerminalQuirks::Urxvt = self {
            // The currently released version has a bug where it terminates the response with `ESC` instead of `ST`.
            // Fixed by revision [1.600](http://cvs.schmorp.de/rxvt-unicode/src/command.C?revision=1.600&view=markup).
            // The bug can be worked around by sending a query with `BEL` which will result in a `BEL`-terminated response.
            &[BEL]
        } else {
            ST
        }
    }

    pub(super) fn write_all(&self, w: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
        w.write_all(bytes)
    }

    pub(super) fn write_string_terminator(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.write_all(writer, self.string_terminator())
    }
}
