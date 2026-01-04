// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use std::cmp::Ordering;

// --------------------------------------------------------------------------
// Float type
// --------------------------------------------------------------------------

/// Generic floating-point type, e.g, `f32` or `f64`
pub trait FloatType: Copy + Clone {
    fn cmp(&self, other: &Self) -> Ordering;
    fn is_nan(self) -> bool;
    fn midpoint(self, other: Self) -> Self;

    #[inline]
    fn eq(self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }

    #[inline]
    fn leq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }
}

impl FloatType for f32 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(!(self.is_nan() || other.is_nan()), "Value must not be NaN!");
        if *self == *other {
            return Ordering::Equal;
        }
        self.total_cmp(other)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    #[inline]
    fn midpoint(self, other: Self) -> Self {
        assert!(!(self.is_nan() || other.is_nan()), "Value must not be NaN!");
        match f32::midpoint(self, other) {
            value if value.is_nan() => f32::default(),
            value => value,
        }
    }
}

impl FloatType for f64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(!(self.is_nan() || other.is_nan()), "Value must not be NaN!");
        if *self == *other {
            return Ordering::Equal;
        }
        self.total_cmp(other)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    #[inline]
    fn midpoint(self, other: Self) -> Self {
        assert!(!(self.is_nan() || other.is_nan()), "Value must not be NaN!");
        match f64::midpoint(self, other) {
            value if value.is_nan() => f64::default(),
            value => value,
        }
    }
}

// --------------------------------------------------------------------------
// Ordered float
// --------------------------------------------------------------------------

/// Ordered floating-point wrapper type, extends on `FloatType`
#[derive(Debug, Clone, Copy)]
pub struct FloatOrd<T: FloatType>(pub T);

impl<T: FloatType> From<T> for FloatOrd<T> {
    #[inline]
    fn from(value: T) -> Self {
        assert!(!value.is_nan(), "Value must not be NaN!");
        Self(value)
    }
}

impl<T: FloatType> PartialEq for FloatOrd<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: FloatType> Eq for FloatOrd<T> {}

impl<T: FloatType> PartialOrd for FloatOrd<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: FloatType> Ord for FloatOrd<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
