# Count Genome Bases

This library counts the letters ACGT as fast as possible.

On 72 logical cores we achieve a throughput of 80.19 GiB/s.

On a laptop with 8 logical cores we obtain 13.766 GiB/s.

## Usage

```rust
use genome_counter;
let results = genome_counter::count_opt(b"ACGT").unwrap();
assert_eq!(results.a, 1);
assert_eq!(results.c, 1);
assert_eq!(results.g, 1);
assert_eq!(results.t, 1);
```

## Benchmarking

We use criterion.rs for benchmarking which yields the following results on 100e6 bytes:

```
Benchmarking count_rand_100000000_opt: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 35.5s or reduce sample count to 20.
count_rand_100000000_opt                                                                             
                        time:   [6.6789 ms 6.7654 ms 6.8687 ms]
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe

Benchmarking count_rand_100000000_simple: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 614.4s or reduce sample count to 10.
count_rand_100000000_simple                                                                             
                        time:   [127.08 ms 133.08 ms 139.84 ms]
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) high mild
  13 (13.00%) high severe
```

