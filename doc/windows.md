# Windows
## Windows Terminal
The Win32 API provides access to the console foreground and background colors.
However, this currently does not work in Windows Terminal. [Incorrect colors are reported] as a result.

Hopefully Windows Terminal [will support] querying for the colors using the OSC sequences.

## Other Terminals
Terminals relying on [conpty] that support OSC sequences on other platforms
do not support them of Windows because [conhost intercepts these OSC sequences].

[Incorrect colors are reported]: https://github.com/microsoft/terminal/issues/10639
[will support]: https://github.com/microsoft/terminal/issues/3718
[conpty]: https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session
[conhost intercepts these OSC sequences]: https://github.com/microsoft/terminal/issues/1173