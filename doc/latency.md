# Latency
Measurements generated using [examples/benchmark](../examples/benchmark/src/main.rs):

| Terminal            | Iterations | min          | max           | mean         |
|---------------------|------------|--------------|---------------|--------------|
| foot                | 10000      | 26.130 µs    | 248.260 µs    | 31.825 µs    |
| XTerm               | 10000      | 33.550 µs    | 295.990 µs    | 39.926 µs    |
| Konsole             | 10000      | 34.110 µs    | 3.652145 ms   | 38.094 µs    |
| Alacritty           | 10000      | 40.340 µs    | 414.961 µs    | 57.569 µs    |
| IntelliJ IDEA       | 10000      | 71.267 µs    | 2.453094 ms   | 154.491 µs   |
| Terminal.app        | 10000      | 196.143 µs   | 25.064408 ms  | 241.916 µs   |
| Hyper               | 10000      | 16.287473 ms | 57.534790 ms  | 20.040066 ms |
| GNOME Console (vte) | 10000      | 8.157828 ms  | 56.823240 ms  | 20.656316 ms |
| VSCode              | 10000      | 24.164008 ms | 140.036258 ms | 26.061349 ms |
| iTerm2              | 10000      | 4.065856 ms  | 49.872777 ms  | 28.259948 ms |

**ℹ️ Note:**
The macOS terminals were not tested on the same machine as the Linux terminals.
