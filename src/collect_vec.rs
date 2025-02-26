use alloc::vec::Vec;

use crate::collect::CollectWith;

/// Blanket implementation for iterators to add vector collection capabilities.
///
/// Provides `collect_vec_with()` and `collect_vec_with_exact()` methods
impl<I: Iterator> CollectVector for I {}

/// Trait providing enhanced vector collection strategies for iterators.
///
/// Essentially, it calls the `CollectWith` trait, simplifying
/// `.collect_with::<Vec<_>>()` to `.collect_vec_with()`.
///
/// Extends iterators with methods for collecting items into a `Vec` with
/// customizable capacity pre-allocation strategies.
pub trait CollectVector: Iterator {
  /// Collect iterator elements into a `Vec` with capacity calculation.
  ///
  /// - capacity:
  ///   - Closure that calculates capacity based on iterator size hints.
  /// - `|size_bound|`: `max(lower_bound, upper_bound)` from the iterator's
  ///   `size_hint()`
  ///
  /// ## About the Final Capacity Size
  ///
  /// For example, `(0..10).collect_vec_with(|_size_bound| 2)`:
  ///
  /// 1. `(0..10).size_hint()` returns `(10, Some(10))`.
  /// 2. _size_bound is 10.
  /// 3. The closure returns 2, the final capacity is `max(10, 2)` = 10.
  /// 4. The vector is created with Vec::with_capacity(10).
  ///
  /// If you need an exact capacity size, please use
  /// [.collect_vec_with_exact()](crate::CollectVector::collect_vec_with_exact)
  ///
  /// ## Example
  ///
  /// ```
  /// use collect_with::CollectVector;
  ///
  /// let nums = (0..10).collect_vec_with(|size_bound| size_bound + 2);
  /// assert_eq!(nums.capacity(), 12);
  /// ```
  ///
  /// Suitable when the exact element count is unknown but better-than-default
  /// pre-allocation is desired.
  fn collect_vec_with<F>(self, capacity: F) -> Vec<Self::Item>
  where
    F: FnOnce(usize) -> usize,
    Self: Sized,
  {
    self.collect_with(capacity)
  }

  /// Collect iterator elements into a Vec with exact capacity calculation.
  ///
  /// ## Example
  ///
  /// ```
  /// use collect_with::CollectVector;
  ///
  /// let _nums = (0..10)
  ///   .collect_vec_with_exact(|size_bound| match size_bound {
  ///     0 => 32,
  ///     u => u,
  /// });
  /// ```
  ///
  /// Preferred when iterator provides exact size information via size_hint()
  /// and precise allocation is critical.
  fn collect_vec_with_exact<F>(self, capacity: F) -> Vec<Self::Item>
  where
    F: FnOnce(usize) -> usize,
    Self: Sized,
  {
    self.collect_with_exact(capacity)
  }
}
