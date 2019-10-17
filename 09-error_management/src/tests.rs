#![allow(dead_code)]


use std::num::ParseIntError;

// NOTE: a `?` operator that does pretty much what you would make that match statement do for you! But you should use a match first.
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>(); //FIXME: match the result of parse()

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}

enum InputError {
    Invalid,
}

fn double_result() {
    unimplemented!();
}

//TODO: uncomment all the asserts
mod double_result_should {
    use super::*;

    #[test]
    fn return_err_zero_when_ok_0() {
        //assert_eq!(Err(DoubleError::Zero), double_result(Ok(0)));
    }
    
    #[test]
    fn return_ok_2_when_ok_1() {
        //assert_eq!(Ok(2), double_result(Ok(1)));
    }
    
    #[test]
    fn return_err_invalidinput_when_err() {
        //assert_eq!(Err(DoubleError::InvalidInput), double_result(Err(InputError::Invalid)));
    }
}

fn checked_division(a: i32, b: i32) {
    unimplemented!();
}

mod checked_division_should {
    use super::*;

    #[test]
    fn return_some_2_when_4_and_2() {
        //assert_eq!(Some(2), checked_division(4, 2));
    }

    #[test]
    fn return_none_when_divising_by_0() {
        //assert_eq!(None, checked_division(42, 0));
    }
}

fn open_box_with() {
    unimplemented!();
}

mod open_box_with_should {
    use super::*;

    #[test]
    fn return_kind_message_when_some_value() {
        //assert_eq!("Oh ! I like banana !", &open_box_with(Some("banana")));
    }

    #[test]
    fn return_disappointed_message_when_none() {
       //assert_eq!("Oh... I'm so sad...", &open_box_with(None));
    }
}