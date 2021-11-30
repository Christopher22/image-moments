use self::private::Sealed;

/// An index specified at compile time.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Index<const I: usize, const J: usize>;

/// An marker trait indicating a compile-time index is valid for a specific order. This trait is sealed.
pub trait SupportedIndex<const ORDER: usize>: Sealed {}

macro_rules! impl_supported_index {
    ($( $order:expr ),* => ($i:expr, $j:expr)) => {
        impl Sealed for Index<$i, $j> {}
        $(
            impl SupportedIndex<$order> for Index<$i, $j> {}
        )*
    };
    ($( $order:expr ),* => ($i:expr) <-> ($j:expr)) => {
        impl Sealed for Index<$i, $j> {}
        impl Sealed for Index<$j, $i> {}
        $(
            impl SupportedIndex<$order> for Index<$i, $j> {}
            impl SupportedIndex<$order> for Index<$j, $i> {}
        )*
    };
}

impl_supported_index!(0, 1, 2, 3 => (0, 0));
impl_supported_index!(1, 2, 3 => (1) <-> (0));
impl_supported_index!(2, 3 => (1, 1));
impl_supported_index!(2, 3 => (2) <-> (0));
impl_supported_index!(3 => (3) <-> (0));
impl_supported_index!(3 => (2) <-> (1));

mod private {
    pub trait Sealed {}
}
