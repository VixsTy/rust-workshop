#![allow(dead_code)]

#[derive(Debug)]
struct Foo {
    bar: String,
    tick: i32,
}

fn get_false() -> bool {
    unimplemented!();
}

fn get_42_as_i32() -> i32 {
    unimplemented!();
}

fn get_unit() -> () {
    unimplemented!();
}


fn get_formated_string(debug_string: &str) -> String {
    let foo = Foo {bar: "bar".to_string(), tick: 32};
    unimplemented!();
    //TODO: use the macro format! https://doc.rust-lang.org/std/fmt/fn.format.html
}

#[test]
#[ignore]
fn get_false_should_return_false_as_bool() {
    let result: bool = get_false();
    assert_eq!(false, result);
}

#[test]
#[ignore]
fn get_42i32_should_return_42_as_i32() {
    let result: i32 = get_42_as_i32();
    assert_eq!(42, result);
}


#[test]
#[ignore]
fn get_unit_should_return_unit() {
    let result: () = get_unit();
    assert_eq!((), result);
}

#[test]
fn get_a_formated_string() {
    assert_eq!(String::from("some context Foo { bar: \"bar\", tick: 32 }"), result);
}
