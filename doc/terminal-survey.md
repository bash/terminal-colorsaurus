# Terminal Survey

| Terminal              | `CSI c` | fg                   | bg                   | mean latency | `TERM`           | `TERM_PROGRAM`    | `TERM_PROGRAM_VERSION` | Version Tested             |
|-----------------------|---------|----------------------|----------------------|--------------|------------------|-------------------|------------------------|----------------------------|
| Jetbrains Fleet       | yes     | no                   | no                   | 82.924µs     | `xterm-256color` | `Jetbrains.Fleet` | yes                    | build 1.29.213 (macOS)     |
| macOS Terminal        | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 67.267µs     | `xterm-256color` | `Apple_Terminal`  | yes                    | Version 2.13 (447)         |
| iTerm2                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 39.944317ms  | `xterm-256color` | `iTerm.app`       | yes                    | Build 3.5.0beta18          |
| Alacritty             | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 121.323µs    | `xterm-256color` | no                | no                     | Version 0.13.1 (1) (macOS) |
| VSCode (xterm.js)     | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 7.96848ms    | `xterm-256color` | `vscode`          | yes                    | 1.85.1 (macOS)             |
| iSH (hterm)           | yes     | no                   | no                   | -            | `xterm-256color` | no                | no                     | 1.3.2 (Build 494) (iOS)    |
| IntelliJ IDEA         | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 53.284µs     | `xterm-256color` | no [^1]           | no                     | PyCharm 2023.3.2 (macOS)   |
| [Contour]             | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 25.833µs     | `contour` [^2]   | no [^3]           | no [^4]                | 0.4.1.6292 (macOS)         |
| GNOME Terminal (vte)  | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 10.539126ms  | `xterm-256color` | no                | no                     | 3.50.1                     |
| (GNOME) Console (vte) | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | -            | `xterm-256color` | `kgx`             | yes                    | 45.0                       |
| Konsole               | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 26.593µs     | `xterm-256color` | no                | no                     | 23.08.4                    |
| [QTerminal]           | yes     | no                   | no                   | 27.85µs      | `xterm-256color` | no                | no                     | 1.3.0                      |
| [foot]                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 15.025µs     | `foot`           | no                | no                     | 1.16.1                     |
| xterm                 | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 18.9µs       | `xterm-256color` | no                | no                     | 385                        |
| Linux console         | yes     | no                   | no                   | 4.073µs      | `linux`          | no                | no                     | -                          |
| Windows Terminal      | yes     | no                   | no                   | -            | no [^5]          | no                | no                     | 1.18.3181.0                |
| Windows Console Host  | yes     | no                   | no                   | -            | no               | no                | no                     | Windows 10.0.22631.2428    |
| PuTTY                 | yes     | no                   | no                   | -            | -                | -                 | -                      | 0.80                       |
| Hyper                 | yes     | yes                  | yes                  | 18.883401ms  | `xterm-256color` | `Hyper`           | yes                    | 3.4.1 (macOS)              |
| ConEmu / Cmder        | yes     | no                   | no                   | -            | -                | -                 | -                      | 230724 stable              |
| Mintty                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | -            | `xterm`          | `mintty`          | yes                    | 3.6.1                      |

> [!note]
> Some Linux terminals are omitted since they all use the `vte` library behind the scenes. \
> Here's a non-exhaustive list: GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (GNOME) Builder, (elementary) Terminal, LXTerminal.

[^1]: But it sets `TERMINAL_EMULATOR=JetBrains-JediTerm` instead.
[^2]: But it provides a terminfo entry by adding `TERMINFO_DIRS`.
[^3]: But it sets `TERMINAL_NAME=contour` instead.
[^4]: But it sets `TERMINAL_VERSION_STRING` and `TERMINAL_VERSION_TRIPLE` instead.
[^5]: But it can be recognized by `WT_SESSION` instead.


[Contour]: https://contour-terminal.org/
[QTerminal]: https://github.com/lxqt/qterminal
[foot]: https://codeberg.org/dnkl/foot
