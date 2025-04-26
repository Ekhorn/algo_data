# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [quick_sort](#quick_sort)

## Benchmark Results

### quick_sort

|              | `quick_sort_std`          | `quick_sort_v2`                   | `quick_sort_v3`                  | `quick_sort_v4`                   |
|:-------------|:--------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**     | `26.38 ns` (✅ **1.00x**)  | `476.47 ns` (❌ *18.06x slower*)   | `65.28 ns` (❌ *2.47x slower*)    | `155.94 ns` (❌ *5.91x slower*)    |
| **`100`**    | `473.12 ns` (✅ **1.00x**) | `5.87 us` (❌ *12.40x slower*)     | `748.48 ns` (❌ *1.58x slower*)   | `2.93 us` (❌ *6.20x slower*)      |
| **`1000`**   | `5.95 us` (✅ **1.00x**)   | `86.24 us` (❌ *14.50x slower*)    | `9.43 us` (❌ *1.58x slower*)     | `35.02 us` (❌ *5.89x slower*)     |
| **`10000`**  | `50.85 us` (✅ **1.00x**)  | `1.71 ms` (❌ *33.69x slower*)     | `251.60 us` (❌ *4.95x slower*)   | `346.34 us` (❌ *6.81x slower*)    |
| **`100000`** | `456.53 us` (✅ **1.00x**) | `47.13 ms` (❌ *103.24x slower*)   | `2.85 ms` (❌ *6.25x slower*)     | `3.42 ms` (❌ *7.50x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

