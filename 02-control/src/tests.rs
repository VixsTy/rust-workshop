#![allow(dead_code)]

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - return
    // - another function call
    // - additional variables
    // Scroll down for hints.
    unimplemented!()
}

#[cfg(test)]
mod basic {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}

/// Computes product of numbers between `from` and `to` (both included !)
/// 
/// For example: `product_range(4, 6) = 4 x 5 x 6 = 120`
/// 
fn product_range(from: u32, to: u32) -> u32 {
    0
}

mod product_range_should {

    use super::product_range;

    #[test]
    fn return_2_when_from_2_to_2() {
        assert_eq!(2u32, product_range(2, 2));
    }

    #[test]
    fn return_120_when_from_4_to_6() {
        assert_eq!(120u32, product_range(4, 6));
    }

}

/// Returns last number before `0` and returns `0` if none.
/// 
/// For example: `last_non_zero([5, 6, 0, 1, 3]) = 6`
/// 
fn last_non_zero(numbers: Vec<u32>) -> u32 {
    0
}

mod last_non_zero_should {

    use super::last_non_zero;

    #[test]
    fn return_0_when_empty() {
        assert_eq!(0u32, last_non_zero(vec![]));
    }

    #[test]
    fn return_6_when_5_6_0_1_3() {
        assert_eq!(6u32, last_non_zero(vec![5, 6, 0, 1, 3]));
    }

}
