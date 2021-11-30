use crate::{implementation::Storage, Order, Scalar};

/// Implementation for calculating the central moments.
pub trait CentralMoments<T: Scalar> {
    type CentralIntermediateResult;

    /// Calculate the central moments inplace.
    /// `input` must contain the raw spatial moments of the appropriate order.
    /// `output` must be filled with zeros, again in appropriate order.
    fn calculate_central_moments<S: Storage<T>>(
        spatial: &S,
        central: &mut S,
    ) -> Self::CentralIntermediateResult;
}

impl<T: Scalar> CentralMoments<T> for Order<0> {
    type CentralIntermediateResult = ();
    fn calculate_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::CentralIntermediateResult {
        *output.get_mut::<0, 0>() = input.get::<0, 0>();
    }
}

impl<T: Scalar> CentralMoments<T> for Order<1> {
    type CentralIntermediateResult = ();
    fn calculate_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::CentralIntermediateResult {
        Order::<0>::calculate_central_moments(input, output);
        /*
         * Output is already zero
         * output.get_mut::<1, 0>() = T::ZERO;
         * output.get_mut::<0, 1>() = T::ZERO;
         */
    }
}

impl<T: Scalar> CentralMoments<T> for Order<2> {
    type CentralIntermediateResult = (T, T);

    fn calculate_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::CentralIntermediateResult {
        Order::<1>::calculate_central_moments(input, output);

        let m00 = input.get::<0, 0>().abs();
        let (cx, cy) = match m00.abs() > T::EPSILON {
            true => (input.get::<1, 0>() / m00, input.get::<0, 1>() / m00),
            false => (T::ZERO, T::ZERO),
        };

        *output.get_mut::<2, 0>() = input.get::<2, 0>() - input.get::<1, 0>() * cx;
        *output.get_mut::<1, 1>() = input.get::<1, 1>() - input.get::<1, 0>() * cy;
        *output.get_mut::<0, 2>() = input.get::<0, 2>() - input.get::<0, 1>() * cy;

        (cx, cy)
    }
}

impl<T: Scalar> CentralMoments<T> for Order<3> {
    type CentralIntermediateResult = ();

    fn calculate_central_moments<S: Storage<T>>(
        input: &S,
        output: &mut S,
    ) -> Self::CentralIntermediateResult {
        let (cx, cy) = Order::<2>::calculate_central_moments(input, output);

        *output.get_mut::<3, 0>() = input.get::<3, 0>()
            - cx * T::THREE.mul_add(output.get::<2, 0>(), cx * input.get::<1, 0>());

        let mu11_2 = output.get::<1, 1>() * T::TWO;

        *output.get_mut::<2, 1>() = input.get::<2, 1>()
            - cx * input.get::<0, 1>().mul_add(cx, mu11_2)
            - cy * output.get::<2, 0>();

        *output.get_mut::<1, 2>() = input.get::<1, 2>()
            - cy * input.get::<1, 0>().mul_add(cy, mu11_2)
            - cx * output.get::<0, 2>();

        *output.get_mut::<0, 3>() = input.get::<0, 3>()
            - cy * T::THREE.mul_add(output.get::<0, 2>(), cy * input.get::<0, 1>());
    }
}
