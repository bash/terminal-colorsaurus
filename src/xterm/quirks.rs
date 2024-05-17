use super::{BEL, ST};
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
        Ok(term) if term == "screen" || term.starts_with("screen.") => Screen,
        Ok(_) | Err(_) => None,
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum TerminalQuirks {
    None,
    Barebones,
    Urxvt,
    Screen,
}

impl TerminalQuirks {
    pub(super) fn is_known_unsupported(&self) -> bool {
        // `TERM=dumb` indicates that the terminal supports very little features.
        // We don't want to send any escape sequences to those terminals.
        matches!(self, TerminalQuirks::Barebones)
    }

    pub(super) fn string_terminator(&self) -> &[u8] {
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
        if let TerminalQuirks::Screen = self {
            screen::write_to_host_terminal(w, bytes)
        } else {
            w.write_all(bytes)
        }
    }

    pub(super) fn write_string_terminator(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.write_all(writer, self.string_terminator())
    }
}

// Screen breaks one of our fundamental assumptions:
// It responds to `DA1` *before* responding to `OSC 10`.
// To work around this we wrap our query in a `DCS` / `ST` pair.
//
// This directs screen to send our query to the underlying terminal instead of
// interpreting our query itself. Hopefully the underlying terminal is more
// *sensible* about order...
mod screen {
    use super::*;
    use crate::xterm::{ESC, ST};
    use memchr::memchr_iter;

    /// From the [manual](https://www.gnu.org/software/screen/manual/html_node/Control-Sequences.html):
    /// > Device Control String \
    /// > Outputs a string directly to the host
    /// > terminal without interpretation.
    const DCS: &[u8] = b"\x1bP";

    pub(super) fn write_to_host_terminal(w: &mut dyn Write, mut bytes: &[u8]) -> io::Result<()> {
        loop {
            // If our query contains `ST` we need to split it across multiple
            // `DCS` / `ST` pairs to avoid screen from interpreting our `ST` as
            // the terminator for the `DCS` sequence.
            if let Some(index) = find_st(bytes) {
                write_to_host_terminal_unchecked(w, &bytes[..index])?;
                write_to_host_terminal_unchecked(w, &[ESC])?;
                write_to_host_terminal_unchecked(w, &[b'\\'])?;
                bytes = &bytes[(index + ST.len())..];
            } else {
                write_to_host_terminal_unchecked(w, bytes)?;
                break;
            }
        }

        Ok(())
    }

    fn write_to_host_terminal_unchecked(w: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
        if !bytes.is_empty() {
            w.write_all(DCS)?;
            w.write_all(bytes)?;
            w.write_all(ST)?;
        }
        Ok(())
    }

    fn find_st(haystack: &[u8]) -> Option<usize> {
        memchr_iter(ESC, haystack)
            .filter_map(|index| {
                let next_byte = *haystack.get(index + 1)?;
                (next_byte == b'\\').then_some(index)
            })
            .next()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::fmt::CaretNotation;

        #[test]
        fn wraps_query_between_dcs_and_st() {
            let expected = b"\x1bP\x1b[c\x1b\\";
            let mut actual = Vec::new();
            write_to_host_terminal(&mut actual, b"\x1b[c").unwrap_or_else(|_| unreachable!());
            assert_eq!(to_string(expected.as_slice()), to_string(&actual));
        }

        #[test]
        fn splits_st_among_multiple_dcs_and_st_pairs() {
            let expected = b"\x1bP\x1b]11;?\x1b\\\x1bP\x1b\x1b\\\x1bP\\\x1b\\";
            let mut actual = Vec::new();
            write_to_host_terminal(&mut actual, b"\x1b]11;?\x1b\\")
                .unwrap_or_else(|_| unreachable!());
            assert_eq!(to_string(expected.as_slice()), to_string(&actual));
        }

        #[test]
        fn finds_st_at_start() {
            assert_eq!(Some(0), find_st(ST));
        }

        #[test]
        fn finds_st_after_esc() {
            assert_eq!(Some(1), find_st(&[ESC, ESC, b'\\']))
        }

        #[test]
        fn finds_first_esc() {
            assert_eq!(
                Some(3),
                find_st(&[b'f', b'o', b'o', ESC, b'\\', ESC, b'\\'])
            )
        }

        fn to_string(input: &[u8]) -> String {
            use std::str::from_utf8;
            format!(
                "{}",
                CaretNotation(from_utf8(input).expect("valid utf-8 data"))
            )
        }
    }
}
