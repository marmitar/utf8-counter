# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `331.88 ns` (✅ **1.00x**) | `551.09 ns` (❌ *1.66x slower*)   | `2.05 us` (❌ *6.17x slower*)     | `517.28 ns` (❌ *1.56x slower*)   | `591.79 ns` (❌ *1.78x slower*)   | `2.21 us` (❌ *6.66x slower*)        |
| **`128`**  | `857.06 ns` (✅ **1.00x**) | `1.11 us` (❌ *1.29x slower*)     | `4.72 us` (❌ *5.51x slower*)     | `1.28 us` (❌ *1.49x slower*)     | `1.16 us` (❌ *1.35x slower*)     | `5.30 us` (❌ *6.19x slower*)        |
| **`256`**  | `2.33 us` (✅ **1.00x**)   | `2.25 us` (✅ **1.04x faster**)   | `12.28 us` (❌ *5.27x slower*)    | `3.69 us` (❌ *1.58x slower*)     | `2.40 us` (✅ **1.03x slower**)   | `14.29 us` (❌ *6.13x slower*)       |
| **`512`**  | `7.44 us` (✅ **1.00x**)   | `4.66 us` (✅ **1.60x faster**)   | `35.63 us` (❌ *4.79x slower*)    | `11.95 us` (❌ *1.61x slower*)    | `4.96 us` (✅ **1.50x faster**)   | `41.74 us` (❌ *5.61x slower*)       |
| **`768`**  | `16.10 us` (✅ **1.00x**)  | `7.21 us` (🚀 **2.23x faster**)   | `71.01 us` (❌ *4.41x slower*)    | `25.29 us` (❌ *1.57x slower*)    | `7.75 us` (🚀 **2.08x faster**)   | `83.19 us` (❌ *5.17x slower*)       |
| **`1024`** | `27.86 us` (✅ **1.00x**)  | `9.81 us` (🚀 **2.84x faster**)   | `117.82 us` (❌ *4.23x slower*)   | `44.57 us` (❌ *1.60x slower*)    | `10.69 us` (🚀 **2.61x faster**)  | `137.26 us` (❌ *4.93x slower*)      |
| **`1536`** | `63.82 us` (✅ **1.00x**)  | `15.68 us` (🚀 **4.07x faster**)  | `245.49 us` (❌ *3.85x slower*)   | `100.33 us` (❌ *1.57x slower*)   | `18.14 us` (🚀 **3.52x faster**)  | `286.90 us` (❌ *4.50x slower*)      |
| **`2048`** | `115.50 us` (✅ **1.00x**) | `21.83 us` (🚀 **5.29x faster**)  | `422.97 us` (❌ *3.66x slower*)   | `180.96 us` (❌ *1.57x slower*)   | `26.01 us` (🚀 **4.44x faster**)  | `484.38 us` (❌ *4.19x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

