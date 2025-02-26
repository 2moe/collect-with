use crate::{ExtendWithCapacity, common::collect_iterator};

// Implement CollectWithCapacity trait for Iterator
impl<I: Iterator> CollectWithCapacity for I {}
/// Trait for collecting iterator elements into a collection with specified
/// capacity
pub trait CollectWithCapacity: Iterator {
  /// Collect iterator elements into a collection with pre-allocated capacity.
  ///
  /// - `capacity`
  ///   - Initial capacity to allocate for the collection
  /// - `T`
  ///   - Collection type that implements ExtendWithCapacity (e.g., `Vec<X>`,
  ///     String, `HashMap<K, V>`)
  ///
  /// > See also: [collect_with()](crate::CollectWith::collect_with)
  fn collect_with_capacity<T>(self, capacity: usize) -> T
  where
    T: ExtendWithCapacity<Self::Item>,
    Self: Sized,
  {
    collect_iterator(self, false, |_| capacity)
  }
}
/// Implement CollectWith trait for Iterator
impl<I: Iterator> CollectWith for I {}

/// Trait for collecting iterator elements with flexible capacity calculation
pub trait CollectWith: Iterator {
  /// Collect elements using a capacity calculated from a closure
  ///
  /// - `capacity`
  ///   - Closure that calculates capacity based on iterator size hints
  ///
  /// ## About the Final Capacity Size
  ///
  /// For example, `(0..10).collect_with::<Vec<_>>(|_size_bound| 2)`
  ///
  /// 1. `(0..10).size_hint()` returns `(10, Some(10))`.
  /// 2. _size_bound is 10.
  /// 3. The closure returns 2, the final capacity is `max(10, 2)` = 10.
  /// 4. The vector is created with Vec::with_capacity(10).
  ///
  /// ## Example
  ///
  /// ```
  /// use collect_with::CollectWith;
  ///
  /// let s = [vec!["a"], vec!["b", "c", "d"]]
  ///   .into_iter()
  ///   .flatten()
  ///   .collect_with::<String>(|size| match size {
  ///     0 => 8,
  ///     n => n,
  ///   });
  /// assert_eq!(s.len(), 4);
  /// assert_eq!(s.capacity(), 8);
  /// ```
  ///
  /// The collection may allocate more capacity than calculated if needed.
  /// If you need an exact capacity size, please use
  /// [collect_with_exact()](crate::CollectWith::collect_with_exact)
  fn collect_with<T>(self, capacity: impl FnOnce(usize) -> usize) -> T
  where
    T: ExtendWithCapacity<Self::Item>,
    Self: Sized,
  {
    collect_iterator(self, false, capacity)
  }

  /// Collect elements using exact capacity calculated from a closure.
  ///
  /// - `capacity`
  ///   - Closure that calculates exact capacity requirement
  ///
  /// The collection will strictly use the calculated capacity without
  /// overallocation.
  fn collect_with_exact<T>(self, capacity: impl FnOnce(usize) -> usize) -> T
  where
    T: ExtendWithCapacity<Self::Item>,
    Self: Sized,
  {
    collect_iterator(self, true, capacity)
  }
}

#[cfg(test)]
mod tests {
  use alloc::{string::String, vec};

  use super::*;

  #[ignore]
  #[test]
  fn test_collect_with() {
    let s = [vec!["a"], vec!["b", "c", "d"]]
      .into_iter()
      .flatten()
      .collect_with::<String>(|size| match size {
        0 => 8,
        n => n,
      });
    assert_eq!(s.len(), 4);
    assert_eq!(s.capacity(), 8);
  }
}
