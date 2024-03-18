A list of terminals that were tested for support of DA1 (`CSI c`) and `OSC 10` / `OSC 11`.

| Terminal              | DA1  | Foreground | Background | Version Tested                     |
|-----------------------|------|------------|------------|------------------------------------|
| Jetbrains Fleet       | yes  | no         | no         | build 1.29.213 (macOS)             |
| macOS Terminal        | yes  | yes        | yes        | Version 2.13 (447)                 |
| iTerm2                | yes  | yes        | yes        | Build 3.5.0beta18                  |
| Alacritty             | yes  | yes        | yes        | Version 0.13.1 (1) (macOS)         |
| VSCode (xterm.js)     | yes  | yes        | yes        | 1.85.1 (macOS)                     |
| iSH (hterm)           | yes  | no         | no         | 1.3.2 (Build 494) (iOS)            |
| IntelliJ IDEA         | yes  | yes        | yes        | PyCharm 2023.3.2 (macOS)           |
| [Contour]             | yes  | yes        | yes        | 0.4.1.6292 (macOS)                 |
| GNOME Terminal (vte)  | yes  | yes        | yes        | 3.50.1                             |
| (GNOME) Console (vte) | yes  | yes        | yes        | 45.0                               |
| Konsole               | yes  | yes        | yes        | 23.08.4                            |
| [QTerminal]           | yes  | no         | no         | 1.3.0                              |
| [foot]                | yes  | yes        | yes        | 1.16.1                             |
| xterm                 | yes  | yes        | yes        | 385                                |
| Linux console         | yes  | no         | no         | -                                  |
| Windows Terminal      | yes  | no         | no         | 1.18.3181.0                        |
| Windows Console Host  | yes  | no         | no         | Windows 10.0.22631.2428            |
| PuTTY                 | yes  | no         | no         | 0.80                               |
| Hyper                 | yes  | yes        | yes        | 3.4.1 (macOS)                      |
| ConEmu / Cmder        | yes  | no         | no         | 230724 stable                      |
| Mintty                | yes  | yes        | yes        | 3.6.1                              |
| [WezTerm]             | yes  | yes        | yes        | 20240203-110809-5046fc22 (flatpak) |
| [kitty]               | yes  | yes        | yes        | 0.31.0                             |
| [Rio Terminal]        | yes  | yes        | yes        | 0.0.36 (wayland)                   |
| [rxvt-unicode]        | yes  | yes        | yes        | 9.31                               |
| QMLKonsole            | yes  | no         | no         | 23.08.5                            |
| mrxvt                 | yes  | no         | no         | 0.5.3                              |
| Eterm                 | no ⚠️ | no         | no         | 0.9.6                              |
| [cool-retro-term]     | yes  | no         | no         | 1.2.0                              |
| [anyterm]             | no ⚠️ | no         | no         | 1.2.3                              |
| [shellinabox]         | no ⚠️ | no         | no         | 2.20                               |
| [Terminology]         | yes  | yes [^1]   | yes        | 1.13.0                             |
| [Termux]              | yes  | yes        | yes        | 0.118.0                            |

<br>

**ℹ️ Note 1:**
Some Linux terminals are omitted since they all use the `vte` library behind the scenes. \
Here's a non-exhaustive list: GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (GNOME) Builder, (elementary) Terminal, LXTerminal, Guake.

**ℹ️ Note 2:**
If not otherwise noted, the terminals respond using the `rgb:r(rrr)/g(ggg)/b(bbbb)` format.
See [Color Strings](https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Color_Strings) for details on what is theoretically possible.

[^1]: Responds using the `#r(rrr)g(ggg)b(bbb)` format.

[Contour]: https://contour-terminal.org/
[QTerminal]: https://github.com/lxqt/qterminal
[foot]: https://codeberg.org/dnkl/foot
[WezTerm]: https://wezfurlong.org/wezterm/
[kitty]: https://sw.kovidgoyal.net/kitty/
[Rio Terminal]: https://raphamorim.io/rio/
[rxvt-unicode]: http://software.schmorp.de/pkg/rxvt-unicode.html
[cool-retro-term]: https://github.com/Swordfish90/cool-retro-term
[anyterm]: https://anyterm.org/
[shellinabox]: https://github.com/shellinabox/shellinabox
[Terminology]: http://www.enlightenment.org/
[Termux]: https://termux.dev/en/
