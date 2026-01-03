// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use ordered_float::OrderedFloat;
use rand_pcg::{
    rand_core::{RngCore, SeedableRng, TryRngCore},
    Pcg64,
};
use rolling_median::Median;

// --------------------------------------------------------------------------
// Utility functions
// --------------------------------------------------------------------------

fn compute_median(values: &Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let len = values.len();
    let mut values: Vec<OrderedFloat<f64>> = values.iter().map(|val| OrderedFloat::from(*val)).collect();
    values.sort();
    let (mid, rem) = (len / 2usize, len % 2usize);

    if rem == 0usize {
        Some((values[mid - 1].0 + values[mid].0) / 2.0f64)
    } else {
        Some(values[mid].0)
    }
}

fn do_test_u64(seed: u64, count: usize) {
    let mut median = Median::new();
    let mut values: Vec<f64> = Vec::with_capacity(count);
    let mut random = Pcg64::seed_from_u64(seed);

    for _ in 0..count {
        let value = random.try_next_u64().unwrap() as f64;
        values.push(value);
        median.push(value);
    }

    assert_eq!(compute_median(&values), median.get());
}

fn do_test_f64(seed: u64, count: usize) {
    let mut median = Median::new();
    let mut values: Vec<f64> = Vec::with_capacity(count);
    let mut random = Pcg64::seed_from_u64(seed);

    for _ in 0..count {
        let value = (random.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64);
        values.push(value);
        median.push(value);
    }

    assert_eq!(compute_median(&values), median.get());
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[test]
fn test_median_0() {
    do_test_u64(0u64, 0usize);
}

#[test]
fn test_median_1a() {
    do_test_u64(0u64, 1usize);
}

#[test]
fn test_median_1b() {
    do_test_u64(1u64, 1usize);
}

#[test]
fn test_median_1c() {
    do_test_u64(2u64, 1usize);
}

#[test]
fn test_median_2a() {
    do_test_u64(0u64, 2usize);
}

#[test]
fn test_median_2b() {
    do_test_u64(1u64, 2usize);
}

#[test]
fn test_median_2c() {
    do_test_u64(2u64, 2usize);
}

#[test]
fn test_median_3a() {
    do_test_u64(0u64, 3usize);
}

#[test]
fn test_median_3b() {
    do_test_u64(1u64, 3usize);
}

#[test]
fn test_median_3c() {
    do_test_u64(2u64, 3usize);
}

#[test]
fn test_median_4a() {
    do_test_u64(0u64, 997usize);
}

#[test]
fn test_median_4b() {
    do_test_u64(1u64, 997usize);
}

#[test]
fn test_median_4c() {
    do_test_u64(2u64, 997usize);
}

#[test]
fn test_median_5a() {
    do_test_u64(0u64, 998usize);
}

#[test]
fn test_median_5b() {
    do_test_u64(1u64, 998usize);
}

#[test]
fn test_median_5c() {
    do_test_u64(2u64, 998usize);
}

#[test]
fn test_median_6a() {
    do_test_f64(0u64, 997usize);
}

#[test]
fn test_median_6b() {
    do_test_f64(1u64, 997usize);
}

#[test]
fn test_median_6c() {
    do_test_f64(2u64, 997usize);
}

#[test]
fn test_median_7a() {
    do_test_f64(0u64, 998usize);
}

#[test]
fn test_median_7b() {
    do_test_f64(1u64, 998usize);
}

#[test]
fn test_median_7c() {
    do_test_f64(2u64, 998usize);
}
