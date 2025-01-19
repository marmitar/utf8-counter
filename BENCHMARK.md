# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sequence](#sequence)

## Benchmark Results

### Sequence

|            | `Factorial`               | `Fibonacci`                      | `UTF8 Counter`                   | `Cumulative Factorial`           | `Cumulative Fibonacci`           | `Cumulative UTF8 Counter`           |
|:-----------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`64`**   | `801.07 ns` (✅ **1.00x**) | `610.97 ns` (✅ **1.31x faster**) | `5.40 us` (❌ *6.74x slower*)     | `983.88 ns` (❌ *1.23x slower*)   | `715.00 ns` (✅ **1.12x faster**) | `5.62 us` (❌ *7.02x slower*)        |
| **`128`**  | `1.87 us` (✅ **1.00x**)   | `1.24 us` (✅ **1.51x faster**)   | `12.84 us` (❌ *6.86x slower*)    | `2.28 us` (❌ *1.22x slower*)     | `1.44 us` (✅ **1.30x faster**)   | `13.43 us` (❌ *7.17x slower*)       |
| **`256`**  | `4.74 us` (✅ **1.00x**)   | `2.48 us` (🚀 **1.91x faster**)   | `29.28 us` (❌ *6.17x slower*)    | `5.89 us` (❌ *1.24x slower*)     | `2.88 us` (✅ **1.65x faster**)   | `32.40 us` (❌ *6.83x slower*)       |
| **`512`**  | `14.10 us` (✅ **1.00x**)  | `4.99 us` (🚀 **2.83x faster**)   | `77.53 us` (❌ *5.50x slower*)    | `18.55 us` (❌ *1.32x slower*)    | `5.88 us` (🚀 **2.40x faster**)   | `83.06 us` (❌ *5.89x slower*)       |
| **`768`**  | `29.94 us` (✅ **1.00x**)  | `7.59 us` (🚀 **3.94x faster**)   | `141.29 us` (❌ *4.72x slower*)   | `38.75 us` (❌ *1.29x slower*)    | `9.22 us` (🚀 **3.25x faster**)   | `153.41 us` (❌ *5.12x slower*)      |
| **`1024`** | `51.66 us` (✅ **1.00x**)  | `10.27 us` (🚀 **5.03x faster**)  | `224.11 us` (❌ *4.34x slower*)   | `70.07 us` (❌ *1.36x slower*)    | `12.85 us` (🚀 **4.02x faster**)  | `243.69 us` (❌ *4.72x slower*)      |
| **`1536`** | `121.40 us` (✅ **1.00x**) | `15.91 us` (🚀 **7.63x faster**)  | `478.39 us` (❌ *3.94x slower*)   | `159.88 us` (❌ *1.32x slower*)   | `20.63 us` (🚀 **5.88x faster**)  | `536.04 us` (❌ *4.42x slower*)      |
| **`2048`** | `213.62 us` (✅ **1.00x**) | `22.68 us` (🚀 **9.42x faster**)  | `805.20 us` (❌ *3.77x slower*)   | `296.63 us` (❌ *1.39x slower*)   | `30.10 us` (🚀 **7.10x faster**)  | `919.13 us` (❌ *4.30x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

