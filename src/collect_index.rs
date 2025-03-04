use core::hash::Hash;

pub use indexmap::{IndexMap, IndexSet};

use crate::collect::CollectWith;

impl<I: Iterator> CollectIndex for I {}

/// Trait for collecting items into IndexMap or IndexSet with specified capacity
/// while preserving insertion order.
pub trait CollectIndex: Iterator {
  /// Collects items into an `IndexMap` with a specified capacity.
  ///
  /// Preserves insertion order and allows lookups by key.
  ///
  /// ## Example
  ///
  /// ```
  /// use indexmap::IndexMap;
  /// use collect_with::CollectIndex;
  ///
  /// let map = ('a'..='i')
  ///   .zip(100..=109)
  ///   .collect_indexmap_with(|u| u + 1); // u + 1 => 9 + 1 = 10
  ///
  /// assert_eq!(map.get(&'a'), Some(&100));
  /// assert_eq!(map.get_index(0), Some((&'a', &100)));
  /// assert_eq!(map.get_index(2), Some((&'c', &102)));
  /// assert_eq!(map.capacity(), 10);
  /// ```
  #[cfg(not(feature = "ahash"))]
  fn collect_indexmap_with<K, V>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `IndexMap<K, V, ahash::RandomState>` with a
  /// specified capacity.
  ///
  /// ## Example
  ///
  /// ```
  /// use indexmap::IndexMap;
  /// use collect_with::CollectIndex;
  ///
  /// let map = ('a'..='i')
  ///   .zip(100..=109)
  ///   .collect_indexmap_with(|u| u + 1); // u + 1 => 9 + 1 = 10
  ///
  /// assert_eq!(map.get(&'a'), Some(&100));
  /// assert_eq!(map.get_index(0), Some((&'a', &100)));
  /// assert_eq!(map.get_index(2), Some((&'c', &102)));
  /// assert_eq!(map.capacity(), 10);
  /// ```
  #[cfg(feature = "ahash")]
  fn collect_indexmap_with<K, V>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexMap<K, V, ahash::RandomState>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `IndexMap` with exact specified capacity.
  ///
  /// Preserves insertion order and strictly uses calculated capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use indexmap::IndexMap;
  /// use collect_with::CollectIndex;
  ///
  /// let map = [(1, "a"), (2, "b"), (3, "c")]
  ///     .into_iter()
  ///     .collect_indexmap_with_exact(|size_hint| size_hint);
  /// assert_eq!(map.len(), 3);
  /// assert_eq!(map.capacity(), 3);
  /// ```
  fn collect_indexmap_with_exact<K, V>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect_with_exact(capacity)
  }

  /// Collects items into an `IndexSet` with specified capacity.
  ///
  /// Preserves insertion order and maintains unique elements.
  ///
  /// # Example
  ///
  /// ```
  /// use indexmap::IndexSet;
  /// use collect_with::CollectIndex;
  ///
  /// let set = (0..3)
  ///     .collect_indexset_with(|size_hint| size_hint + 2);
  /// assert_eq!(set.len(), 3);
  /// assert!(set.capacity() >= 3);
  /// ```
  #[cfg(not(feature = "ahash"))]
  fn collect_indexset_with<K>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexSet<K>
  where
    Self: Sized + Iterator<Item = K>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `IndexSet<K, ahash::RandomState>` with specified
  /// capacity.
  ///
  /// Preserves insertion order and maintains unique elements.
  ///
  /// # Example
  ///
  /// ```
  /// use indexmap::IndexSet;
  /// use collect_with::CollectIndex;
  ///
  /// let set = (0..3)
  ///     .collect_indexset_with(|size_hint| size_hint + 2);
  /// assert_eq!(set.len(), 3);
  /// assert!(set.capacity() >= 3);
  /// ```
  #[cfg(feature = "ahash")]
  fn collect_indexset_with<K>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexSet<K, ahash::RandomState>
  where
    Self: Sized + Iterator<Item = K>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `IndexSet` with exact specified capacity.
  ///
  /// Preserves insertion order and strictly uses calculated capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use indexmap::IndexSet;
  /// use collect_with::CollectIndex;
  ///
  /// let set = (0..3)
  ///     .into_iter()
  ///     .collect_indexset_with_exact(|size_hint| size_hint);
  /// assert_eq!(set.len(), 3);
  /// assert_eq!(set.capacity(), 3);
  /// ```
  fn collect_indexset_with_exact<K>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> IndexSet<K>
  where
    Self: Sized + Iterator<Item = K>,
    K: Hash + Eq,
  {
    self.collect_with_exact(capacity)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::TryCollectWith;

  #[ignore]
  #[test]
  fn test_index_map() {
    let map = (1..=9)
      .zip('a'..='i')
      .collect_indexmap_with(|u| u + 1);
    assert_eq!(map.get(&1), Some(&'a'));
    assert_eq!(map.capacity(), 10);

    assert_eq!(map.get_index(2), Some((&3, &'c')));
  }

  #[ignore]
  #[test]
  fn test_indexmap2() {
    let result = ["42", "76", "03"]
      .into_iter()
      // &str -> Result<i32>
      .map(|x| x.parse::<i32>())
      .try_collect_with::<Vec<_>, _, _>(|u| u + 3); // Result<Vec<i32>, ParseIntError>
    assert_eq!(
      result
        .expect("Invalid IndexMap")
        .capacity(),
      6
    );
    // dbg!(result);
  }
}
