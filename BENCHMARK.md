# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `332.84 ns` (✅ **1.00x**) | `452.69 ns` (❌ *1.36x slower*)   | `2.07 us` (❌ *6.21x slower*)     | `520.28 ns` (❌ *1.56x slower*)   | `554.81 ns` (❌ *1.67x slower*)   | `2.14 us` (❌ *6.44x slower*)        |
| **`128`**  | `800.25 ns` (✅ **1.00x**) | `903.71 ns` (❌ *1.13x slower*)   | `4.63 us` (❌ *5.79x slower*)     | `1.27 us` (❌ *1.59x slower*)     | `1.10 us` (❌ *1.37x slower*)     | `5.10 us` (❌ *6.37x slower*)        |
| **`256`**  | `2.30 us` (✅ **1.00x**)   | `1.85 us` (✅ **1.25x faster**)   | `11.62 us` (❌ *5.05x slower*)    | `3.69 us` (❌ *1.60x slower*)     | `2.26 us` (✅ **1.02x faster**)   | `13.62 us` (❌ *5.92x slower*)       |
| **`512`**  | `7.38 us` (✅ **1.00x**)   | `3.82 us` (🚀 **1.93x faster**)   | `33.86 us` (❌ *4.59x slower*)    | `12.02 us` (❌ *1.63x slower*)    | `4.65 us` (✅ **1.59x faster**)   | `40.30 us` (❌ *5.46x slower*)       |
| **`768`**  | `15.64 us` (✅ **1.00x**)  | `6.06 us` (🚀 **2.58x faster**)   | `68.24 us` (❌ *4.36x slower*)    | `25.08 us` (❌ *1.60x slower*)    | `7.30 us` (🚀 **2.14x faster**)   | `80.08 us` (❌ *5.12x slower*)       |
| **`1024`** | `27.62 us` (✅ **1.00x**)  | `8.22 us` (🚀 **3.36x faster**)   | `113.97 us` (❌ *4.13x slower*)   | `44.19 us` (❌ *1.60x slower*)    | `9.95 us` (🚀 **2.77x faster**)   | `132.47 us` (❌ *4.80x slower*)      |
| **`1536`** | `62.96 us` (✅ **1.00x**)  | `13.18 us` (🚀 **4.78x faster**)  | `241.98 us` (❌ *3.84x slower*)   | `99.28 us` (❌ *1.58x slower*)    | `16.23 us` (🚀 **3.88x faster**)  | `277.16 us` (❌ *4.40x slower*)      |
| **`2048`** | `113.91 us` (✅ **1.00x**) | `18.59 us` (🚀 **6.13x faster**)  | `416.02 us` (❌ *3.65x slower*)   | `179.75 us` (❌ *1.58x slower*)   | `23.50 us` (🚀 **4.85x faster**)  | `474.53 us` (❌ *4.17x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

