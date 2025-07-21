# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `350.34 ns` (✅ **1.00x**) | `550.28 ns` (❌ *1.57x slower*)   | `2.15 us` (❌ *6.15x slower*)     | `562.89 ns` (❌ *1.61x slower*)   | `618.45 ns` (❌ *1.77x slower*)   | `2.29 us` (❌ *6.54x slower*)        |
| **`128`**  | `802.47 ns` (✅ **1.00x**) | `1.10 us` (❌ *1.37x slower*)     | `5.17 us` (❌ *6.44x slower*)     | `1.36 us` (❌ *1.69x slower*)     | `1.25 us` (❌ *1.56x slower*)     | `5.45 us` (❌ *6.80x slower*)        |
| **`256`**  | `2.34 us` (✅ **1.00x**)   | `2.24 us` (✅ **1.04x faster**)   | `13.09 us` (❌ *5.60x slower*)    | `3.71 us` (❌ *1.59x slower*)     | `2.60 us` (✅ **1.11x slower**)   | `14.33 us` (❌ *6.13x slower*)       |
| **`512`**  | `7.47 us` (✅ **1.00x**)   | `4.64 us` (✅ **1.61x faster**)   | `37.15 us` (❌ *4.98x slower*)    | `11.93 us` (❌ *1.60x slower*)    | `5.47 us` (✅ **1.36x faster**)   | `45.08 us` (❌ *6.04x slower*)       |
| **`768`**  | `15.86 us` (✅ **1.00x**)  | `7.21 us` (🚀 **2.20x faster**)   | `73.44 us` (❌ *4.63x slower*)    | `25.38 us` (❌ *1.60x slower*)    | `8.40 us` (🚀 **1.89x faster**)   | `82.26 us` (❌ *5.18x slower*)       |
| **`1024`** | `27.95 us` (✅ **1.00x**)  | `9.98 us` (🚀 **2.80x faster**)   | `120.95 us` (❌ *4.33x slower*)   | `44.53 us` (❌ *1.59x slower*)    | `11.43 us` (🚀 **2.45x faster**)  | `135.73 us` (❌ *4.86x slower*)      |
| **`1536`** | `63.70 us` (✅ **1.00x**)  | `15.85 us` (🚀 **4.02x faster**)  | `251.33 us` (❌ *3.95x slower*)   | `99.98 us` (❌ *1.57x slower*)    | `18.21 us` (🚀 **3.50x faster**)  | `282.87 us` (❌ *4.44x slower*)      |
| **`2048`** | `115.09 us` (✅ **1.00x**) | `22.20 us` (🚀 **5.19x faster**)  | `426.51 us` (❌ *3.71x slower*)   | `180.43 us` (❌ *1.57x slower*)   | `26.16 us` (🚀 **4.40x faster**)  | `480.00 us` (❌ *4.17x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

