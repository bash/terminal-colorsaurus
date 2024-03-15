# Latency Measurements
Measurements generated using [examples/benchmark](../examples/benchmark/src/main.rs):
```shell
cargo run --release -p benchmark '<TERM_NAME>'
```

| Terminal            | Iterations | min          | max           | mean         | supported |
|---------------------|------------|--------------|---------------|--------------|-----------|
| foot                | 10000      | 26.130 µs    | 248.260 µs    | 31.825 µs    | yes       |
| XTerm               | 10000      | 33.550 µs    | 295.990 µs    | 39.926 µs    | yes       |
| Konsole             | 10000      | 34.110 µs    | 3.652145 ms   | 38.094 µs    | yes       |
| Alacritty           | 10000      | 40.340 µs    | 414.961 µs    | 57.569 µs    | yes       |
| IntelliJ IDEA       | 10000      | 71.267 µs    | 2.453094 ms   | 154.491 µs   | yes       |
| Terminal.app        | 10000      | 196.143 µs   | 25.064408 ms  | 241.916 µs   | yes       |
| Hyper               | 10000      | 16.287473 ms | 57.534790 ms  | 20.040066 ms | yes       |
| GNOME Console (vte) | 10000      | 8.157828 ms  | 56.823240 ms  | 20.656316 ms | yes       |
| VSCode              | 10000      | 24.164008 ms | 140.036258 ms | 26.061349 ms | yes       |
| iTerm2              | 10000      | 4.065856 ms  | 49.872777 ms  | 28.259948 ms | yes       |
| QTerminal           | 10000      | 26.22 µs     | 4.127641 ms   | 37.446 µs    | no        |
| linux               | 10000      | 15.47 µs     | 75.19 µs      | 16.107 µs    | no        |
| WezTerm             | 10000      | 1.174129 ms  | 6.400318 ms   | 3.461548 ms  | yes       |
| kitty               | 10000      | 1.412243 ms  | 6.343324 ms   | 3.137705 ms  | yes       |
| Rio                 | 10000      | 36.94 µs     | 1.626094 ms   | 56.959 µs    | yes       |
| rxvt-unicode        | 10000      | 27.9 µs      | 11.97489 ms   | 37.092 µs    | yes       |
| QMLKonsole          | 10000      | 25.01 µs     | 0.969482 ms   | 27.317 µs    | no        |
| cool-retro-term     | 10000      | 28.07 µs     | 3.457008 ms   | 35.218 µs    | no        |
| Terminology         | 10000      | 30.57 µs     | 0.620351 ms   | 36.248 µs    | yes       |

**ℹ️ Note:**
The macOS terminals were not tested on the same machine as the Linux terminals.
