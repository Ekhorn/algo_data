# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [quick_sort](#quick_sort)

## Benchmark Results

### quick_sort

|              | `quick_sort_std`          | `quick_sort_v2`                   | `quick_sort_v3`                  | `quick_sort_v4`                  | `quick_sort_v5`                  | `quick_sort_v6`                   |
|:-------------|:--------------------------|:----------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`10`**     | `28.61 ns` (✅ **1.00x**)  | `374.57 ns` (❌ *13.09x slower*)   | `54.18 ns` (❌ *1.89x slower*)    | `143.38 ns` (❌ *5.01x slower*)   | `44.73 ns` (❌ *1.56x slower*)    | `42.89 ns` (❌ *1.50x slower*)     |
| **`100`**    | `475.13 ns` (✅ **1.00x**) | `7.04 us` (❌ *14.82x slower*)     | `601.34 ns` (❌ *1.27x slower*)   | `3.00 us` (❌ *6.32x slower*)     | `568.43 ns` (❌ *1.20x slower*)   | `531.28 ns` (❌ *1.12x slower*)    |
| **`1000`**   | `6.14 us` (✅ **1.00x**)   | `87.84 us` (❌ *14.31x slower*)    | `9.62 us` (❌ *1.57x slower*)     | `33.89 us` (❌ *5.52x slower*)    | `9.62 us` (❌ *1.57x slower*)     | `9.05 us` (❌ *1.47x slower*)      |
| **`10000`**  | `53.80 us` (✅ **1.00x**)  | `1.69 ms` (❌ *31.32x slower*)     | `257.36 us` (❌ *4.78x slower*)   | `345.06 us` (❌ *6.41x slower*)   | `248.13 us` (❌ *4.61x slower*)   | `250.60 us` (❌ *4.66x slower*)    |
| **`100000`** | `469.06 us` (✅ **1.00x**) | `47.39 ms` (❌ *101.02x slower*)   | `2.62 ms` (❌ *5.58x slower*)     | `3.26 ms` (❌ *6.94x slower*)     | `2.60 ms` (❌ *5.55x slower*)     | `2.68 ms` (❌ *5.71x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

