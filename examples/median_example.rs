// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use rolling_median::Median;

const VALUES: [f64; 10usize] = [3.27f64, 4.60f64, 5.95f64, 9.93f64, 7.79f64, 4.73f64, 3.33f64, 6.35f64, 4.97f64, 4.06f64];

fn main() {
    let mut rolling_median = Median::new();

    for value in VALUES {
        rolling_median.push(value).expect("Invalid value!");
        println!("Median, so far: {}", rolling_median.get().expect("No result!"))
    }

    println!("Final median: {}", rolling_median.get().expect("No result!"))
}
