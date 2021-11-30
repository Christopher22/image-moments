use std::iter::FromIterator;

use crate::{
    implementation::{Accumulator, SealedSupportedOrder, Storage},
    Index, Moments, Order, Point, Scalar, SupportedIndex, SupportedOrder,
};

/// The raw, spatial moments of an image or contour.
#[derive(Debug, Clone, PartialEq)]
pub struct Spatial<T: Scalar, const ORDER: usize>(
    pub(crate) <Order<ORDER> as SealedSupportedOrder<T>>::Storage,
)
where
    Order<ORDER>: SupportedOrder<T>;

impl<T: Scalar, const ORDER: usize> Moments<T, ORDER> for Spatial<T, ORDER>
where
    Order<ORDER>: SupportedOrder<T>,
{
    #[inline(always)]
    fn get<const I: usize, const J: usize>(&self) -> T
    where
        Index<I, J>: SupportedIndex<ORDER>,
    {
        self.0.get::<I, J>()
    }
}

impl<'a, T: Scalar, P: 'a + Point<T>, const ORDER: usize> FromIterator<&'a P> for Spatial<T, ORDER>
where
    Order<ORDER>: SupportedOrder<T>,
{
    fn from_iter<I: IntoIterator<Item = &'a P>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        let (mut acc, first_point) = match iterator.next() {
            Some(point) => {
                let acc = Accumulator::from_point(point);
                (acc, (*point).clone())
            }
            None => return Self(<Order<ORDER> as SealedSupportedOrder<T>>::Storage::zeros()),
        };

        // Feed in all the values and "wrap around" in the end
        for point in iterator {
            acc.update::<Order<ORDER>, _>(point);
        }
        acc.update::<Order<ORDER>, _>(&first_point);

        // Calculate the final moments
        Self(acc.finalize::<Order<ORDER>>())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{Moments, Spatial};

    #[test]
    fn test_empty() {
        let points: [(f64, f64); 0] = [];
        let moments: Spatial<f64, 3> = points.iter().collect();
        assert_abs_diff_eq!(moments.get::<0, 0>(), 0.0);
    }

    #[test]
    fn test_spatial_moments() {
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

        let moments: Spatial<f64, 3> = points.iter().collect();
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
