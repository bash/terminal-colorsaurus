A list of terminals that were tested for support of DA1 (`CSI c`) and `OSC 10` / `OSC 11`.

| Terminal              | DA1 | Foreground | Background | Version Tested             |
|-----------------------|-----|------------|------------|----------------------------|
| Jetbrains Fleet       | yes | no         | no         | build 1.29.213 (macOS)     |
| macOS Terminal        | yes | yes        | yes        | Version 2.13 (447)         |
| iTerm2                | yes | yes        | yes        | Build 3.5.0beta18          |
| Alacritty             | yes | yes        | yes        | Version 0.13.1 (1) (macOS) |
| VSCode (xterm.js)     | yes | yes        | yes        | 1.85.1 (macOS)             |
| iSH (hterm)           | yes | no         | no         | 1.3.2 (Build 494) (iOS)    |
| IntelliJ IDEA         | yes | yes        | yes        | PyCharm 2023.3.2 (macOS)   |
| [Contour]             | yes | yes        | yes        | 0.4.1.6292 (macOS)         |
| GNOME Terminal (vte)  | yes | yes        | yes        | 3.50.1                     |
| (GNOME) Console (vte) | yes | yes        | yes        | 45.0                       |
| Konsole               | yes | yes        | yes        | 23.08.4                    |
| [QTerminal]           | yes | no         | no         | 1.3.0                      |
| [foot]                | yes | yes        | yes        | 1.16.1                     |
| xterm                 | yes | yes        | yes        | 385                        |
| Linux console         | yes | no         | no         | -                          |
| Windows Terminal      | yes | no         | no         | 1.18.3181.0                |
| Windows Console Host  | yes | no         | no         | Windows 10.0.22631.2428    |
| PuTTY                 | yes | no         | no         | 0.80                       |
| Hyper                 | yes | yes        | yes        | 3.4.1 (macOS)              |
| ConEmu / Cmder        | yes | no         | no         | 230724 stable              |
| Mintty                | yes | yes        | yes        | 3.6.1                      |

<br>

**ℹ️ Note:**
Some Linux terminals are omitted since they all use the `vte` library behind the scenes. \
Here's a non-exhaustive list: GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (GNOME) Builder, (elementary) Terminal, LXTerminal.


[Contour]: https://contour-terminal.org/
[QTerminal]: https://github.com/lxqt/qterminal
[foot]: https://codeberg.org/dnkl/foot
