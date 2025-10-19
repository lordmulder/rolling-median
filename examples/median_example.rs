// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

use rolling_median::Median;
use std::sync::{LazyLock, Mutex};

fn main() {
    let mut rolling_median = Median::new();

    while let Some(value) = get_data() {
        rolling_median.push(value);
        println!("Median, so far: {}", rolling_median.get::<f64>().expect("No result!"))
    }

    println!("Final median: {}", rolling_median.get::<f64>().expect("No result!"))
}

fn get_data() -> Option<u32> {
    static VALUES: LazyLock<Mutex<Vec<u32>>> =
        LazyLock::new(|| Mutex::new(vec![93u32, 89u32, 17u32, 54u32, 44u32, 30u32, 55u32, 75u32, 49u32, 22u32]));
    VALUES.lock().unwrap().pop()
}
