// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

use rolling_median::Median;
use std::sync::{LazyLock, Mutex};

fn main() {
    let mut rolling_median = Median::new();

    while let Some(value) = get_data() {
        rolling_median.push(value);
        println!("Median, so far: {}", rolling_median.get().expect("No result!"))
    }

    println!("Final median: {}", rolling_median.get().expect("No result!"))
}

fn get_data() -> Option<f64> {
    static VALUES: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| {
        Mutex::new(vec![3.27f64, 4.60f64, 5.95f64, 9.93f64, 7.79f64, 4.73f64, 3.33f64, 6.35f64, 4.97f64, 4.06f64])
    });
    VALUES.lock().unwrap().pop()
}
