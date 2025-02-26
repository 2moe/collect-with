use alloc::{
  collections::{BinaryHeap, VecDeque},
  string::String,
  vec::Vec,
};

/// A trait for collections that can be pre-allocated with specific capacity and
/// extended with elements.
///
/// This extends the standard [`Extend`] trait to provide a uniform interface
/// for collection types that support capacity pre-allocation.
///
/// Pre-allocating capacity can improve performance when the number of elements
/// is known in advance.
/// Due to the fact that in Rust iterators, [size_hint()](Iterator::size_hint)
/// might return `(0, None)`, pre-allocating capacity can be particularly
/// useful.
///
/// # Implementors
/// - alloc collections: `Vec<T>`, `String`, `VecDeque<T>`, `BinaryHeap<T>`
/// - std types (with `std` feature): `OsString`, `HashMap`, `HashSet`,
///   `PathBuf`
/// - AHash collections (with `ahash` & `std` features): `AHashMap`, `AHashSet`
pub trait ExtendWithCapacity<T>: Extend<T> {
  fn with_capacity(capacity: usize) -> Self;
}
// ----------

impl<T> ExtendWithCapacity<T> for Vec<T> {
  fn with_capacity(capacity: usize) -> Self {
    Vec::with_capacity(capacity)
  }
}

impl<T> ExtendWithCapacity<T> for String
where
  String: Extend<T>,
{
  fn with_capacity(capacity: usize) -> Self {
    String::with_capacity(capacity)
  }
}

impl<T> ExtendWithCapacity<T> for VecDeque<T> {
  fn with_capacity(capacity: usize) -> Self {
    VecDeque::with_capacity(capacity)
  }
}

impl<T: Ord> ExtendWithCapacity<T> for BinaryHeap<T> {
  fn with_capacity(capacity: usize) -> Self {
    BinaryHeap::with_capacity(capacity)
  }
}

// OsString

#[cfg(feature = "std")]
impl<T> ExtendWithCapacity<T> for std::ffi::OsString
where
  std::ffi::OsString: Extend<T>,
{
  fn with_capacity(capacity: usize) -> Self {
    std::ffi::OsString::with_capacity(capacity)
  }
}

// Hash{Map, Set}

#[cfg(feature = "std")]
impl<K: Eq + core::hash::Hash, V> ExtendWithCapacity<(K, V)>
  for std::collections::HashMap<K, V>
{
  fn with_capacity(capacity: usize) -> Self {
    std::collections::HashMap::with_capacity(capacity)
  }
}
#[cfg(feature = "std")]
impl<K: Eq + core::hash::Hash> ExtendWithCapacity<K>
  for std::collections::HashSet<K>
{
  fn with_capacity(capacity: usize) -> Self {
    std::collections::HashSet::with_capacity(capacity)
  }
}

// ahash{map, set}

#[cfg(feature = "ahash")]
impl<K: Eq + core::hash::Hash, V> ExtendWithCapacity<(K, V)>
  for ahash::AHashMap<K, V>
{
  fn with_capacity(capacity: usize) -> Self {
    ahash::AHashMap::with_capacity(capacity)
  }
}
#[cfg(feature = "ahash")]
impl<K: Eq + core::hash::Hash> ExtendWithCapacity<K> for ahash::AHashSet<K> {
  fn with_capacity(capacity: usize) -> Self {
    ahash::AHashSet::with_capacity(capacity)
  }
}

// PathBuf

#[cfg(feature = "std")]
impl<T: AsRef<std::path::Path>> ExtendWithCapacity<T> for std::path::PathBuf {
  fn with_capacity(capacity: usize) -> Self {
    std::path::PathBuf::with_capacity(capacity)
  }
}

// index{map, set}

#[cfg(feature = "indexmap")]
impl<K: Eq + core::hash::Hash, V> ExtendWithCapacity<(K, V)>
  for indexmap::IndexMap<K, V>
{
  fn with_capacity(capacity: usize) -> Self {
    indexmap::IndexMap::with_capacity(capacity)
  }
}

#[cfg(feature = "indexmap")]
impl<K: Eq + core::hash::Hash> ExtendWithCapacity<K> for indexmap::IndexSet<K> {
  fn with_capacity(capacity: usize) -> Self {
    indexmap::IndexSet::with_capacity(capacity)
  }
}
