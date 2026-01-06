// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use itertools::Itertools;
use rolling_median::float_utils::{FloatOrd, FloatType};

// --------------------------------------------------------------------------
// Utility functions
// --------------------------------------------------------------------------

fn assert_arrays_equal<T: FloatType>(array_a: &[FloatOrd<T>], array_b: &[T]) {
    assert_eq!(array_a.len(), array_b.len());
    for (a, b) in array_a.iter().map(|val| val.into_inner()).zip(array_b.iter().copied()) {
        assert_eq!(a, b)
    }
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[test]
fn test_float_0a() {
    assert!(!FloatType::is_nan(0.0f32));
    assert!(!FloatType::is_nan(0.0f64));
}

#[test]
fn test_float_0b() {
    assert!(!FloatType::is_nan(f32::MAX));
    assert!(!FloatType::is_nan(f64::MAX));
}

#[test]
fn test_float_0c() {
    assert!(!FloatType::is_nan(f32::MIN));
    assert!(!FloatType::is_nan(f64::MIN));
}

#[test]
fn test_float_0d() {
    assert!(!FloatType::is_nan(f32::INFINITY));
    assert!(!FloatType::is_nan(f64::INFINITY));
}

#[test]
fn test_float_0e() {
    assert!(!FloatType::is_nan(f32::NEG_INFINITY));
    assert!(!FloatType::is_nan(f64::NEG_INFINITY));
}

#[test]
fn test_float_0f() {
    assert!(FloatType::is_nan(f32::NAN));
    assert!(FloatType::is_nan(f32::NAN));
}

#[test]
fn test_float_1a() {
    assert_eq!(FloatType::midpoint(0.0f32, 0.0f32), 0.0f32);
    assert_eq!(FloatType::midpoint(0.0f64, 0.0f64), 0.0f64);
}

#[test]
fn test_float_1b() {
    assert_eq!(FloatType::midpoint(1.0f32, -1.0f32), 0.0f32);
    assert_eq!(FloatType::midpoint(1.0f64, -1.0f64), 0.0f64);
}

#[test]
fn test_float_1c() {
    assert_eq!(FloatType::midpoint(1.0f32, 2.0f32), 1.5f32);
    assert_eq!(FloatType::midpoint(1.0f64, 2.0f64), 1.5f64);
}

#[test]
fn test_float_1d() {
    assert_eq!(FloatType::midpoint(-1.0f32, -2.0f32), -1.5f32);
    assert_eq!(FloatType::midpoint(-1.0f64, -2.0f64), -1.5f64);
}

#[test]
fn test_float_2a() {
    assert!(FloatOrd::new(0.0f32).is_ok());
    assert!(FloatOrd::new(0.0f64).is_ok());
}

#[test]
fn test_float_2b() {
    assert!(FloatOrd::new(f32::MAX).is_ok());
    assert!(FloatOrd::new(f64::MAX).is_ok());
}

#[test]
fn test_float_2c() {
    assert!(FloatOrd::new(f32::MIN).is_ok());
    assert!(FloatOrd::new(f64::MIN).is_ok());
}

#[test]
fn test_float_2d() {
    assert!(FloatOrd::new(f32::INFINITY).is_ok());
    assert!(FloatOrd::new(f64::INFINITY).is_ok());
}

#[test]
fn test_float_2e() {
    assert!(FloatOrd::new(f32::NEG_INFINITY).is_ok());
    assert!(FloatOrd::new(f64::NEG_INFINITY).is_ok());
}

#[test]
fn test_float_2f() {
    assert!(FloatOrd::new(f32::NAN).is_err());
    assert!(FloatOrd::new(f64::NAN).is_err());
}

#[test]
fn test_float_3a() {
    static VALUES: [f32; 11usize] = [
        f32::NEG_INFINITY,
        f32::MIN,
        -16777215.0f32,
        -1.0f32,
        -f32::MIN_POSITIVE,
        0.0f32,
        f32::MIN_POSITIVE,
        1.0f32,
        16777215.0f32,
        f32::MAX,
        f32::INFINITY,
    ];

    let ordered: Vec<FloatOrd<f32>> = VALUES.iter().copied().map(|val| FloatOrd::new(val).unwrap()).collect();
    for mut array in ordered.iter().copied().permutations(VALUES.len()) {
        array.sort();
        assert_arrays_equal(&array[..], &VALUES[..]);
    }
}

#[test]
fn test_float_3b() {
    static VALUES: [f64; 11usize] = [
        f64::NEG_INFINITY,
        f64::MIN,
        -9007199254740991.0f64,
        -1.0f64,
        -f64::MIN_POSITIVE,
        0.0f64,
        f64::MIN_POSITIVE,
        1.0f64,
        9007199254740991.0f64,
        f64::MAX,
        f64::INFINITY,
    ];

    let ordered: Vec<FloatOrd<f64>> = VALUES.iter().copied().map(|val| FloatOrd::new(val).unwrap()).collect();
    for mut array in ordered.iter().copied().permutations(VALUES.len()) {
        array.sort();
        assert_arrays_equal(&array[..], &VALUES[..]);
    }
}

#[test]
fn test_float_4a() {
    static VALUES: [f32; 11usize] = [
        f32::MIN,
        -16777215.0f32,
        -1.0f32,
        -f32::EPSILON,
        -f32::MIN_POSITIVE / 2.0f32,
        0.0f32,
        f32::MIN_POSITIVE / 2.0f32,
        f32::EPSILON,
        1.0f32,
        16777215.0f32,
        f32::MAX,
    ];

    let ordered: Vec<FloatOrd<f32>> = VALUES.iter().copied().map(|val| FloatOrd::new(val).unwrap()).collect();
    for mut array in ordered.iter().copied().permutations(VALUES.len()) {
        array.sort();
        assert_arrays_equal(&array[..], &VALUES[..]);
    }
}

#[test]
fn test_float_4b() {
    static VALUES: [f64; 11usize] = [
        f64::MIN,
        -9007199254740991.0f64,
        -1.0f64,
        -f64::EPSILON,
        -f64::MIN_POSITIVE / 2.0f64,
        0.0f64,
        f64::MIN_POSITIVE / 2.0f64,
        f64::EPSILON,
        1.0f64,
        9007199254740991.0f64,
        f64::MAX,
    ];

    let ordered: Vec<FloatOrd<f64>> = VALUES.iter().copied().map(|val| FloatOrd::new(val).unwrap()).collect();
    for mut array in ordered.iter().copied().permutations(VALUES.len()) {
        array.sort();
        assert_arrays_equal(&array[..], &VALUES[..]);
    }
}
