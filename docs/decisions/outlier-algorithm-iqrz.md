# Outlier algorithm

Which outlier detection algorithm to use?

## Decision

2026.02 - Use IQRZ


## Detail

**IQRZ** is an enhanced variant of the traditional Interquartile Range (IQR) method, designed to handle outlier detection in **highly stable datasets**, such as those found in ultra-stable benchmarks.

## The Problem: Tight Clustering and False Positives

In performance measurement scenarios, it's common to encounter **tight clustering**: data points are extremely concentrated around the median, with minimal dispersion (e.g., coefficient of variation < 1%). While this indicates high stability, it creates a critical issue for outlier detection:

> **Traditional dispersion-based methods (IQR, MAD, or Modified Z-Score) can become hyper-sensitive in tightly clustered datasets (in ultra-stable datasets ).**


Even minor, normal fluctuations can be incorrectly flagged as outliers, undermining the reliability of benchmark analysis.

## Why This Matters in Benchmarking

The goal of a benchmarking library is to detect **true anomalies**, like OS interruptions, context switches, or garbage collection, not to penalize the natural micro-variations of a stable system. Over-filtering due to tight clustering hides real-world performance characteristics and reduces the validity of comparisons.

## The IQRZ Solution: Adaptive Minimum Variability

IQRZ solves this by introducing a **minimum iqr floor** to the IQR calculation, preventing it from becoming too small in stable datasets.

## How IQRZ Works

### 1. Standard IQR
   - First Quartile (Q1)
   - Third Quartile (Q3)
   - IQR = Q3 - Q1
   
   ```rust
   let iqr = q3 - q1;
   ```

**example**

In very stable datasets:

- q1 = 6.000
- q3 = 6.004
- IQR = **0.004**

**The Problem**

Under the traditional IQR method (where the fence is 1.5×IQR), the Upper Bound for outliers would be:

```
6.004+(1.5×0.004) = 6.010 ns
```

Result: A sample taking 6.011 ns, only 0.011 ns slower than the Upper Bound, would be flagged as an outlier.


###  2. IQRZ Adjustment

   - First Quartile (Q1)
   - Third Quartile (Q3)
   - IRQZ = max(IQR, min_iqr_floor)

   To avoid over-sensitivity:
   ```rust
   let min_iqr_floor: f64 = 0.01 * q3; // 1% of Q3
   let iqrz = (q3 - q1).max(min_iqr_floor);
   ```

**example**

In very stable datasets:

- q1 = 6.000
- q3 = 6.004
- min_iqr_floor = 0.01 × 6.004 = 0.060
- IQRZ = max(0.004, 0.060) = 0.060

**The Solution**

Under the IQRZ method (where the fence is 1.5×IQRZ), the Upper Bound becomes: 
```
6.004+(1.5×0.060) = 6.094 ns
```

Result: A sample taking 6.011ns would NOT be flagged as an outlier.

### What Changed?
- Traditional IQR: Upper Bound = 6.010 ns
- IQRZ: Upper Bound = 6.094 ns

The detection threshold now scales with the dataset magnitude instead of collapsing under extremely low variance.

### Outlier Detection
   Uses Tukey’s Fences with the adjusted IQRZ:
   - Lower Bound = Q1 - 1.5 × IQRZ
   - Upper Bound = Q3 + 1.5 × IQRZ
   - Values outside these bounds are marked as outliers.

## Conclusion

This simple adjustment significantly reduces false positives in highly consistent data without sacrificing the detection of true anomalies.
