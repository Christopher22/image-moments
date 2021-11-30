use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

/// The type of floating-point number used to calculate the image moments.
pub trait Scalar:
    std::fmt::Debug
    + Copy
    + PartialEq
    + PartialOrd
    + AddAssign
    + MulAssign
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    /// The smallest incremental step of this floating-point number.
    const EPSILON: Self;

    /// The value of the scalar matching 0.0.
    const ZERO: Self;

    /// The value of the scalar matching 1.0.
    const ONE: Self;

    /// The value of the scalar matching 2.0.
    const TWO: Self;

    /// The value of the scalar matching 3.0.
    const THREE: Self;

    /// The value of the scalar matching 1/2.
    const F1_2: Self;

    /// The value of the scalar matching 1/6.
    const F1_6: Self;

    /// The value of the scalar matching 1/12.
    const F1_12: Self;

    /// The value of the scalar matching 1/20.
    const F1_20: Self;

    /// The value of the scalar matching 1/24.
    const F1_24: Self;

    /// The value of the scalar matching 1/60.
    const F1_60: Self;

    /// Computes the absolute value of self
    fn abs(self) -> Self;

    /// Fused multiply-add. Computes (self * a) + b with only one rounding error, yielding a more accurate result than an unfused multiply-add.
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Raises a number to an integer power.
    fn powi(self, n: i32) -> Self;

    /// Returns a number composed of the magnitude of self and the sign of sign.
    fn copysign(self, sign: Self) -> Self;

    /// Calculate the square root.
    fn sqrt(self) -> Self;
}

macro_rules! impl_scalar {
    ( $scalar:ty ) => {
        impl Scalar for $scalar {
            const EPSILON: Self = <$scalar>::EPSILON;
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const TWO: Self = 2.0;
            const THREE: Self = 3.0;
            const F1_2: Self = 1.0 / 2.0;
            const F1_6: Self = 1.0 / 6.0;
            const F1_12: Self = 1.0 / 12.0;
            const F1_20: Self = 1.0 / 20.0;
            const F1_24: Self = 1.0 / 24.0;
            const F1_60: Self = 1.0 / 60.0;

            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            #[inline(always)]
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }

            #[inline(always)]
            fn copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
        }
    };
}

impl_scalar!(f32);
impl_scalar!(f64);

/// A generalization over different possible representations of points.
/// Implementing this trait on custom structs ensure their seamless usage with this crate.
pub trait Point<S: Scalar>: Clone {
    /// The x position of the point casted into the required resolution.
    fn x(&self) -> S;
    /// The y position of the point casted into the required resolution.
    fn y(&self) -> S;
}

macro_rules! impl_point {
    ( $scalar:ty ) => {
        impl Point<$scalar> for ($scalar, $scalar) {
            #[inline(always)]
            fn x(&self) -> $scalar {
                self.0
            }

            #[inline(always)]
            fn y(&self) -> $scalar {
                self.1
            }
        }

        impl<'a> Point<$scalar> for &'a ($scalar, $scalar) {
            #[inline(always)]
            fn x(&self) -> $scalar {
                self.0
            }

            #[inline(always)]
            fn y(&self) -> $scalar {
                self.1
            }
        }

        impl Point<$scalar> for [$scalar; 2] {
            #[inline(always)]
            fn x(&self) -> $scalar {
                self[0]
            }

            #[inline(always)]
            fn y(&self) -> $scalar {
                self[1]
            }
        }

        impl<'a> Point<$scalar> for &'a [$scalar; 2] {
            #[inline(always)]
            fn x(&self) -> $scalar {
                self[0]
            }

            #[inline(always)]
            fn y(&self) -> $scalar {
                self[1]
            }
        }

        impl_point!(i8 as $scalar);
        impl_point!(u8 as $scalar);
        impl_point!(i16 as $scalar);
        impl_point!(u16 as $scalar);
        impl_point!(i32 as $scalar);
        impl_point!(u32 as $scalar);
        impl_point!(i64 as $scalar);
        impl_point!(u64 as $scalar);
        impl_point!(isize as $scalar);
        impl_point!(usize as $scalar);
    };
    ( $old_type:ty as $new_type:ty ) => {
        impl Point<$new_type> for ($old_type, $old_type) {
            #[inline(always)]
            fn x(&self) -> $new_type {
                self.0 as $new_type
            }

            #[inline(always)]
            fn y(&self) -> $new_type {
                self.1 as $new_type
            }
        }

        impl<'a> Point<$new_type> for &'a ($old_type, $old_type) {
            #[inline(always)]
            fn x(&self) -> $new_type {
                self.0 as $new_type
            }

            #[inline(always)]
            fn y(&self) -> $new_type {
                self.1 as $new_type
            }
        }

        impl Point<$new_type> for [$old_type; 2] {
            #[inline(always)]
            fn x(&self) -> $new_type {
                self[0] as $new_type
            }

            #[inline(always)]
            fn y(&self) -> $new_type {
                self[1] as $new_type
            }
        }

        impl<'a> Point<$new_type> for &'a [$old_type; 2] {
            #[inline(always)]
            fn x(&self) -> $new_type {
                self[0] as $new_type
            }

            #[inline(always)]
            fn y(&self) -> $new_type {
                self[1] as $new_type
            }
        }
    };
}

impl_point!(f32);
impl_point!(f64);
