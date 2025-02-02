# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `335.04 ns` (✅ **1.00x**) | `455.85 ns` (❌ *1.36x slower*)   | `2.02 us` (❌ *6.03x slower*)     | `518.16 ns` (❌ *1.55x slower*)   | `584.06 ns` (❌ *1.74x slower*)   | `2.22 us` (❌ *6.61x slower*)        |
| **`128`**  | `796.25 ns` (✅ **1.00x**) | `923.33 ns` (❌ *1.16x slower*)   | `4.66 us` (❌ *5.85x slower*)     | `1.28 us` (❌ *1.61x slower*)     | `1.17 us` (❌ *1.46x slower*)     | `5.54 us` (❌ *6.95x slower*)        |
| **`256`**  | `2.26 us` (✅ **1.00x**)   | `1.87 us` (✅ **1.21x faster**)   | `12.19 us` (❌ *5.39x slower*)    | `3.66 us` (❌ *1.62x slower*)     | `2.40 us` (✅ **1.06x slower**)   | `14.31 us` (❌ *6.32x slower*)       |
| **`512`**  | `7.39 us` (✅ **1.00x**)   | `3.94 us` (🚀 **1.87x faster**)   | `34.97 us` (❌ *4.73x slower*)    | `12.14 us` (❌ *1.64x slower*)    | `4.89 us` (✅ **1.51x faster**)   | `42.17 us` (❌ *5.71x slower*)       |
| **`768`**  | `15.80 us` (✅ **1.00x**)  | `6.05 us` (🚀 **2.61x faster**)   | `71.60 us` (❌ *4.53x slower*)    | `24.91 us` (❌ *1.58x slower*)    | `7.74 us` (🚀 **2.04x faster**)   | `83.60 us` (❌ *5.29x slower*)       |
| **`1024`** | `28.04 us` (✅ **1.00x**)  | `8.42 us` (🚀 **3.33x faster**)   | `117.21 us` (❌ *4.18x slower*)   | `44.33 us` (❌ *1.58x slower*)    | `10.68 us` (🚀 **2.62x faster**)  | `135.56 us` (❌ *4.83x slower*)      |
| **`1536`** | `62.75 us` (✅ **1.00x**)  | `13.33 us` (🚀 **4.71x faster**)  | `246.10 us` (❌ *3.92x slower*)   | `100.07 us` (❌ *1.59x slower*)   | `17.85 us` (🚀 **3.52x faster**)  | `285.17 us` (❌ *4.54x slower*)      |
| **`2048`** | `114.23 us` (✅ **1.00x**) | `18.79 us` (🚀 **6.08x faster**)  | `420.91 us` (❌ *3.68x slower*)   | `182.18 us` (❌ *1.59x slower*)   | `26.31 us` (🚀 **4.34x faster**)  | `494.44 us` (❌ *4.33x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

