❯ cargo bench

```
   Compiling crc32_xxhash3_benchmark v0.1.0 (/Users/z/git/crc32_xxhash3_benchmark)
    Finished bench [optimized] target(s) in 2.70s
     Running benches/benchmark.rs (target/release/deps/benchmark-f952636e20aa0c00)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
crc32fast               time:   [27.164 ns 27.261 ns 27.366 ns]
                        change: [-6.3256% -3.7159% -1.8624%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

crc32                   time:   [138.03 ns 138.49 ns 138.98 ns]
                        change: [-3.5363% -2.5534% -1.5946%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

xxhash32                time:   [34.191 ns 34.286 ns 34.383 ns]
                        change: [-2.5523% -1.7833% -1.0344%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
```
