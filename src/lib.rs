// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025-2026 by LoRd_MuldeR <mulder2@gmx.de>

#![allow(clippy::needless_doctest_main)]
#![allow(clippy::unnecessary_map_or)]

//! # rolling-median
//!
//! Computes the [**median**](https://en.wikipedia.org/wiki/Median) of a data set, using a “rolling” (online) algorithm.
//!
//! It uses two heaps (a “min” heap and a “max” heap) to efficiently keep track of the “middle” element.
//!
//! ## Complexity
//!
//! The `push()` operation has a complexity of: **`O(log(n))`**
//!
//! The `get()` operation has a complexity of: **`O(1)`**
//!
//! ## Usage
//!
//! Here is a simple example that demonstrates how to use it in your code:
//!
//! ```rust
//! use rolling_median::Median;
//!
//! const VALUES: [f64; 6usize] = [3.27f64, 4.60f64, 5.95f64, 9.93f64, 7.79f64, 4.73f64];
//!
//! fn main() {
//!     let mut rolling_median = Median::new();
//!
//!     for value in VALUES {
//!         rolling_median.push(value).expect("Invalid value!");
//!         println!("Median, so far: {}", rolling_median.get().expect("No result!"))
//!     }
//!
//!     println!("Final median: {}", rolling_median.get().expect("No result!"))
//! }
//! ```

use ordered_float::{FloatCore, OrderedFloat};
use std::{cmp::Reverse, collections::BinaryHeap, convert::TryInto};

// --------------------------------------------------------------------------
// Error type
// --------------------------------------------------------------------------

/// Indicates that the given value was invalid.
#[derive(Debug)]
pub struct InvalidValue;

// --------------------------------------------------------------------------
// Utility function
// --------------------------------------------------------------------------

#[inline]
fn midpoint<T: FloatCore>(a: T, b: T) -> T {
    if size_of::<T>() == 4usize {
        T::from(a.to_f32().unwrap().midpoint(b.to_f32().unwrap())).unwrap()
    } else if size_of::<T>() == 8usize {
        T::from(a.to_f64().unwrap().midpoint(b.to_f64().unwrap())).unwrap()
    } else {
        unimplemented!("Unsupported floating-point type!");
    }
}

// --------------------------------------------------------------------------
// Rolling median
// --------------------------------------------------------------------------

/// Computes the median of a data set, using a "rolling" (online) algorithm
pub struct Median<T: FloatCore> {
    heap_lo: BinaryHeap<OrderedFloat<T>>,
    heap_hi: BinaryHeap<Reverse<OrderedFloat<T>>>,
}

impl<T: FloatCore> Median<T> {
    /// Initializes a new rolling median computation
    pub fn new() -> Self {
        Median { heap_lo: BinaryHeap::new(), heap_hi: BinaryHeap::new() }
    }

    /// Insert the next value
    ///
    /// Returns `Ok(())`, if the value was inserted, or `Err(InvalidValue)`, if an attempt to insert a non-finite value was made.
    ///
    /// This operation has a complexity of **`O(log(n))`**.
    pub fn push(&mut self, value: T) -> Result<(), InvalidValue> {
        if !value.is_finite() {
            return Err(InvalidValue);
        }

        if self.heap_lo.peek().map_or(true, |peek| value <= peek.0) {
            self.heap_lo.push(value.into());
        } else {
            self.heap_hi.push(Reverse(value.into()));
        }

        if self.heap_lo.len() > self.heap_hi.len().checked_add(1usize).unwrap() {
            if let Some(value) = self.heap_lo.pop() {
                self.heap_hi.push(Reverse(value));
            }
        } else if self.heap_hi.len() > self.heap_lo.len() {
            if let Some(Reverse(value)) = self.heap_hi.pop() {
                self.heap_lo.push(value);
            }
        }

        Ok(())
    }

    /// Get the current median
    ///
    /// Returns `Some(median_value)`, if at least one value was inserted; otherwise `None`.
    ///
    /// This operation has a complexity of **`O(1)`**.
    pub fn get(&self) -> Option<T> {
        if self.heap_lo.is_empty() {
            None
        } else if self.heap_lo.len() == self.heap_hi.len() {
            let lo_top = *self.heap_lo.peek().unwrap();
            let hi_top = self.heap_hi.peek().unwrap().0;
            Some(midpoint(lo_top.0, hi_top.0))
        } else {
            Some(self.heap_lo.peek().unwrap().0)
        }
    }

    /// Clear all values that have been pushed so far
    pub fn clear(&mut self) {
        self.heap_lo.clear();
        self.heap_hi.clear();
    }
}

impl<T: FloatCore> Default for Median<T> {
    /// Initializes a new rolling median computation
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatCore> TryInto<f32> for Median<T> {
    type Error = ();
    fn try_into(self) -> Result<f32, Self::Error> {
        match self.get() {
            Some(value) => value.to_f32().ok_or(()),
            None => Err(()),
        }
    }
}

impl<T: FloatCore> TryInto<f64> for Median<T> {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        match self.get() {
            Some(value) => value.to_f64().ok_or(()),
            None => Err(()),
        }
    }
}
