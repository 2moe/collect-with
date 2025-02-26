use crate::ExtendWithCapacity;

pub(crate) fn collect_iterator<T, I>(
  iter: I,
  exact: bool,
  capacity: impl FnOnce(usize) -> usize,
) -> T
where
  T: ExtendWithCapacity<I::Item>,
  I: Iterator,
{
  let bound = get_max_hint_bound(iter.size_hint());
  let real_capacity = match capacity(bound) {
    n if exact => n,
    n => n.max(bound),
  };
  collect_with_exact_capacity(iter, real_capacity)
}

pub(crate) fn collect_with_exact_capacity<T, I>(iter: I, capacity: usize) -> T
where
  T: ExtendWithCapacity<I::Item>,
  I: Iterator,
{
  let mut container = T::with_capacity(capacity);
  container.extend(iter);
  container
}

/// Calculate maximum potential element count from iterator size hints.
///
/// - `size_hint`
///   - Tuple containing lower and upper bounds from
///     [size_hint()](Iterator::size_hint())
///
/// Returns the maximum possible number of elements based on the hint.
pub(crate) fn get_max_hint_bound(size_hint: (usize, Option<usize>)) -> usize {
  match size_hint {
    // When upper bound exists, take maximum of lower and upper
    (lower, Some(upper)) => lower.max(upper),
    // When no upper bound, use lower bound
    (n, _) => n,
  }
}
