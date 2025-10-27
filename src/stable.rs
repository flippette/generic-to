/// method-generic [`Into`].
pub trait To: Sized {
  fn to<T>(self) -> T
  where
    Self: Into<T>;
}

/// method-generic [`TryInto`].
pub trait TryTo: Sized {
  fn try_to<T, E>(self) -> Result<T, E>
  where
    Self: TryInto<T, Error = E>;
}

/// method-generic [`From`].
pub trait Fro: Sized {
  fn fro<T>(val: T) -> Self
  where
    Self: From<T>;
}

/// method-generic [`TryFrom`].
pub trait TryFro: Sized {
  fn try_fro<T, E>(val: T) -> Result<Self, E>
  where
    Self: TryFrom<T, Error = E>;
}

impl<T> To for T {
  fn to<U>(self) -> U
  where
    Self: Into<U>,
  {
    Into::into(self)
  }
}

impl<T> TryTo for T {
  fn try_to<U, E>(self) -> Result<U, E>
  where
    Self: TryInto<U, Error = E>,
  {
    TryInto::try_into(self)
  }
}

impl<T> Fro for T {
  fn fro<U>(val: U) -> Self
  where
    Self: From<U>,
  {
    From::from(val)
  }
}

impl<T> TryFro for T {
  fn try_fro<U, E>(val: U) -> Result<Self, E>
  where
    Self: TryFrom<U, Error = E>,
  {
    TryFrom::try_from(val)
  }
}
