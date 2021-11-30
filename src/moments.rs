use crate::{Index, Scalar, SupportedIndex};

/// A generalization over different moments.
pub trait Moments<T: Scalar, const ORDER: usize> {
    /// Get the moment at a specific position checked at compile time.
    fn get<const I: usize, const J: usize>(&self) -> T
    where
        Index<I, J>: SupportedIndex<ORDER>;
}
