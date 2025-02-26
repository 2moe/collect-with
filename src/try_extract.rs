use core::ops::ControlFlow;

/// A trait for uniformly extracting success/error values from various container
/// types.
///
/// This provides a generic interface to convert different outcome-carrying
/// types into a standard [`Result`] form, enabling unified error handling
/// across:
/// - [`Result`] (by value and by reference)
/// - [`Option`] (by value and by reference)
/// - [`ControlFlow`] (by value and by reference)
///
/// The conversion preserves semantic meanings:
/// - Success cases map to `Ok`
/// - Termination/error cases map to `Err`
pub trait TryExtract<'a> {
  /// The type of success values (may be a reference)
  type Ok;
  /// The type of error values (may be a reference)
  type Err;

  /// Converts the container into a [`Result`], preserving semantic meaning
  ///
  /// For reference implementations (`&Result`, `&Option`, `&ControlFlow`),
  /// this returns references to contained values rather than moving them
  fn try_extract(self) -> Result<Self::Ok, Self::Err>;
}

// Core result implementations --------------------------------------
impl<'a, T, E> TryExtract<'a> for Result<T, E> {
  type Ok = T;
  type Err = E;

  /// Directly returns the owned Result (identity conversion)
  fn try_extract(
    self,
  ) -> Result<
    <Result<T, E> as TryExtract<'a>>::Ok,
    <Result<T, E> as TryExtract<'a>>::Err,
  > {
    self
  }
}
// Reference implementations ----------------------------------------
impl<'a, T, E> TryExtract<'a> for &'a Result<T, E> {
  type Ok = &'a T;
  type Err = &'a E;

  /// Extracts references to the contained value:
  /// - &Result<Ok, Err> => Result<&Ok, &Err>
  fn try_extract(self) -> Result<Self::Ok, Self::Err> {
    self.as_ref()
  }
}

// Option implementations -------------------------------------------
impl<T> TryExtract<'_> for Option<T> {
  type Ok = T;
  type Err = ();

  /// Converts an Option into a Result:
  /// - Some(value) => Ok(value)
  /// - None => Err(())
  ///
  /// This consumes the Option and moves the contained value
  fn try_extract(self) -> Result<Self::Ok, Self::Err> {
    self.ok_or(())
  }
}

impl<'a, T> TryExtract<'a> for &'a Option<T> {
  type Ok = &'a T;
  type Err = ();

  /// Extracts reference to the contained value if exists
  /// - &Option:
  ///   - Some(value) => Ok(&value)
  ///   - None => Err(())
  fn try_extract(self) -> Result<Self::Ok, Self::Err> {
    self.as_ref().ok_or(())
  }
}

// ControlFlow implementations --------------------------------------

impl<C, B> TryExtract<'_> for ControlFlow<B, C> {
  type Ok = C;
  type Err = B;

  /// Converts ControlFlow into Result:
  /// - Continue(c) => Ok(c)
  /// - Break(b) => Err(b)
  #[inline]
  fn try_extract(self) -> Result<Self::Ok, Self::Err> {
    use ControlFlow::*;
    match self {
      Continue(c) => Ok(c),
      Break(b) => Err(b),
    }
  }
}

impl<'a, C, B> TryExtract<'a> for &'a ControlFlow<B, C> {
  type Ok = &'a C;
  type Err = &'a B;

  /// Extracts references from ControlFlow:
  /// - Continue(c) => Ok(&c)
  /// - Break(b) => Err(&b)
  #[inline]
  fn try_extract(self) -> Result<Self::Ok, Self::Err> {
    match self {
      ControlFlow::Continue(c) => Ok(c),
      ControlFlow::Break(b) => Err(b),
    }
  }
}
