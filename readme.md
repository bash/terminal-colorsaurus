# terminal-colorsaurus 🦕

[![Docs](https://img.shields.io/docsrs/terminal-colorsaurus/latest)](https://docs.rs/terminal-colorsaurus)
[![Crate Version](https://img.shields.io/crates/v/terminal-colorsaurus)](https://crates.io/crates/terminal-colorsaurus)


Determines the background and foreground color of the terminal
using the `OSC 10` and `OSC 11` terminal sequence.
On Windows, the colors are queried using the Win32 Console API.

This is useful for answering the question *"Is this terminal dark or light?"*.

## Example
```rust,no_run
use terminal_colorsaurus::{color_scheme, QueryOptions};

let colors = color_scheme(QueryOptions::default()).unwrap();
dbg!(colors.is_dark_on_light());
```

## [Docs](https://docs.rs/terminal-colorsaurus)

## Wishlist
These are some features that I would like to include in this crate,
but have not yet had the time to implement. PRs are welcome :)

* [ ] A CLI tool version of this library.
* [ ] Improve dynamic latency strategy (iTerm is a bit of a headache here as it has quite a big range when it comes to latency).
* [ ] Add support for `$COLORFGBG`
     (I don't  know if this is a good idea actually, the [readme](https://github.com/Canop/terminal-light#colorfgbg-strategy) of `terminal-light` lists some downsides.)

## Inspiration
This crate borrows ideas from many other projects. This list is by no means exhaustive.

* [xterm-query]: Use `mio` to wait for the terminal's response with a timeout.
* [termbg]: Lists a lot of terminals which served as a good starting point for me to test terminals as well.
* [macOS doesn't like polling /dev/tty][macos-dev-tty] by Nathan Craddock
* [This excellent answer on Stack Overflow][perceived-lightness] for determining the perceived lightness of a color. 

## License
Licensed under either of

* Apache License, Version 2.0
  ([license-apache.txt](license-apache.txt) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([license-mit.txt](license-mit.txt) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[xterm-query]: https://github.com/Canop/xterm-query
[termbg]: https://github.com/dalance/termbg
[macos-dev-tty]: https://nathancraddock.com/blog/macos-dev-tty-polling/
[perceived-lightness]: https://stackoverflow.com/a/56678483
