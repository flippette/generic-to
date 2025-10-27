/// method-generic [`Into`].
pub const trait To: Sized {
  fn to<T>(self) -> T
  where
    Self: [const] Into<T>;
}

/// method-generic [`TryInto`].
pub const trait TryTo: Sized {
  fn try_to<T, E>(self) -> Result<T, E>
  where
    Self: [const] TryInto<T, Error = E>;
}

/// method-generic [`From`].
pub const trait Fro: Sized {
  fn fro<T>(val: T) -> Self
  where
    Self: [const] From<T>;
}

/// method-generic [`TryFrom`].
pub const trait TryFro: Sized {
  fn try_fro<T, E>(val: T) -> Result<Self, E>
  where
    Self: [const] TryFrom<T, Error = E>;
}

impl<T> const To for T {
  fn to<U>(self) -> U
  where
    Self: [const] Into<U>,
  {
    Into::into(self)
  }
}

impl<T> const TryTo for T {
  fn try_to<U, E>(self) -> Result<U, E>
  where
    Self: [const] TryInto<U, Error = E>,
  {
    TryInto::try_into(self)
  }
}

impl<T> const Fro for T {
  fn fro<U>(val: U) -> Self
  where
    Self: [const] From<U>,
  {
    From::from(val)
  }
}

impl<T> const TryFro for T {
  fn try_fro<U, E>(val: U) -> Result<Self, E>
  where
    Self: [const] TryFrom<U, Error = E>,
  {
    TryFrom::try_from(val)
  }
}
