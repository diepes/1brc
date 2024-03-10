# 1 Billion row challenge - 2024

* [https://1brc.dev/]

## Code

* Cloned form [https://github.com/tumdum/1brc]
  * My rust solution got to 26s (10s just to read file), this one is much simpler and runs in 5seconds, amazing.
  * One trick is the unsafe memmap of the 1brc file into ram, makes the reading much faster.

* Added criterion benchmarking
  * $ cargo bench
  * View results under target/criterion/bench_1brc/report/index.html

## ToDo

* Test custom hash algorithm
