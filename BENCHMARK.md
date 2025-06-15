# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `333.49 ns` (✅ **1.00x**) | `450.63 ns` (❌ *1.35x slower*)   | `2.08 us` (❌ *6.23x slower*)     | `531.40 ns` (❌ *1.59x slower*)   | `553.77 ns` (❌ *1.66x slower*)   | `2.31 us` (❌ *6.92x slower*)        |
| **`128`**  | `794.75 ns` (✅ **1.00x**) | `903.58 ns` (❌ *1.14x slower*)   | `4.79 us` (❌ *6.03x slower*)     | `1.27 us` (❌ *1.60x slower*)     | `1.09 us` (❌ *1.38x slower*)     | `5.50 us` (❌ *6.92x slower*)        |
| **`256`**  | `2.20 us` (✅ **1.00x**)   | `1.85 us` (✅ **1.19x faster**)   | `11.85 us` (❌ *5.38x slower*)    | `3.81 us` (❌ *1.73x slower*)     | `2.25 us` (✅ **1.02x slower**)   | `13.94 us` (❌ *6.32x slower*)       |
| **`512`**  | `7.40 us` (✅ **1.00x**)   | `3.84 us` (🚀 **1.93x faster**)   | `34.49 us` (❌ *4.66x slower*)    | `11.89 us` (❌ *1.61x slower*)    | `4.67 us` (✅ **1.58x faster**)   | `40.72 us` (❌ *5.50x slower*)       |
| **`768`**  | `15.81 us` (✅ **1.00x**)  | `5.92 us` (🚀 **2.67x faster**)   | `68.74 us` (❌ *4.35x slower*)    | `25.38 us` (❌ *1.61x slower*)    | `7.21 us` (🚀 **2.19x faster**)   | `80.68 us` (❌ *5.10x slower*)       |
| **`1024`** | `27.81 us` (✅ **1.00x**)  | `8.21 us` (🚀 **3.39x faster**)   | `115.10 us` (❌ *4.14x slower*)   | `44.30 us` (❌ *1.59x slower*)    | `10.10 us` (🚀 **2.75x faster**)  | `133.11 us` (❌ *4.79x slower*)      |
| **`1536`** | `62.98 us` (✅ **1.00x**)  | `13.14 us` (🚀 **4.79x faster**)  | `242.26 us` (❌ *3.85x slower*)   | `99.72 us` (❌ *1.58x slower*)    | `16.35 us` (🚀 **3.85x faster**)  | `277.56 us` (❌ *4.41x slower*)      |
| **`2048`** | `114.15 us` (✅ **1.00x**) | `18.53 us` (🚀 **6.16x faster**)  | `416.62 us` (❌ *3.65x slower*)   | `179.45 us` (❌ *1.57x slower*)   | `23.68 us` (🚀 **4.82x faster**)  | `476.58 us` (❌ *4.17x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

