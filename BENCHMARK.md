# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `339.65 ns` (✅ **1.00x**) | `552.19 ns` (❌ *1.63x slower*)   | `2.09 us` (❌ *6.15x slower*)     | `523.70 ns` (❌ *1.54x slower*)   | `611.99 ns` (❌ *1.80x slower*)   | `2.24 us` (❌ *6.61x slower*)        |
| **`128`**  | `863.34 ns` (✅ **1.00x**) | `1.11 us` (❌ *1.29x slower*)     | `4.91 us` (❌ *5.69x slower*)     | `1.26 us` (❌ *1.46x slower*)     | `1.23 us` (❌ *1.43x slower*)     | `5.36 us` (❌ *6.21x slower*)        |
| **`256`**  | `2.32 us` (✅ **1.00x**)   | `2.27 us` (✅ **1.02x faster**)   | `13.00 us` (❌ *5.60x slower*)    | `3.71 us` (❌ *1.60x slower*)     | `2.58 us` (✅ **1.11x slower**)   | `14.11 us` (❌ *6.07x slower*)       |
| **`512`**  | `7.42 us` (✅ **1.00x**)   | `4.69 us` (✅ **1.58x faster**)   | `37.35 us` (❌ *5.03x slower*)    | `11.83 us` (❌ *1.59x slower*)    | `5.34 us` (✅ **1.39x faster**)   | `41.81 us` (❌ *5.63x slower*)       |
| **`768`**  | `16.01 us` (✅ **1.00x**)  | `7.33 us` (🚀 **2.19x faster**)   | `73.28 us` (❌ *4.58x slower*)    | `25.55 us` (❌ *1.60x slower*)    | `8.27 us` (🚀 **1.93x faster**)   | `82.75 us` (❌ *5.17x slower*)       |
| **`1024`** | `28.01 us` (✅ **1.00x**)  | `10.18 us` (🚀 **2.75x faster**)  | `120.85 us` (❌ *4.32x slower*)   | `44.35 us` (❌ *1.58x slower*)    | `11.37 us` (🚀 **2.46x faster**)  | `138.17 us` (❌ *4.93x slower*)      |
| **`1536`** | `63.60 us` (✅ **1.00x**)  | `16.31 us` (🚀 **3.90x faster**)  | `250.61 us` (❌ *3.94x slower*)   | `99.82 us` (❌ *1.57x slower*)    | `18.51 us` (🚀 **3.44x faster**)  | `282.82 us` (❌ *4.45x slower*)      |
| **`2048`** | `114.99 us` (✅ **1.00x**) | `22.80 us` (🚀 **5.04x faster**)  | `427.73 us` (❌ *3.72x slower*)   | `180.48 us` (❌ *1.57x slower*)   | `26.43 us` (🚀 **4.35x faster**)  | `483.04 us` (❌ *4.20x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

