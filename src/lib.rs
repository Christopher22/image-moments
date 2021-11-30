//! Efficient and compile-time checked calculations of image moments

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// The actual implementation details.
mod implementation;
mod index;
mod moments;
mod order;
mod primitives;
mod spatial;

pub use self::index::{Index, SupportedIndex};
pub use self::moments::Moments;
pub use self::order::{Order, SupportedOrder};
pub use self::primitives::{Point, Scalar};
pub use self::spatial::Spatial;
