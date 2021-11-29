use std::ops::{Add, Mul, Sub};

/// The type of floating-point number used to calculate the image moments.
pub trait Scalar: Copy + Add<Self> + Sub<Self> + Mul<Self> {
    /// Two times the scalar.
    const TWO: Self;

    /// Three times the scalar.
    const THREE: Self;

    /// Fused multiply-add. Computes (self * a) + b with only one rounding error, yielding a more accurate result than an unfused multiply-add.
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl Scalar for f32 {
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
}

impl Scalar for f64 {
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
}

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
