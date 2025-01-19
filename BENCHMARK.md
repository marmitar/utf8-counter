# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`          | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------|:----------------------------------- |
| **`64`**   | `757.63 ns` (✅ **1.00x**) | `799.46 ns` (✅ **1.06x slower**) | `2.48 us` (❌ *3.27x slower*)     | `1.06 us` (❌ *1.39x slower*)     | `1.44 us` (❌ *1.91x slower*)    | `3.59 us` (❌ *4.74x slower*)        |
| **`128`**  | `1.63 us` (✅ **1.00x**)   | `1.59 us` (✅ **1.03x faster**)   | `5.82 us` (❌ *3.57x slower*)     | `2.29 us` (❌ *1.40x slower*)     | `2.88 us` (❌ *1.77x slower*)    | `8.39 us` (❌ *5.14x slower*)        |
| **`256`**  | `3.89 us` (✅ **1.00x**)   | `3.24 us` (✅ **1.20x faster**)   | `14.49 us` (❌ *3.73x slower*)    | `5.75 us` (❌ *1.48x slower*)     | `6.60 us` (❌ *1.70x slower*)    | `20.70 us` (❌ *5.32x slower*)       |
| **`512`**  | `10.64 us` (✅ **1.00x**)  | `6.78 us` (✅ **1.57x faster**)   | `41.32 us` (❌ *3.89x slower*)    | `16.17 us` (❌ *1.52x slower*)    | `13.95 us` (❌ *1.31x slower*)   | `55.83 us` (❌ *5.25x slower*)       |
| **`768`**  | `20.52 us` (✅ **1.00x**)  | `10.35 us` (🚀 **1.98x faster**)  | `80.27 us` (❌ *3.91x slower*)    | `31.54 us` (❌ *1.54x slower*)    | `21.97 us` (✅ **1.07x slower**) | `105.37 us` (❌ *5.13x slower*)      |
| **`1024`** | `34.66 us` (✅ **1.00x**)  | `14.06 us` (🚀 **2.46x faster**)  | `132.00 us` (❌ *3.81x slower*)   | `53.81 us` (❌ *1.55x slower*)    | `30.91 us` (✅ **1.12x faster**) | `166.88 us` (❌ *4.81x slower*)      |
| **`1536`** | `81.19 us` (✅ **1.00x**)  | `22.44 us` (🚀 **3.62x faster**)  | `276.12 us` (❌ *3.40x slower*)   | `119.39 us` (❌ *1.47x slower*)   | `51.50 us` (✅ **1.58x faster**) | `346.26 us` (❌ *4.26x slower*)      |
| **`2048`** | `138.21 us` (✅ **1.00x**) | `31.33 us` (🚀 **4.41x faster**)  | `469.29 us` (❌ *3.40x slower*)   | `219.99 us` (❌ *1.59x slower*)   | `72.83 us` (🚀 **1.90x faster**) | `577.28 us` (❌ *4.18x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

