#![allow(dead_code)]
use std::ops::RangeFrom;

fn collect_odd_and_square_them(vec: RangeFrom<usize>) -> Vec<usize> {
    // Help here: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // Collect odd numbers: filter
    // Only take five numbers: take
    // Square each number: map
    // Collect into <Vec<usize>>
    vec![]
}

#[test]
fn test_collect_odd_and_square_them() {
    let vector = 1..; // Infinite range of integers

    let res = collect_odd_and_square_them(vector);
    assert_eq!(vec![1, 9, 25, 49, 81], res)
}

fn get_words_that_contain_the_letter_i(sentence: &str) -> Vec<&str> {
    // Help here: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // split_whitespace() doc here: https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
    // filter with the contains() method: https://doc.rust-lang.org/std/primitive.str.html#method.contains
    // collect into Vec<&str>
    vec![]
}

#[test]
fn test_split_whitespace_filter_string_contains_letter_i() {
    let sentence = "This is a sentence in Rust.";
    assert_eq!(vec!["This", "is", "in"], get_words_that_contain_the_letter_i(sentence))
}
