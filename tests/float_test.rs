// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use rolling_median::float_utils::{FloatOrd, FloatType};
use std::{array::from_fn, fmt::Debug};

// --------------------------------------------------------------------------
// Utility functions
// --------------------------------------------------------------------------

fn create_array<const N: usize, T: FloatType>(values: &[T]) -> [FloatOrd<T>; N] {
    let mut iter = values.iter().map(|val| FloatOrd::new(*val).unwrap());
    from_fn(|_| iter.next().unwrap())
}

fn assert_arrays<T: FloatType + Debug>(array_a: &[FloatOrd<T>], array_b: &[T]) {
    assert_eq!(array_a.len(), array_b.len());
    for (a, b) in array_a.iter().zip(array_b.iter()) {
        assert_eq!(a.into_inner(), *b);
    }
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[test]
fn test_float_0a() {
    assert!(FloatOrd::new(0.0f32).is_ok());
}

#[test]
fn test_float_0b() {
    assert!(FloatOrd::new(0.0f64).is_ok());
}

#[test]
fn test_float_1a() {
    assert!(FloatOrd::new(f32::MAX).is_ok());
}

#[test]
fn test_float_1b() {
    assert!(FloatOrd::new(f64::MAX).is_ok());
}

#[test]
fn test_float_2a() {
    assert!(FloatOrd::new(f32::MIN).is_ok());
}

#[test]
fn test_float_2b() {
    assert!(FloatOrd::new(f64::MIN).is_ok());
}

#[test]
fn test_float_3a() {
    assert!(FloatOrd::new(f32::INFINITY).is_ok());
}

#[test]
fn test_float_3b() {
    assert!(FloatOrd::new(f64::INFINITY).is_ok());
}

#[test]
fn test_float_4a() {
    assert!(FloatOrd::new(f32::NEG_INFINITY).is_ok());
}

#[test]
fn test_float_4b() {
    assert!(FloatOrd::new(f64::NEG_INFINITY).is_ok());
}

#[test]
fn test_float_5a() {
    assert!(FloatOrd::new(f32::NAN).is_err());
}

#[test]
fn test_float_5b() {
    assert!(FloatOrd::new(f64::NAN).is_err());
}

#[test]
fn test_float_6a() {
    static VALUES: [f32; 9usize] = [f32::NEG_INFINITY, f32::MIN, -1.0f32, -f32::EPSILON, 0.0f32, f32::EPSILON, 1.0f32, f32::MAX, f32::INFINITY];
    let mut values: [FloatOrd<f32>; 9usize] = create_array(&VALUES);

    values.sort();
    assert_arrays(&values[..], &VALUES[..]);

    values.reverse();
    values.sort();
    assert_arrays(&values[..], &VALUES[..]);

    for shift in 0usize..VALUES.len() {
        values.rotate_right(shift);
        values.sort();
        assert_arrays(&values[..], &VALUES[..]);
    }

    for shift in 0usize..VALUES.len() {
        values.rotate_left(shift);
        values.sort();
        assert_arrays(&values[..], &VALUES[..]);
    }
}

#[test]
fn test_float_6b() {
    static VALUES: [f64; 9usize] = [f64::NEG_INFINITY, f64::MIN, -1.0f64, -f64::EPSILON, 0.0f64, f64::EPSILON, 1.0f64, f64::MAX, f64::INFINITY];
    let mut values: [FloatOrd<f64>; 9usize] = create_array(&VALUES);

    values.sort();
    assert_arrays(&values[..], &VALUES[..]);

    values.reverse();
    values.sort();
    assert_arrays(&values[..], &VALUES[..]);

    for shift in 0usize..VALUES.len() {
        values.rotate_right(shift);
        values.sort();
        assert_arrays(&values[..], &VALUES[..]);
    }

    for shift in 0usize..VALUES.len() {
        values.rotate_left(shift);
        values.sort();
        assert_arrays(&values[..], &VALUES[..]);
    }
}
