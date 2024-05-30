# Changelog
## 0.4.2
* Add `keywords` to package metadata.

## 0.4.1
* 🐛 Fixed `OSC 11` response being visible to users of GNU Screen
     by detecting Screen and erroring before sending any control sequences (bash/terminal-colorsaurus#16).

## 0.4.0
* ⚡ Renamed «color scheme» to «color palette».
* ⚡ Removed `is_dark_on_light` and `is_light_on_dark` functions. Use `color_scheme` instead.
* Add new convenience function `color_scheme` which returns a nice `Dark / Light` enum.
* Add support for urxvt's `rgba:` color format.
* Further refined the documentation (more organized terminal list, new terminals tested).
* Improved handling of ambigous color palettes (e.g. when background color is the same as foreground).
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
