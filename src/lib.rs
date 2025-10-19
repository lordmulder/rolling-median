// SPDX-License-Identifier: 0BSD
// rolling-median
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

#![allow(clippy::needless_doctest_main)]

//! # rolling-median
//!
//! Computes the [**median**](https://en.wikipedia.org/wiki/Median) of a data set, using a “rolling” (online) algorithm.
//!
//! It uses two heaps (a “min” heap and a “max” heap) to efficiently keep track of the “middle” element.
//!
//! ### Complexity:
//!
//! The `push()` opreration has a complexity of: **`O(log(n))`**
//!
//! The `get()` opreration has a complexity of: **`O(1)`**
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
//!         println!("Median, so far: {:?}", rolling_median.get())
//!     }
//!
//!     println!("Final median: {:?}", rolling_median.get())
//! }
//!
//! fn get_data() -> Option<f64> {
//!     None /* actually generate some data here! */
//! }
//! ```

use ordered_float::{FloatCore, OrderedFloat};
use std::{cmp::Reverse, collections::BinaryHeap};

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
    /// This operation has a complexity of **O(log(n))**.
    pub fn push(&mut self, value: T) {
        if self.heap_lo.peek().is_none_or(|peek| value <= peek.0) {
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
    }

    /// Get the current median
    ///
    /// This operation has a complexity of **O(1)**.
    pub fn get(&self) -> Option<T> {
        if self.heap_lo.is_empty() {
            None
        } else if self.heap_lo.len() == self.heap_hi.len() {
            let lo_top = *self.heap_lo.peek().unwrap();
            let hi_top = self.heap_hi.peek().unwrap().0;
            Some((lo_top.0 + hi_top.0) / T::from(2).unwrap())
        } else {
            Some(self.heap_lo.peek().unwrap().0)
        }
    }
}

impl<T: FloatCore> Default for Median<T> {
    /// Initializes a new rolling median computation
    fn default() -> Self {
        Self::new()
    }
}
