# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `334.16 ns` (✅ **1.00x**) | `458.22 ns` (❌ *1.37x slower*)   | `2.06 us` (❌ *6.15x slower*)     | `507.00 ns` (❌ *1.52x slower*)   | `581.89 ns` (❌ *1.74x slower*)   | `2.21 us` (❌ *6.60x slower*)        |
| **`128`**  | `811.08 ns` (✅ **1.00x**) | `923.18 ns` (❌ *1.14x slower*)   | `4.74 us` (❌ *5.84x slower*)     | `1.25 us` (❌ *1.55x slower*)     | `1.17 us` (❌ *1.44x slower*)     | `5.29 us` (❌ *6.52x slower*)        |
| **`256`**  | `2.36 us` (✅ **1.00x**)   | `1.89 us` (✅ **1.25x faster**)   | `12.24 us` (❌ *5.17x slower*)    | `3.72 us` (❌ *1.57x slower*)     | `2.39 us` (✅ **1.01x slower**)   | `14.38 us` (❌ *6.08x slower*)       |
| **`512`**  | `7.42 us` (✅ **1.00x**)   | `3.96 us` (🚀 **1.87x faster**)   | `35.61 us` (❌ *4.80x slower*)    | `11.89 us` (❌ *1.60x slower*)    | `4.97 us` (✅ **1.49x faster**)   | `41.84 us` (❌ *5.64x slower*)       |
| **`768`**  | `15.80 us` (✅ **1.00x**)  | `6.10 us` (🚀 **2.59x faster**)   | `69.82 us` (❌ *4.42x slower*)    | `25.21 us` (❌ *1.60x slower*)    | `7.77 us` (🚀 **2.03x faster**)   | `83.56 us` (❌ *5.29x slower*)       |
| **`1024`** | `27.99 us` (✅ **1.00x**)  | `8.41 us` (🚀 **3.33x faster**)   | `117.39 us` (❌ *4.19x slower*)   | `44.22 us` (❌ *1.58x slower*)    | `10.78 us` (🚀 **2.60x faster**)  | `138.15 us` (❌ *4.94x slower*)      |
| **`1536`** | `63.70 us` (✅ **1.00x**)  | `13.32 us` (🚀 **4.78x faster**)  | `245.58 us` (❌ *3.86x slower*)   | `99.40 us` (❌ *1.56x slower*)    | `17.78 us` (🚀 **3.58x faster**)  | `283.51 us` (❌ *4.45x slower*)      |
| **`2048`** | `115.26 us` (✅ **1.00x**) | `18.78 us` (🚀 **6.14x faster**)  | `421.57 us` (❌ *3.66x slower*)   | `178.64 us` (❌ *1.55x slower*)   | `25.58 us` (🚀 **4.51x faster**)  | `486.59 us` (❌ *4.22x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

