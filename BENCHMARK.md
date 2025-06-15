# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `346.42 ns` (✅ **1.00x**) | `450.60 ns` (❌ *1.30x slower*)   | `2.06 us` (❌ *5.94x slower*)     | `518.15 ns` (❌ *1.50x slower*)   | `549.05 ns` (❌ *1.58x slower*)   | `2.31 us` (❌ *6.67x slower*)        |
| **`128`**  | `802.29 ns` (✅ **1.00x**) | `906.21 ns` (❌ *1.13x slower*)   | `4.63 us` (❌ *5.77x slower*)     | `1.26 us` (❌ *1.57x slower*)     | `1.09 us` (❌ *1.35x slower*)     | `5.41 us` (❌ *6.74x slower*)        |
| **`256`**  | `2.22 us` (✅ **1.00x**)   | `1.85 us` (✅ **1.20x faster**)   | `12.02 us` (❌ *5.42x slower*)    | `3.74 us` (❌ *1.69x slower*)     | `2.24 us` (✅ **1.01x slower**)   | `13.83 us` (❌ *6.23x slower*)       |
| **`512`**  | `7.26 us` (✅ **1.00x**)   | `3.84 us` (🚀 **1.89x faster**)   | `34.38 us` (❌ *4.74x slower*)    | `11.84 us` (❌ *1.63x slower*)    | `4.63 us` (✅ **1.57x faster**)   | `40.47 us` (❌ *5.57x slower*)       |
| **`768`**  | `15.68 us` (✅ **1.00x**)  | `6.11 us` (🚀 **2.57x faster**)   | `68.82 us` (❌ *4.39x slower*)    | `25.45 us` (❌ *1.62x slower*)    | `7.26 us` (🚀 **2.16x faster**)   | `80.60 us` (❌ *5.14x slower*)       |
| **`1024`** | `27.60 us` (✅ **1.00x**)  | `8.21 us` (🚀 **3.36x faster**)   | `115.07 us` (❌ *4.17x slower*)   | `44.44 us` (❌ *1.61x slower*)    | `10.08 us` (🚀 **2.74x faster**)  | `133.29 us` (❌ *4.83x slower*)      |
| **`1536`** | `62.82 us` (✅ **1.00x**)  | `13.19 us` (🚀 **4.76x faster**)  | `242.55 us` (❌ *3.86x slower*)   | `99.62 us` (❌ *1.59x slower*)    | `16.22 us` (🚀 **3.87x faster**)  | `278.03 us` (❌ *4.43x slower*)      |
| **`2048`** | `114.27 us` (✅ **1.00x**) | `18.55 us` (🚀 **6.16x faster**)  | `415.06 us` (❌ *3.63x slower*)   | `179.25 us` (❌ *1.57x slower*)   | `23.58 us` (🚀 **4.85x faster**)  | `475.17 us` (❌ *4.16x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

