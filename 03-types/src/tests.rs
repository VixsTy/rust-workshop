#![allow(dead_code)]

///!
///! UNCOMMENT THE CODE IN THE METHOD ANNOTED BY  #[test] AND REMOVE THE #[ignore] BEFORE TO RUN THE TEST THAT YOU TRY TO PASS.
///!

mod tuple_point_should {
    use super::*;

    //TODO: create a TuplePoint struct and add Debug trait to it

    #[test]
    #[ignore]
    fn be_created_as_tuple_of_i64() {
        //let point = TuplePoint(1i64, 2i64);
    }

    #[test]
    #[ignore]
    fn have_access_to_first_element_through_x_method() {
        //let point = TuplePoint(4, 8);
        //assert_eq!(4i64, point.x());
    }

    #[test]
    #[ignore]
    fn have_access_to_second_element_through_y_method() {
        //let point = TuplePoint(16, 32);
        //assert_eq!(32i64, point.y());
    }

    #[test]
    #[ignore]
    fn support_debug() {
        //assert_eq!(String::from("TuplePoint(64, 128)"), format!("{:?}", TuplePoint(64, 128)))
    }

    #[test]
    #[ignore]
    fn consider_equal_256_512_against_256_512() {
        //assert_eq!(TuplePoint(256, 512), TuplePoint(256, 512));
    }
}

mod named_point_should {
    use super::*;



    #[test]
    #[ignore]
    fn be_created_as_struct() {
        //let point = NamedPoint { x: 1i64, y: 2i64, name: String::from("standard init") };
    }

    #[test]
    #[ignore]
    fn be_created_through_constructor() {
        //let point = NamedPoint::new(4, 8, "constructor init");
        //assert_eq!(4, point.x);
        //assert_eq!(8, point.y);
        //assert_eq!(String::from("constructor init"), point.name);
    }

    #[test]
    #[ignore]
    fn be_created_through_copy_constructor() {
        //let base = NamedPoint { x: 16, y: 32, name: String::from("base") };
        //let point = NamedPoint::copy(base, "new name");
        //assert_eq!(16, point.x);
        //assert_eq!(32, point.y);
        //assert_eq!(String::from("new name"), point.name);
    }

    #[test]
    #[ignore]
    fn have_access_to_x_though_getter() {
        //let point = NamedPoint { x: 64, y: 128, name: String::from("get_x") };
        //assert_eq!(64, point.x());
    }

    #[test]
    #[ignore]
    fn have_access_to_second_element_through_y_method() {
        //let point = TuplePoint(16, 32);
        //assert_eq!(32i64, point.y());
    }

    #[test]
    #[ignore]
    fn support_debug() {
        //let point = NamedPoint { x: 64, y: 128, name: String::from("debug") };
        //assert_eq!(String::from("NamedPoint { x: 64, y: 128, name: \"debug\" }"), format!("{:?}", point));
    }
}
