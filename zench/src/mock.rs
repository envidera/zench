//use std::time::{Duration, Instant};

use crate::bx;

/// Generates a synthetic dataset for benchmarking outlier detection for IQR, IQRZ and Modified Z-Score algorithms.
///
/// It creates layers of noise and anomalies, allowing each algorithm
/// to demonstrate different sensitivity levels based on their statistical logic.
#[allow(unused)]
pub fn generate_data(len: usize) -> Vec<f64> {
    let mut seed = 123456789u64;
    let mut data = Vec::with_capacity(len);
    let base = 1000.0;

    for i in 0..len {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        let noise = ((seed >> 11) as f64) / ((1u64 << 53) as f64);

        // 1. Base data with a little more "spreading"
        let mut value = base + (noise * 0.1);

        // 2. Frequent noise (30% of the data)
        // This will expand the Q3 and IQR, but the Median (50%) will still be the 'base'
        if i % 3 == 0 {
            value += 1.5;
        }

        // 3. The "Point of Divergence" (every 100 items)
        // We will inject a value that the IQR will consider "normal" (because the IQR increased in step 2)
        // But the Modified Z-Score will consider it an "Outlier" (because the Median didn't change that much)
        if i % 100 == 0 && i != 0 {
            value += 8.0;
        }

        // 4. Extreme Outlier (Everyone gets it)
        if i % 500 == 0 && i != 0 {
            value += 100.0;
        }

        data.push(value);
    }
    data
}

#[allow(unused)]
pub fn simulate_cpu_work() -> u64 {
    let mut sum = 0u64;
    for i in 0..50 {
        let val = bx(i);
        sum = sum.wrapping_add(val * val);
    }
    sum
}

// pub fn simulate_cpu_work_duration(duration: Duration) -> u64 {
//     let mut sum = 0u64;
//     let mut i = 1;

//     let start_time = Instant::now();
//     loop {
//         let val = bx(i);
//         sum = sum.wrapping_add(val * val);
//         i += 1;

//         // condition
//         let timeout = start_time.elapsed() >= duration;
//         if timeout {
//             break;
//         }
//     }

//     sum
// }

#[allow(unused)]
pub mod fibonacci {

    pub fn slow(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => slow(n - 1) + slow(n - 2),
        }
    }

    pub fn fast(n: u64) -> u64 {
        let mut a = 0;
        let mut b = 1;

        match n {
            0 => b,
            _ => {
                for _ in 0..n {
                    let c = a + b;
                    a = b;
                    b = c;
                }
                b
            }
        }
    }
}
