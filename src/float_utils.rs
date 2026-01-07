// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

use std::{cmp::Ordering, fmt::Debug};

// --------------------------------------------------------------------------
// Error Type
// --------------------------------------------------------------------------

/// Indicates that the given value was invalid.
#[derive(Debug)]
pub struct InvalidValue;

// --------------------------------------------------------------------------
// Float type
// --------------------------------------------------------------------------

/// Generic floating-point type, e.g, `f32` or `f64`
pub trait FloatType: Copy + Clone + PartialEq + PartialOrd + Default + Debug {
    fn is_nan(self) -> bool;
    fn midpoint(self, other: Self) -> Self;
}

impl FloatType for f32 {
    #[inline]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    #[inline]
    fn midpoint(self, other: Self) -> Self {
        f32::midpoint(self, other)
    }
}

impl FloatType for f64 {
    #[inline]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    #[inline]
    fn midpoint(self, other: Self) -> Self {
        f64::midpoint(self, other)
    }
}

// --------------------------------------------------------------------------
// Ordered wrapper
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct FloatOrd<T: FloatType>(T);

impl<T: FloatType> FloatOrd<T> {
    #[inline]
    pub fn new(value: T) -> Result<Self, InvalidValue> {
        if value.is_nan() {
            return Err(InvalidValue);
        }
        Ok(Self(value))
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    #[inline]
    pub fn midpoint(self, other: Self) -> T {
        let val = self.0.midpoint(other.0);
        if !val.is_nan() {
            val
        } else {
            Default::default()
        }
    }
}

impl<T: FloatType> PartialEq for FloatOrd<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
