use std::fmt::Debug;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegisterFieldMask<T>(T) where T: Copy + Clone + Debug + PartialEq;

pub trait RegisterFieldMaskProperties {
    type MaskType: Copy;

    fn value(self) -> Self::MaskType;
    fn most_significant_bit(self) -> Option<usize>;
    fn least_significant_bit(self) -> Option<usize>;
    fn width_bits(self) -> usize;
}

macro_rules! impl_traits_for {
    ($($types:ident),+) => {
        $(
            impl std::convert::From<$types> for RegisterFieldMask<$types> {
                fn from(value: $types) -> RegisterFieldMask<$types> {
                    RegisterFieldMask(value)
                }
            }

            impl RegisterFieldMaskProperties for RegisterFieldMask<$types> {
                type MaskType = $types;

                fn value(self) -> Self::MaskType { self.0 }

                fn most_significant_bit(self) -> Option<usize> {
                    let index = ($types::BITS - self.0.leading_zeros()) as isize - 1;
                    if index >= 0 { Some(index as usize) } else { None }
                }

                fn least_significant_bit(self) -> Option<usize> {
                    let index = self.0.trailing_zeros() as usize;
                    if index < $types::BITS as usize { Some(index) } else { None }
                }

                fn width_bits(self) -> usize {
                    match (self.most_significant_bit(), self.least_significant_bit()) {
                        (Some(msb), Some(lsb)) => msb - lsb + 1,
                        _ => 0
                    }
                }
            }
        )+
    }
}

impl_traits_for![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::*;

    use super::*;

    #[test]
    fn value__called__expect_same_value_passed_to_from() {
        value__called__expect_same_value_passed_to_from_for(any_i8());
        value__called__expect_same_value_passed_to_from_for(any_u8());
        value__called__expect_same_value_passed_to_from_for(any_i16());
        value__called__expect_same_value_passed_to_from_for(any_u16());
        value__called__expect_same_value_passed_to_from_for(any_i32());
        value__called__expect_same_value_passed_to_from_for(any_u32());
        value__called__expect_same_value_passed_to_from_for(any_i64());
        value__called__expect_same_value_passed_to_from_for(any_u64());
        value__called__expect_same_value_passed_to_from_for(any_isize());
        value__called__expect_same_value_passed_to_from_for(any_usize());
    }

    fn value__called__expect_same_value_passed_to_from_for<T>(value: T)
        where
            T: Copy + Clone + Debug + PartialEq + Into<RegisterFieldMask<T>>,
            RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T> {

        let mask: RegisterFieldMask<_> = value.into();
        expect!(mask.value()).to_equal(value);
    }

    #[test]
    fn most_significant_bit__called_for_zero_mask__expect_none() {
        let mask = RegisterFieldMask::from(0_u32);
        expect!(mask.most_significant_bit()).to_be_none();
    }

    #[test]
    fn most_significant_bit__called_for_nonzero_mask__expect_index_of_first_one() {
        let value = any_isize_except(0);
        let mask = RegisterFieldMask::from(value);
        expect!(mask.most_significant_bit()).to_equal(Some((isize::BITS - value.leading_zeros() - 1) as usize));
    }

    #[test]
    fn least_significant_bit__called_for_zero_mask__expect_none() {
        let mask = RegisterFieldMask::from(0_i64);
        expect!(mask.least_significant_bit()).to_be_none();
    }

    #[test]
    fn least_significant_bit__called_for_nonzero_mask__expect_number_of_trailing_zeroes() {
        let value = any_usize_except(0);
        let mask = RegisterFieldMask::from(value);
        expect!(mask.least_significant_bit()).to_equal(Some(value.trailing_zeros() as usize));
    }

    #[test]
    fn width_bits__called_for_zero_mask__expect_zero() {
        let mask = RegisterFieldMask::from(0);
        expect!(mask.width_bits()).to_equal(0);
    }

    #[test]
    fn width_bits__called_for_all_ones_mask__expect_number_of_bits() {
        let mask = RegisterFieldMask::from(!0_i8);
        expect!(mask.width_bits()).to_equal(8);

        let mask = RegisterFieldMask::from(!0_u32);
        expect!(mask.width_bits()).to_equal(32);

        let mask = RegisterFieldMask::from(!0_u64);
        expect!(mask.width_bits()).to_equal(64);
    }

    #[test]
    fn width_bits__called_for_nonzero_mask__expect_difference_between_msb_and_lsb_adjusted_for_zero_indexing() {
        let mask = RegisterFieldMask::from(any_isize_except(0));
        expect!(mask.width_bits()).to_equal(mask.most_significant_bit().unwrap() - mask.least_significant_bit().unwrap() + 1);
    }
}
