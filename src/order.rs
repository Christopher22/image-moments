use crate::{Scalar, Storage};

/// An order of image moments specified at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Order<const O: usize>;

/// An order currently supported by this crate for calculating image moments.
pub trait SupportedOrder<T: Scalar> {
    /// The underlying data storage.
    type Storage: Storage<T>;
}

impl<T: Scalar> SupportedOrder<T> for Order<0> {
    type Storage = [T; crate::storage::calculate_space::<0>()];
}

impl<T: Scalar> SupportedOrder<T> for Order<1> {
    type Storage = [T; crate::storage::calculate_space::<1>()];
}

impl<T: Scalar> SupportedOrder<T> for Order<2> {
    type Storage = [T; crate::storage::calculate_space::<2>()];
}

impl<T: Scalar> SupportedOrder<T> for Order<3> {
    type Storage = [T; crate::storage::calculate_space::<3>()];
}
