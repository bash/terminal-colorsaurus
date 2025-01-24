# Changelog
## 0.4.8
* 🐛 Fixed an error on windows where the query would not
  succeed when the standard input was redirected.
* ⚡ The color parsing code has been extracted into its own crate
  that terminal-colorsaurus now depends on: [`xterm-color](https://crates.io/crates/xterm-color).
* 📝 The terminal survey has been extended and updated.

## 0.4.7
* 🐛 Re-add missing license texts to the published crate
     (this was a regression introduced in `0.4.5`).
* ✨ Recognize `Eterm` as unsupported.

## 0.4.6
* 🐛 Switch the string terminator back to `BEL` to work around
     and issue in urxvt. Previously this was done only when urxvt
     was detected. Unfortunately this detection was not reliable.

## 0.4.5
* ✨ Added support for Windows (starting with Windows Terminal v1.22, in preview at the time of writing).
### Docs
* Included more terminals in terminal survey.
* The top level crate docs have been reduced to improve readability.

## 0.4.4
* Bump `mio` dependency to 1.0.
* ✨ Add helpful aliases to the docs.

## 0.4.3
* Remove private `docs` crate feature.
* 🐛 Fix broken link in docs.

## 0.4.2
* ✨ Add optional dependency on `anstyle` to enable conversions from `Color` to `anstyle::RgbColor`.
* ✨ Add conversion from `Color` to `rgb::RGB8`.
* ✨ Treat environments with no `TERM` env var as unsupported.
* Add `keywords` to package metadata.
* Remove dependency on `thiserror`.

## 0.4.1
* 🐛 Fixed `OSC 11` response being visible to users of GNU Screen
     by detecting Screen and erroring before sending any control sequences (bash/terminal-colorsaurus#16).

## 0.4.0
* ⚡ Renamed «color scheme» to «color palette».
* ⚡ Removed `is_dark_on_light` and `is_light_on_dark` functions. Use `color_scheme` instead.
* Add new convenience function `color_scheme` which returns a nice `Dark / Light` enum.
* Add support for urxvt's `rgba:` color format.
* Further refined the documentation (more organized terminal list, new terminals tested).
* Improved handling of ambiguous color palettes (e.g. when background color is the same as foreground).
* Queries are now terminated with `ST` (the standard string terminator) instead of `BEL` (which is an xterm extension).

## 0.3.3
* Feature: Add new `Color::scale_to_8bit` function.
* Fix: Correctly scale colors up to 16 bits per channel.
* Fix: Support full range of `#r(rrr)g(ggg)b(bbb)` color syntax.
### Docs
* Update terminal survey docs.
* Replace table with pretty graphs for latency docs ✨.

## 0.3.2
* Add support for Terminology's color format.
* Bump `mio` dependency.

### Docs
* Include benchmark results in rustdocs.
* Extend terminal survey to more terminals.

## 0.3.1
* Remove support for Windows. [Why?](./doc/windows.md)
* Remove preconditions from public API.

## 0.2.3
* Updated to latest version of `terminal-trx`.
* Improved docs: Terminal Survey has been simplified.

## 0.2.2
* Added missing docs and clarified them in some places.

## 0.2.1
* Exposed pager detection heuristic.

## 0.2.0
* Improved detection of terminals that support querying for colors.
* Renamed `QueryOptions.max_timeout` -> `QueryOptions.timeout`.

## 0.1.0
* Initial release
