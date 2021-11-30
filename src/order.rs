use crate::{accumulator::SealedSupportedOrder, Scalar};

/// An order of image moments specified at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Order<const O: usize>;

/// An order currently supported by this crate for calculating image moments.
/// The trait is sealed and the actual implementation details hidden from the user.
pub trait SupportedOrder<T: Scalar>: SealedSupportedOrder<T> {}

impl<T: Scalar> SupportedOrder<T> for Order<0> {}
impl<T: Scalar> SupportedOrder<T> for Order<1> {}
impl<T: Scalar> SupportedOrder<T> for Order<2> {}
impl<T: Scalar> SupportedOrder<T> for Order<3> {}
