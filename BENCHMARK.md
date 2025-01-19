# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`          | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------|:----------------------------------- |
| **`64`**   | `759.27 ns` (✅ **1.00x**) | `885.98 ns` (❌ *1.17x slower*)   | `5.37 us` (❌ *7.07x slower*)     | `1.03 us` (❌ *1.36x slower*)     | `1.28 us` (❌ *1.69x slower*)    | `6.42 us` (❌ *8.45x slower*)        |
| **`128`**  | `1.61 us` (✅ **1.00x**)   | `1.76 us` (✅ **1.09x slower**)   | `11.78 us` (❌ *7.30x slower*)    | `2.29 us` (❌ *1.42x slower*)     | `2.56 us` (❌ *1.59x slower*)    | `14.35 us` (❌ *8.90x slower*)       |
| **`256`**  | `3.84 us` (✅ **1.00x**)   | `3.66 us` (✅ **1.05x faster**)   | `27.97 us` (❌ *7.28x slower*)    | `5.72 us` (❌ *1.49x slower*)     | `5.75 us` (❌ *1.50x slower*)    | `32.83 us` (❌ *8.54x slower*)       |
| **`512`**  | `10.57 us` (✅ **1.00x**)  | `7.31 us` (✅ **1.45x faster**)   | `72.01 us` (❌ *6.81x slower*)    | `16.00 us` (❌ *1.51x slower*)    | `12.08 us` (❌ *1.14x slower*)   | `85.57 us` (❌ *8.10x slower*)       |
| **`768`**  | `20.59 us` (✅ **1.00x**)  | `11.05 us` (🚀 **1.86x faster**)  | `131.42 us` (❌ *6.38x slower*)   | `31.65 us` (❌ *1.54x slower*)    | `18.79 us` (✅ **1.10x faster**) | `159.45 us` (❌ *7.75x slower*)      |
| **`1024`** | `34.90 us` (✅ **1.00x**)  | `15.01 us` (🚀 **2.32x faster**)  | `203.78 us` (❌ *5.84x slower*)   | `53.56 us` (❌ *1.53x slower*)    | `25.94 us` (✅ **1.35x faster**) | `248.30 us` (❌ *7.11x slower*)      |
| **`1536`** | `79.23 us` (✅ **1.00x**)  | `23.63 us` (🚀 **3.35x faster**)  | `424.65 us` (❌ *5.36x slower*)   | `120.93 us` (❌ *1.53x slower*)   | `42.46 us` (🚀 **1.87x faster**) | `524.15 us` (❌ *6.62x slower*)      |
| **`2048`** | `140.18 us` (✅ **1.00x**) | `33.75 us` (🚀 **4.15x faster**)  | `755.57 us` (❌ *5.39x slower*)   | `211.62 us` (❌ *1.51x slower*)   | `59.84 us` (🚀 **2.34x faster**) | `876.67 us` (❌ *6.25x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

