use crate::{Order, Point, Scalar, Storage};

#[derive(Debug, Clone)]
pub struct Accumulator<T: Scalar, S: Storage<T>> {
    pub storage: S,
    pub last_point: (T, T),
    pub current_point: (T, T),
}

impl<T: Scalar, S: Storage<T>> Accumulator<T, S> {
    pub fn from_point<P: Point<T>>(point: &P) -> Self {
        Accumulator {
            storage: S::zeros(),
            last_point: (point.x(), point.y()),
            current_point: (T::TWO, T::TWO),
        }
    }

    pub fn update<O: SealedSupportedOrder<T>, P: Point<T>>(&mut self, point: &P) {
        self.current_point = (point.x(), point.y());
        O::update(self);
        self.last_point = self.current_point;
    }

    pub fn finalize<O: SealedSupportedOrder<T>>(mut self) -> S {
        let first_moment = self.storage.get::<0, 0>();
        // Check wether there would be divisions (almost) by 0
        if first_moment.abs() > T::EPSILON {
            let sign = T::ONE.copysign(first_moment);
            O::finalize(&mut self, sign);
        }
        self.storage
    }
}

pub trait SealedSupportedOrder<T: Scalar> {
    /// The underlying data storage.
    type Storage: Storage<T>;

    /// A intermediate result of the computation which is useful in subsequent computitions of higher orders.
    type IntermediateResult;

    /// Update the storage given the current state of the accumulator.
    /// Higher order functions must ensure the lower order is called.
    fn update<S: Storage<T>>(acc: &mut Accumulator<T, S>) -> Self::IntermediateResult;

    /// Finalize the associated values in the storage.
    /// Higher order functions must ensure the lower order is called.
    fn finalize<S: Storage<T>>(acc: &mut Accumulator<T, S>, sign: T);
}

impl<T: Scalar> SealedSupportedOrder<T> for Order<0> {
    type Storage = [T; crate::storage::calculate_space::<0>()];
    type IntermediateResult = T;

    fn update<S: Storage<T>>(acc: &mut Accumulator<T, S>) -> Self::IntermediateResult {
        let dxy = acc
            .last_point
            .0
            .mul_add(acc.current_point.1, acc.current_point.0 * -acc.last_point.1);
        *acc.storage.get_mut::<0, 0>() += dxy;
        dxy
    }

    fn finalize<S: Storage<T>>(acc: &mut Accumulator<T, S>, sign: T) {
        *acc.storage.get_mut::<0, 0>() *= T::F1_2.copysign(sign);
    }
}

impl<T: Scalar> SealedSupportedOrder<T> for Order<1> {
    type Storage = [T; crate::storage::calculate_space::<1>()];
    type IntermediateResult = (T, (T, T));

    fn update<S: Storage<T>>(acc: &mut Accumulator<T, S>) -> Self::IntermediateResult {
        let dxy = Order::<0>::update(acc);
        let last_plus_current = (
            acc.last_point.0 + acc.current_point.0,
            acc.last_point.1 + acc.current_point.1,
        );

        {
            let a10 = acc.storage.get_mut::<1, 0>();
            *a10 = dxy.mul_add(last_plus_current.0, *a10);
        }

        {
            let a01 = acc.storage.get_mut::<0, 1>();
            *a01 = dxy.mul_add(last_plus_current.1, *a01);
        }

        (dxy, last_plus_current)
    }

    fn finalize<S: Storage<T>>(acc: &mut Accumulator<T, S>, sign: T) {
        Order::<0>::finalize(acc, sign);
        let factor = T::F1_6.copysign(sign);
        *acc.storage.get_mut::<1, 0>() *= factor;
        *acc.storage.get_mut::<0, 1>() *= factor;
    }
}

impl<T: Scalar> SealedSupportedOrder<T> for Order<2> {
    type Storage = [T; crate::storage::calculate_space::<2>()];
    type IntermediateResult = (T, (T, T), (T, T));

    fn update<S: Storage<T>>(acc: &mut Accumulator<T, S>) -> Self::IntermediateResult {
        let (dxy, last_plus_current) = Order::<1>::update(acc);
        let current_pow = (acc.current_point.0.powi(2), acc.current_point.1.powi(2));

        {
            let a20 = acc.storage.get_mut::<2, 0>();
            *a20 = dxy.mul_add(
                acc.last_point.0.mul_add(last_plus_current.0, current_pow.0),
                *a20,
            );
        }

        {
            let a11 = acc.storage.get_mut::<1, 1>();
            *a11 = dxy.mul_add(
                acc.last_point.0.mul_add(
                    last_plus_current.1 + acc.last_point.1,
                    acc.current_point.0 * (last_plus_current.1 + acc.current_point.1),
                ),
                *a11,
            );
        }

        {
            let a02 = acc.storage.get_mut::<0, 2>();
            *a02 = dxy.mul_add(
                acc.last_point.1.mul_add(last_plus_current.1, current_pow.1),
                *a02,
            );
        }

        (dxy, last_plus_current, current_pow)
    }

    fn finalize<S: Storage<T>>(acc: &mut Accumulator<T, S>, sign: T) {
        Order::<1>::finalize(acc, sign);
        let f1_12 = T::F1_12.copysign(sign);
        *acc.storage.get_mut::<2, 0>() *= f1_12;
        *acc.storage.get_mut::<0, 2>() *= f1_12;
        *acc.storage.get_mut::<1, 1>() *= T::F1_24.copysign(sign);
    }
}

impl<T: Scalar> SealedSupportedOrder<T> for Order<3> {
    type Storage = [T; crate::storage::calculate_space::<3>()];
    type IntermediateResult = ();

    fn update<S: Storage<T>>(acc: &mut Accumulator<T, S>) -> Self::IntermediateResult {
        let (dxy, last_plus_current, current_pow) = Order::<2>::update(acc);
        let last_pow = (acc.last_point.0.powi(2), acc.last_point.1.powi(2));

        {
            let a30 = acc.storage.get_mut::<3, 0>();
            *a30 = dxy.mul_add(last_plus_current.0 * (last_pow.0 + current_pow.0), *a30);
        }

        {
            let a03 = acc.storage.get_mut::<0, 3>();
            *a03 = dxy.mul_add(last_plus_current.1 * (last_pow.1 + current_pow.1), *a03);
        }

        {
            let a21 = acc.storage.get_mut::<2, 1>();
            *a21 = dxy.mul_add(
                last_pow.0 * T::THREE.mul_add(acc.last_point.1, acc.current_point.1)
                    + T::TWO * acc.current_point.0 * acc.last_point.0 * last_plus_current.1
                    + current_pow.0 * T::THREE.mul_add(acc.current_point.1, acc.last_point.1),
                *a21,
            );
        }

        {
            let a12 = acc.storage.get_mut::<1, 2>();
            *a12 = dxy.mul_add(
                last_pow.1 * T::THREE.mul_add(acc.last_point.0, acc.current_point.0)
                    + T::TWO * acc.current_point.1 * acc.last_point.1 * last_plus_current.0
                    + current_pow.1 * T::THREE.mul_add(acc.current_point.0, acc.last_point.0),
                *a12,
            );
        }
    }

    fn finalize<S: Storage<T>>(acc: &mut Accumulator<T, S>, sign: T) {
        Order::<2>::finalize(acc, sign);
        let f1_20 = T::F1_20.copysign(sign);
        let f1_60 = T::F1_60.copysign(sign);
        *acc.storage.get_mut::<3, 0>() *= f1_20;
        *acc.storage.get_mut::<2, 1>() *= f1_60;
        *acc.storage.get_mut::<1, 2>() *= f1_60;
        *acc.storage.get_mut::<0, 3>() *= f1_20;
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{
        accumulator::{Accumulator, SealedSupportedOrder},
        Order, Storage,
    };

    #[test]
    fn test_accumulator() {
        let points = [
            (53.0, 19.0),
            (52.0, 20.0),
            (49.0, 20.0),
            (48.0, 21.0),
            (47.0, 21.0),
            (46.0, 22.0),
            (45.0, 22.0),
            (44.0, 23.0),
            (43.0, 23.0),
            (42.0, 24.0),
            (41.0, 24.0),
            (39.0, 26.0),
            (38.0, 26.0),
            (34.0, 30.0),
            (76.0, 30.0),
            (77.0, 31.0),
            (79.0, 31.0),
            (80.0, 32.0),
            (81.0, 32.0),
            (83.0, 34.0),
            (84.0, 34.0),
            (86.0, 36.0),
            (86.0, 37.0),
            (87.0, 38.0),
            (87.0, 39.0),
            (88.0, 40.0),
            (88.0, 47.0),
            (86.0, 49.0),
            (86.0, 50.0),
            (83.0, 53.0),
            (82.0, 53.0),
            (81.0, 54.0),
            (81.0, 55.0),
            (82.0, 55.0),
            (84.0, 57.0),
            (84.0, 58.0),
            (85.0, 59.0),
            (85.0, 60.0),
            (86.0, 61.0),
            (86.0, 63.0),
            (87.0, 64.0),
            (87.0, 65.0),
            (88.0, 66.0),
            (93.0, 66.0),
            (94.0, 65.0),
            (94.0, 64.0),
            (95.0, 63.0),
            (95.0, 60.0),
            (96.0, 59.0),
            (99.0, 59.0),
            (99.0, 53.0),
            (98.0, 52.0),
            (97.0, 52.0),
            (96.0, 51.0),
            (95.0, 51.0),
            (94.0, 50.0),
            (93.0, 50.0),
            (90.0, 47.0),
            (90.0, 46.0),
            (91.0, 45.0),
            (91.0, 44.0),
            (92.0, 43.0),
            (92.0, 42.0),
            (93.0, 41.0),
            (93.0, 39.0),
            (94.0, 38.0),
            (94.0, 36.0),
            (91.0, 33.0),
            (91.0, 32.0),
            (85.0, 26.0),
            (84.0, 26.0),
            (82.0, 24.0),
            (81.0, 24.0),
            (80.0, 23.0),
            (79.0, 23.0),
            (78.0, 22.0),
            (77.0, 22.0),
            (76.0, 21.0),
            (75.0, 21.0),
            (74.0, 20.0),
            (71.0, 20.0),
            (70.0, 19.0),
            (69.0, 19.0),
            (67.0, 21.0),
            (66.0, 21.0),
            (62.0, 25.0),
            (60.0, 25.0),
            (54.0, 19.0),
        ];
        let moments = {
            let mut acc =
                Accumulator::<f64, <Order<3> as SealedSupportedOrder<f64>>::Storage>::from_point(
                    points.last().expect("last point"),
                );

            for point in points.iter() {
                acc.update::<Order<3>, _>(point);
            }
            acc.finalize::<Order<3>>()
        };

        assert_abs_diff_eq!(moments.get::<0, 0>(), 703.0);
        assert_abs_diff_eq!(moments.get::<1, 0>(), 52175.166666666664);
        assert_abs_diff_eq!(moments.get::<0, 1>(), 25661.5);
        assert_abs_diff_eq!(moments.get::<2, 0>(), 4084450.6666666665);
        assert_abs_diff_eq!(moments.get::<1, 1>(), 2024477.75);
        assert_abs_diff_eq!(moments.get::<0, 2>(), 1071256.0);
        assert_abs_diff_eq!(moments.get::<3, 0>(), 332589780.65000004);
        assert_abs_diff_eq!(moments.get::<2, 1>(), 166738807.83333334);
        assert_abs_diff_eq!(moments.get::<1, 2>(), 89124447.63333333);
        assert_abs_diff_eq!(moments.get::<0, 3>(), 50269189.75);
    }
}
