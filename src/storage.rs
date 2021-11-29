use crate::Scalar;

/// An efficient storage to access the required coefficients.
pub trait Storage<T: Scalar> {
    /// The "order" of the storage.
    const ORDER: usize;

    /// The required number of elements at the stack.
    const SIZE: usize;

    /// Create an instance of this storage with all values set to 0.
    fn zeros() -> Self;

    /// Access the element at a specific 0-based position.
    /// I + J must be <= Self::ORDER
    fn get<const I: usize, const J: usize>(&self) -> T;

    /// Access the element at a specific 0-based position mutably.
    /// I + J must be <= Self::ORDER
    fn get_mut<const I: usize, const J: usize>(&mut self) -> &mut T;
}

/// Calculate the space required to hold a storage of a specific order.
pub const fn calculate_space<const ORDER: usize>() -> usize {
    (ORDER + 1) * ((ORDER + 1) + 1) / 2
}

const fn calculate_index<const I: usize, const J: usize, const ORDER: usize>() -> usize {
    J * (ORDER + 1) - ((J as isize - 1) * J as isize / 2) as usize + I
}

/// Implement the Storage crate for some meaningful values.
macro_rules! impl_storage_for_order {
    ( $order:expr ) => {
        impl<T: Scalar> Storage<T> for [T; calculate_space::<$order>()] {
            const ORDER: usize = $order;
            const SIZE: usize = calculate_space::<$order>();

            #[inline(always)]
            fn zeros() -> Self {
                [T::ZERO; calculate_space::<$order>()]
            }

            #[inline(always)]
            fn get<const I: usize, const J: usize>(&self) -> T {
                self[calculate_index::<I, J, $order>()]
            }

            #[inline(always)]
            fn get_mut<const I: usize, const J: usize>(&mut self) -> &mut T {
                &mut self[calculate_index::<I, J, $order>()]
            }
        }
    };
}

impl_storage_for_order!(0);
impl_storage_for_order!(1);
impl_storage_for_order!(2);
impl_storage_for_order!(3);

#[cfg(test)]
mod tests {
    use crate::storage::{calculate_index, calculate_space, Storage};

    #[test]
    fn test_index() {
        // Check corner case
        assert_eq!(calculate_index::<0, 0, 1>(), 0);

        // Check even: Size == 4
        assert_eq!(calculate_index::<0, 0, 4>(), 0);
        assert_eq!(calculate_index::<0, 1, 4>(), 5);
        assert_eq!(calculate_index::<0, 2, 4>(), 9);
        assert_eq!(calculate_index::<2, 2, 4>(), 11);
        assert_eq!(calculate_index::<0, 3, 4>(), 12);
        assert_eq!(calculate_index::<0, 4, 4>(), 14);

        // Check uneven: Size == 5
        assert_eq!(calculate_index::<0, 0, 5>(), 0);
        assert_eq!(calculate_index::<0, 2, 5>(), 11);
        assert_eq!(calculate_index::<2, 2, 5>(), 13);
        assert_eq!(calculate_index::<0, 5, 5>(), 20);
    }

    #[test]
    fn test_space_calculation() {
        assert_eq!(calculate_space::<0>(), 1);
        assert_eq!(calculate_space::<1>(), 3);
        assert_eq!(calculate_space::<2>(), 6);
        assert_eq!(calculate_space::<3>(), 10);
        assert_eq!(calculate_space::<4>(), 15);
        assert_eq!(calculate_space::<5>(), 21);
    }

    #[test]
    fn test_access() {
        let mut data = [0.0; calculate_space::<0>()];
        assert_eq!(data.get::<0, 0>(), 0.0);
        *data.get_mut::<0, 0>() = 42.0;
        assert_eq!(data.get::<0, 0>(), 42.0);
    }
}
