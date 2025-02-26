use core::hash::Hash;

pub use ahash::{AHashMap, AHashSet};

use crate::collect::CollectWith;

// pub trait CollectHashContainer: Iterator {
//   type Container<K, V>: crate::ExtendWithCapacity<(K, V)>;

//   fn collect_map_with<K, V>(
//     self,
//     capacity: impl FnOnce(usize) -> usize,
//   ) -> Self::Container<K, V>
//   where
//     Self: Sized + Iterator<Item = (K, V)>,
//     K: Hash + Eq;
// }

impl<I: Iterator> CollectAHash for I {}

/// Trait for collecting items into AHashMap or AHashSet with a specified
/// capacity.
pub trait CollectAHash: Iterator {
  /// Collects items into an `AHashMap` with a specified capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use ahash::AHashMap;
  /// use collect_with::CollectAHash;
  ///
  /// let map =  ('a'..='i')
  ///   .zip(1..=9)
  ///   .collect_ahashmap_with(|u| u + 5);
  /// assert_eq!(map.get(&'a'), Some(&1));
  /// assert_eq!(map.len(), 9);
  /// assert_eq!(map.capacity(), 14);
  /// ```
  fn collect_ahashmap_with<K, V>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> AHashMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `AHashMap` with an exact specified capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use ahash::AHashMap;
  /// use collect_with::CollectAHash;
  ///
  /// let map = [(1, "a"), (2, "b"), (3, "c")]
  ///   .into_iter()
  ///   .collect_ahashmap_with_exact(|size_hint| size_hint);
  /// assert_eq!(map.len(), 3);
  /// ```
  fn collect_ahashmap_with_exact<K, V>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> AHashMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect_with_exact(capacity)
  }

  /// Collects items into an `AHashSet` with a specified capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use ahash::AHashSet;
  /// use collect_with::CollectAHash;
  ///
  /// let set = (0..3)
  ///   .collect_ahashset_with(|size_hint| size_hint);
  /// assert_eq!(set.len(), 3);
  /// ```
  fn collect_ahashset_with<K>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> AHashSet<K>
  where
    Self: Sized + Iterator<Item = K>,
    K: Hash + Eq,
  {
    self.collect_with(capacity)
  }

  /// Collects items into an `AHashSet` with an exact specified capacity.
  ///
  /// # Example
  ///
  /// ```
  /// use ahash::AHashSet;
  /// use collect_with::CollectAHash;
  ///
  /// let set = (0..3)
  ///   .into_iter()
  ///   .collect_ahashset_with_exact(|size_hint| size_hint);
  /// assert_eq!(set.len(), 3);
  /// ```
  fn collect_ahashset_with_exact<K>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> AHashSet<K>
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

  #[ignore]
  #[test]
  fn dbg_collect_ahash_map() {
    let map = ('a'..='i')
      .zip(1..20)
      .collect_ahashmap_with(|x| {
        dbg!(x);
        x
      });
    assert_eq!(map.get(&'a'), Some(&1));
    dbg!(map);
  }

  #[ignore]
  #[test]
  fn dbg_collect_ahash_set() {
    let map = ('a'..='i')
      .zip(1..=9)
      .collect_ahashset_with(|x| {
        dbg!(x);
        x
      });
    dbg!(map);
  }
}
