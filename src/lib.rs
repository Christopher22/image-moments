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

mod accumulator;
mod order;
mod primitives;
mod storage;

pub use self::order::{Order, SupportedOrder};
pub use self::primitives::{Point, Scalar};
