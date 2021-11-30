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
mod order;
mod primitives;

pub use self::index::{Index, SupportedIndex};
pub use self::order::{Order, SupportedOrder};
pub use self::primitives::{Point, Scalar};
