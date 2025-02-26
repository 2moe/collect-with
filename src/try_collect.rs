use alloc::vec::Vec;

use crate::{ExtendWithCapacity, TryExtract, common::get_max_hint_bound};

impl<I: Iterator> TryCollectWith for I {}

pub trait TryCollectWith: Iterator {
  /// Attempt to collect iterator elements into a collection with error handling
  ///
  /// ## Closure
  ///
  /// * `capacity` - Closure that calculates initial capacity based on iterator
  ///   size hint
  ///
  /// ## Type Parameters
  ///
  /// * `T` - Target collection type implementing ExtendWithCapacity
  /// * `OK` - Type of successfully extracted values
  /// * `ERR` - Error type returned from failed extraction
  ///
  /// ## Behavior
  ///
  /// - Uses size hints to pre-allocate capacity
  /// - Short-circuits on first extraction error
  /// - Returns collected values or first encountered error
  ///
  /// ## Example
  ///
  /// ```rust
  /// use collect_with::TryCollectWith;
  ///
  /// let result = ["42", "76", "abc"]
  ///   .into_iter()
  ///   .map(|x| x.parse::<i32>()) // &str -> Result<i32>
  ///   .try_collect_with::<Vec<_>, _, _>(|u| u + 3); // -> Result<Vec<i32>, ParseIntError>
  ///
  /// assert!(result.is_err());
  /// ```
  fn try_collect_with<'a, T, OK, ERR>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> Result<T, ERR>
  where
    T: ExtendWithCapacity<OK>,
    Self: Sized,
    Self::Item: TryExtract<'a, Ok = OK, Err = ERR>,
  {
    let bound = get_max_hint_bound(self.size_hint());
    let mut container = T::with_capacity(capacity(bound).max(bound));

    for item in self {
      let value = item.try_extract()?;
      container.extend(core::iter::once(value));
    }

    Ok(container)
  }

  /// Convenience method for collecting into `Result<Vec<OK>, Err>`
  ///
  /// ## Closure
  ///
  /// - `capacity`
  ///   - Closure that calculates initial vector capacity
  ///
  /// ## Example
  ///
  /// ```rust
  /// use collect_with::TryCollectWith;
  ///
  /// let result = ["42", "73"]
  ///   .into_iter()
  ///   .map(|x| x.parse::<i32>()) // &str -> Result<i32>
  ///   .try_collect_vec_with(|u| u+2); // -> Result<Vec<i32>, ParseIntError>
  ///
  /// assert_eq!(result.as_deref(), Ok(&[42, 73][..]));
  /// ```
  #[cfg(feature = "collect_vec")]
  fn try_collect_vec_with<'a, OK, ERR>(
    self,
    capacity: impl FnOnce(usize) -> usize,
  ) -> Result<Vec<OK>, ERR>
  where
    Self: Sized,
    Self::Item: TryExtract<'a, Ok = OK, Err = ERR>,
  {
    self.try_collect_with(capacity)
  }
}
