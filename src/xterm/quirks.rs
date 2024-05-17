use std::env;
use std::io::{self, Write};
use std::sync::{Arc, OnceLock};

pub(super) fn terminal_quirks_from_env() -> &'static dyn TerminalQuirks {
    static TERMINAL_QUIRK: OnceLock<Arc<dyn TerminalQuirks>> = OnceLock::new();
    TERMINAL_QUIRK
        .get_or_init(terminal_quirk_from_env_eager)
        .as_ref()
}

fn terminal_quirk_from_env_eager() -> Arc<dyn TerminalQuirks> {
    match env::var("TERM") {
        Ok(term) if term == "dumb" => Arc::new(Barebones),
        Ok(term) if term == "rxvt-unicode" || term.starts_with("rxvt-unicode-") => Arc::new(Urxvt),
        Ok(_) | Err(_) => Arc::new(Generic),
    }
}

pub(super) trait TerminalQuirks: Send + Sync {
    fn is_known_unsupported(&self) -> bool {
        false
    }

    fn string_terminator(&self) -> &[u8] {
        const ST: &[u8] = b"\x1b\\";
        ST
    }

    fn write_string_terminator(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.write_all(writer, self.string_terminator())
    }

    fn write_all(&self, writer: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
        writer.write_all(bytes)
    }
}

struct Generic;

impl TerminalQuirks for Generic {}

/// `TERM=dumb` indicates that the terminal supports very little features.
/// We don't want to send any escape sequences to those terminals.
struct Barebones;

impl TerminalQuirks for Barebones {
    fn is_known_unsupported(&self) -> bool {
        true
    }
}

struct Urxvt;

impl TerminalQuirks for Urxvt {
    // The currently released version has a bug where it terminates the response with `ESC` instead of `ST`.
    // Fixed by revision [1.600](http://cvs.schmorp.de/rxvt-unicode/src/command.C?revision=1.600&view=markup).
    // The bug can be worked around by sending a query with `BEL` which will result in a `BEL`-terminated response.
    fn string_terminator(&self) -> &[u8] {
        const BEL: u8 = 0x07;
        &[BEL]
    }
}
