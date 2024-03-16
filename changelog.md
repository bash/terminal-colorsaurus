# Changelog
## 0.3.3
* Feature: Add new `Color::to_rgb8` convenience function that converts channels to 8 bit.
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
