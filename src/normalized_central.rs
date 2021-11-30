use crate::{
    implementation::{NormalizedCentralMoments, SealedSupportedOrder, Storage},
    Central, Index, Moments, Order, Scalar, SupportedIndex, SupportedOrder,
};

/// The normalized central moments of an image or contour which are translational and scale invariant.
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedCentral<T: Scalar, const ORDER: usize>(
    pub(crate) <Order<ORDER> as SealedSupportedOrder<T>>::Storage,
)
where
    Order<ORDER>: SupportedOrder<T>;

impl<'a, T: Scalar, const ORDER: usize> From<&'a Central<T, ORDER>> for NormalizedCentral<T, ORDER>
where
    Order<ORDER>: SupportedOrder<T>,
{
    fn from(central_moments: &'a Central<T, ORDER>) -> Self {
        let mut normalized_central_moments =
            <Order<ORDER> as SealedSupportedOrder<T>>::Storage::zeros();
        Order::<ORDER>::calculate_normalized_central_moments(
            &central_moments.0,
            &mut normalized_central_moments,
        );
        Self(normalized_central_moments)
    }
}

impl<T: Scalar, const ORDER: usize> Moments<T, ORDER> for NormalizedCentral<T, ORDER>
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

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{central::Central, Moments, NormalizedCentral, Spatial};

    #[test]
    fn test_normalized_central_moments() {
        let points = [
            (53, 19),
            (52, 20),
            (49, 20),
            (48, 21),
            (47, 21),
            (46, 22),
            (45, 22),
            (44, 23),
            (43, 23),
            (42, 24),
            (41, 24),
            (39, 26),
            (38, 26),
            (34, 30),
            (76, 30),
            (77, 31),
            (79, 31),
            (80, 32),
            (81, 32),
            (83, 34),
            (84, 34),
            (86, 36),
            (86, 37),
            (87, 38),
            (87, 39),
            (88, 40),
            (88, 47),
            (86, 49),
            (86, 50),
            (83, 53),
            (82, 53),
            (81, 54),
            (81, 55),
            (82, 55),
            (84, 57),
            (84, 58),
            (85, 59),
            (85, 60),
            (86, 61),
            (86, 63),
            (87, 64),
            (87, 65),
            (88, 66),
            (93, 66),
            (94, 65),
            (94, 64),
            (95, 63),
            (95, 60),
            (96, 59),
            (99, 59),
            (99, 53),
            (98, 52),
            (97, 52),
            (96, 51),
            (95, 51),
            (94, 50),
            (93, 50),
            (90, 47),
            (90, 46),
            (91, 45),
            (91, 44),
            (92, 43),
            (92, 42),
            (93, 41),
            (93, 39),
            (94, 38),
            (94, 36),
            (91, 33),
            (91, 32),
            (85, 26),
            (84, 26),
            (82, 24),
            (81, 24),
            (80, 23),
            (79, 23),
            (78, 22),
            (77, 22),
            (76, 21),
            (75, 21),
            (74, 20),
            (71, 20),
            (70, 19),
            (69, 19),
            (67, 21),
            (66, 21),
            (62, 25),
            (60, 25),
            (54, 19),
        ];

        let normalized_central_moments = {
            let moments: Spatial<f64, 3> = points.iter().collect();
            NormalizedCentral::from(&Central::from(&moments))
        };

        assert_abs_diff_eq!(normalized_central_moments.get::<0, 0>(), 1.0);
        assert_abs_diff_eq!(normalized_central_moments.get::<1, 0>(), 0.0);
        assert_abs_diff_eq!(normalized_central_moments.get::<0, 1>(), 0.0);

        assert_abs_diff_eq!(
            normalized_central_moments.get::<2, 0>(),
            0.4292123953519341,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<1, 1>(),
            0.24268220715350328,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<0, 2>(),
            0.27222945011138916,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<3, 0>(),
            -0.15535939037876878,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<2, 1>(),
            -0.012058728594976597,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<1, 2>(),
            0.06579235651780115,
            epsilon = 10e-7
        );
        assert_abs_diff_eq!(
            normalized_central_moments.get::<0, 3>(),
            0.10250983666512753,
            epsilon = 10e-7
        );
    }
}
