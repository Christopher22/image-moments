use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

/// The type of floating-point number used to calculate the image moments.
pub trait Scalar:
    Copy
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
}

macro_rules! impl_scalar {
    ( $scalar:ty ) => {
        impl Scalar for $scalar {
            const EPSILON: Self = <$scalar>::EPSILON;
            const ZERO: Self = 0.0;
            const ONE: Self = 2.0;
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
        }
    };
}

impl_scalar!(f32);
impl_scalar!(f64);

/// A generalization over different possible representations of points.
/// Implementing this trait on custom structs ensure their seamless usage with this crate.
pub trait Point<S: Scalar> {
    /// The x position of the point.
    fn x(&self) -> S;
    /// The y position of the point.
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
    };
}

impl_point!(f32);
impl_point!(f64);
