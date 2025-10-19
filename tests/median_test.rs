// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

use rand_pcg::{
    rand_core::{SeedableRng, TryRngCore},
    Pcg64,
};
use rolling_median::Median;

// --------------------------------------------------------------------------
// Utility functions
// --------------------------------------------------------------------------

fn compute_median(values: &Vec<u64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let len = values.len();
    let mut values = values.clone();
    values.sort();
    let (mid, rem) = (len / 2usize, len % 2usize);

    if rem == 0usize {
        Some(((values[mid - 1] as f64) + (values[mid] as f64)) / 2.0)
    } else {
        Some(values[mid] as f64)
    }
}

fn do_test(seed: u64, count: usize) {
    let mut median: Median<u64> = Median::new();
    let mut all_values: Vec<u64> = Vec::with_capacity(count);
    let mut random = Pcg64::seed_from_u64(seed);

    for _ in 0..count {
        let value = random.try_next_u64().unwrap();
        median.push(value);
        all_values.push(value);
    }

    assert_eq!(compute_median(&all_values), median.get());
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[test]
fn test_median_0() {
    do_test(0u64, 0usize);
}

#[test]
fn test_median_1a() {
    do_test(0u64, 1usize);
}

#[test]
fn test_median_1b() {
    do_test(1u64, 1usize);
}

#[test]
fn test_median_1c() {
    do_test(2u64, 1usize);
}

#[test]
fn test_median_2a() {
    do_test(0u64, 2usize);
}

#[test]
fn test_median_2b() {
    do_test(1u64, 2usize);
}

#[test]
fn test_median_2c() {
    do_test(2u64, 2usize);
}

#[test]
fn test_median_3a() {
    do_test(0u64, 3usize);
}

#[test]
fn test_median_3b() {
    do_test(1u64, 3usize);
}

#[test]
fn test_median_3c() {
    do_test(2u64, 3usize);
}

#[test]
fn test_median_4a() {
    do_test(0u64, 997usize);
}

#[test]
fn test_median_4b() {
    do_test(1u64, 997usize);
}

#[test]
fn test_median_4c() {
    do_test(2u64, 997usize);
}

#[test]
fn test_median_5a() {
    do_test(0u64, 998usize);
}

#[test]
fn test_median_5b() {
    do_test(1u64, 998usize);
}

#[test]
fn test_median_5c() {
    do_test(2u64, 998usize);
}
