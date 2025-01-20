# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `330.49 ns` (✅ **1.00x**) | `457.95 ns` (❌ *1.39x slower*)   | `2.05 us` (❌ *6.20x slower*)     | `514.05 ns` (❌ *1.56x slower*)   | `576.24 ns` (❌ *1.74x slower*)   | `2.16 us` (❌ *6.54x slower*)        |
| **`128`**  | `799.29 ns` (✅ **1.00x**) | `919.91 ns` (❌ *1.15x slower*)   | `4.74 us` (❌ *5.93x slower*)     | `1.26 us` (❌ *1.58x slower*)     | `1.16 us` (❌ *1.45x slower*)     | `5.27 us` (❌ *6.59x slower*)        |
| **`256`**  | `2.35 us` (✅ **1.00x**)   | `1.88 us` (✅ **1.25x faster**)   | `11.92 us` (❌ *5.07x slower*)    | `3.63 us` (❌ *1.54x slower*)     | `2.36 us` (✅ **1.00x slower**)   | `14.10 us` (❌ *6.00x slower*)       |
| **`512`**  | `7.42 us` (✅ **1.00x**)   | `3.89 us` (🚀 **1.91x faster**)   | `35.36 us` (❌ *4.76x slower*)    | `11.67 us` (❌ *1.57x slower*)    | `4.88 us` (✅ **1.52x faster**)   | `41.60 us` (❌ *5.60x slower*)       |
| **`768`**  | `15.93 us` (✅ **1.00x**)  | `6.17 us` (🚀 **2.58x faster**)   | `69.63 us` (❌ *4.37x slower*)    | `25.07 us` (❌ *1.57x slower*)    | `7.65 us` (🚀 **2.08x faster**)   | `82.30 us` (❌ *5.17x slower*)       |
| **`1024`** | `27.64 us` (✅ **1.00x**)  | `8.29 us` (🚀 **3.33x faster**)   | `116.06 us` (❌ *4.20x slower*)   | `43.72 us` (❌ *1.58x slower*)    | `10.61 us` (🚀 **2.61x faster**)  | `136.45 us` (❌ *4.94x slower*)      |
| **`1536`** | `63.44 us` (✅ **1.00x**)  | `13.59 us` (🚀 **4.67x faster**)  | `243.26 us` (❌ *3.83x slower*)   | `98.10 us` (❌ *1.55x slower*)    | `17.79 us` (🚀 **3.57x faster**)  | `283.69 us` (❌ *4.47x slower*)      |
| **`2048`** | `114.65 us` (✅ **1.00x**) | `18.90 us` (🚀 **6.07x faster**)  | `418.76 us` (❌ *3.65x slower*)   | `177.02 us` (❌ *1.54x slower*)   | `25.62 us` (🚀 **4.47x faster**)  | `483.01 us` (❌ *4.21x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

