#![allow(dead_code)]

    use super::*;

    // This method only accept [i32] -> i32 and we want to be able to accept char and i32
    // Transform this method by using a generic type T.
    // Add a check on generic to verify that the types of the values in the slice that we pass into the function must implement the PartialOrd and Copy
    fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


    #[test]
    fn largest_for_every_one() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        assert_eq!(100, result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        assert_eq!('y', result);
    }

/*
#[test]
fn support_debug_for_u64() {
    let p: Pair<u64, u64> = Pair(42, 128);
    assert_eq!("Pair(42, 128)", &format!("{:?}", p));
}

#[test]
fn add_each_components_on_add() {
    let p1 =   Pair(1,  2);
    let p2 =   Pair(4,  8);
    assert_eq!(Pair(5, 10), p1 + p2);
}

#[test]
fn support_convert_into_string() {
    let p = Pair(8, String::from("16"));
    let string: String = <Pair<_,_> as Convert<_>>::convert(&p);
    assert_eq!("{8;\"16\"}", &string);
}

fn compare_tuple<A: PartialEq + Debug,B: PartialEq + Debug>(actual: (A,B), expected0: A, expected1: B) {
    assert_eq!(expected0, actual.0, "index: 0");
    assert_eq!(expected1, actual.1, "index: 0");
}
#[test]
fn support_convert_into_tuple() {
    let p = Pair(String::from("foobar"), 42);
    compare_tuple(p.convert(), String::from("foobar"), 42);
}*/
