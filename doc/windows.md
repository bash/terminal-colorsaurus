# Why is Windows not supported?
Terminals on Windows rely on a component called [Console Host], also referred to as `conhost`.

This is done in one of two ways:
1. By using the [Pseudoconsole] API. This is what newer terminals such as **Windows Terminal** or **Alacritty** do.
2. By [launching a hidden console window][RealConsole]. This is what third party terminals that have been around since before the introduction of the [Pseudoconsole] API do. One example is **ConEmu**.

To preserve backwards compatibility with programs that use (the now mostly obsolete) [Console API],
`conhost` intercepts some escape sequences such as `OSC 10` and `OSC 11`. However, `conhost` only supports setting colors using these two sequences, [but not querying][conhost/osc].

An alternative approach would be to retrieve the foreground and background color by using the [Console API]. \
However, for most Terminals—this includes **Windows Terminal**—[incorrect colors are reported](conhost/palette). \
Using the [Console API] to retrieve colors only produces correct results in `conhost`'s own terminal window.


[Console Host]: https://learn.microsoft.com/en-us/windows/console/definitions#console-host
[RealConsole]: https://conemu.github.io/en/RealConsole.html
[Pseudoconsole]: https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session
[Console API]: https://learn.microsoft.com/en-us/windows/console/console-functions
[conhost/osc]: https://github.com/microsoft/terminal/issues/3718
[conhost/palette]: https://github.com/microsoft/terminal/issues/10639
