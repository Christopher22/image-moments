use crate::{implementation::Storage, Order, Scalar};

/// Implementation for calculating the normalized central moments.
pub trait NormalizedCentralMoments<T: Scalar> {
    type NormalizedCentralIntermediateResult;

    /// Calculate the normalized central moments.
    /// `input` must contain the central moments of the appropriate order.
    /// `output` must be filled with zeros, again in appropriate order.
    fn calculate_normalized_central_moments<S: Storage<T>>(
        spatial: &S,
        central: &mut S,
    ) -> Self::NormalizedCentralIntermediateResult;
}

impl<T: Scalar> NormalizedCentralMoments<T> for Order<0> {
    type NormalizedCentralIntermediateResult = T;
    fn calculate_normalized_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::NormalizedCentralIntermediateResult {
        *output.get_mut::<0, 0>() = T::ONE;

        let m00 = input.get::<0, 0>();
        match m00.abs() > T::EPSILON {
            true => T::ONE / m00,
            false => T::ZERO,
        }
    }
}

impl<T: Scalar> NormalizedCentralMoments<T> for Order<1> {
    type NormalizedCentralIntermediateResult = T;
    fn calculate_normalized_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::NormalizedCentralIntermediateResult {
        Order::<0>::calculate_normalized_central_moments(input, output)
        /*
         * Output is already zero
         * output.get_mut::<1, 0>() = T::ZERO;
         * output.get_mut::<0, 1>() = T::ZERO;
         */
    }
}

impl<T: Scalar> NormalizedCentralMoments<T> for Order<2> {
    type NormalizedCentralIntermediateResult = (T, T);
    fn calculate_normalized_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::NormalizedCentralIntermediateResult {
        let inv_m00 = Order::<1>::calculate_normalized_central_moments(input, output);
        let s2 = inv_m00.powi(2);

        *output.get_mut::<2, 0>() = input.get::<2, 0>() * s2;
        *output.get_mut::<1, 1>() = input.get::<1, 1>() * s2;
        *output.get_mut::<0, 2>() = input.get::<0, 2>() * s2;

        (inv_m00, s2)
    }
}

impl<T: Scalar> NormalizedCentralMoments<T> for Order<3> {
    type NormalizedCentralIntermediateResult = ();
    fn calculate_normalized_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::NormalizedCentralIntermediateResult {
        let (inv_m00, s2) = Order::<2>::calculate_normalized_central_moments(input, output);
        let inv_sqrt_m00 = inv_m00.abs().sqrt();
        let s3 = s2 * inv_sqrt_m00;

        *output.get_mut::<3, 0>() = input.get::<3, 0>() * s3;
        *output.get_mut::<2, 1>() = input.get::<2, 1>() * s3;
        *output.get_mut::<1, 2>() = input.get::<1, 2>() * s3;
        *output.get_mut::<0, 3>() = input.get::<0, 3>() * s3;
    }
}
