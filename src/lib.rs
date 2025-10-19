// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

#![allow(clippy::needless_doctest_main)]

//! # rolling-median
//!
//! Computes the [**median**](https://en.wikipedia.org/wiki/Median) of a data set, using a "rolling" (online) algorithm.
//!
//! ### Complexity:
//!
//! The `push()` opreration has a complexity of: **O(log(n))**
//!
//! The `get()` opreration has a complexity of: **O(1)**
//!
//! ### Usage
//!
//! Here is a simple example that demonstrates how to use it in your code:
//!
//! ```rust
//! use rolling_median::Median;
//!
//! fn main() {
//!     let mut rolling_median = Median::new();
//!
//!     while let Some(value) = get_data() {
//!         rolling_median.push(value);
//!         println!("Median, so far: {:?}", rolling_median.get::<f64>())
//!     }
//!
//!     println!("Final median: {:?}", rolling_median.get::<f64>())
//! }
//!
//! fn get_data() -> Option<u32> {
//!     None /* actually generate some data here! */
//! }
//! ```

use num_traits::{Float, PrimInt};
use std::{cmp::Reverse, collections::BinaryHeap};

// --------------------------------------------------------------------------
// Rolling median
// --------------------------------------------------------------------------

/// Computes the median of a data set, using a "rolling" (online) algorithm
pub struct Median<T: PrimInt> {
    lo: BinaryHeap<T>,
    hi: BinaryHeap<Reverse<T>>,
}

impl<T: PrimInt> Median<T> {
    /// Initializes a new rolling median computation
    pub fn new() -> Self {
        Median { lo: BinaryHeap::new(), hi: BinaryHeap::new() }
    }

    /// Insert the next value
    ///
    /// This operation has a complexity of **O(log(n))**.
    pub fn push(&mut self, value: T) {
        if self.lo.peek().is_none_or(|peek| value <= *peek) {
            self.lo.push(value);
        } else {
            self.hi.push(Reverse(value));
        }

        if self.lo.len() > self.hi.len().checked_add(1usize).unwrap() {
            if let Some(value) = self.lo.pop() {
                self.hi.push(Reverse(value));
            }
        } else if self.hi.len() > self.lo.len() {
            if let Some(Reverse(value)) = self.hi.pop() {
                self.lo.push(value);
            }
        }
    }

    /// Get the current median
    ///
    /// This operation has a complexity of **O(1)**.
    pub fn get<U: Float>(&self) -> Option<U> {
        if self.lo.is_empty() {
            None
        } else if self.lo.len() == self.hi.len() {
            let lo_top = *self.lo.peek().unwrap();
            let hi_top = self.hi.peek().unwrap().0;
            Some((U::from(lo_top).unwrap() + U::from(hi_top).unwrap()) / U::from(2).unwrap())
        } else {
            Some(U::from(*self.lo.peek().unwrap()).unwrap())
        }
    }
}

impl<T: PrimInt> Default for Median<T> {
    /// Initializes a new rolling median computation
    fn default() -> Self {
        Self::new()
    }
}
