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
        // Something is very wrong if we don't have a TERM env var
        // or if it's not valid unicode.
        Err(env::VarError::NotPresent | env::VarError::NotUnicode(_)) => Unsupported,
        // `TERM=dumb` indicates that the terminal supports very little features.
        // We don't want to send any escape sequences to those terminals.
        Ok(term) if term == "dumb" => Unsupported,
        // Why is GNU Screen unsupported?
        //
        // Note: The following only applies if screen was compiled with `--enable-rxvt_osc`.
        //       Homebrew is a notable packager who doesn't enable this feature.
        //
        // 1. Screen only supports `OSC 11` (background) and not `OSC 10` (foreground)
        //
        // 2. Screen replies to queries in the incorrect order.
        //    We send  `OSC 11` + `DA1` and expect the answers to also be in that order.
        //    However, as far as I can tell, Screen relays the `OSC 11` query to the underlying terminal,
        //    and so we get the `DA1` response back *first*. This is usually an indicator that
        //    the terminal doesn't support the `OSC` query.
        //
        //    There are two both equally broken workarounds:
        //
        //    * Don't send `DA1`, just `OSC 11`. \
        //      Since Screen forwards the query to the underlying terminal, we won't get an answer
        //      if the underlying terminal doesn't support it. And we don't have a way to detect that
        //      => we hit the 1s timeout :/
        //
        //    * Send the query (`OSC 11` + `DA1`) to the underlying terminal by wrapping it between `CSI P` and `ST`.
        //      (There's a reverted commit that does exactly this: f06206b53d2499e95627ef29e5e35278209725db)
        //      * If there's exactly one attached display (underlying terminal)
        //        => everything works as usual.
        //      * If there's no attached display we don't get an answer to DA1
        //        => we hit the 1s timeout :/
        //      * If there are multiple displays attached (yes this is supported and quite fun to try) we get back multiple responses
        //        => since there's no way to know that we need to expect multiple responses
        //           some of them are not consumed by us and end up on the user's screen :/
        Ok(term) if term == "screen" || term.starts_with("screen.") => Unsupported,
        Ok(term) if term == "rxvt-unicode" || term.starts_with("rxvt-unicode-") => Urxvt,
        Ok(_) => None,
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum TerminalQuirks {
    None,
    Unsupported,
    Urxvt,
}

impl TerminalQuirks {
    pub(super) fn is_known_unsupported(self) -> bool {
        matches!(self, TerminalQuirks::Unsupported)
    }

    pub(super) fn string_terminator(self) -> &'static [u8] {
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

    pub(super) fn write_all(self, w: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
        w.write_all(bytes)
    }

    pub(super) fn write_string_terminator(self, writer: &mut dyn Write) -> io::Result<()> {
        self.write_all(writer, self.string_terminator())
    }
}
