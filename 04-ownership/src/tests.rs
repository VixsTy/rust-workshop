//tips: just read the compilator error
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

#[test]
fn test_easy_fill_vec() {
    let vec0 = Vec::new();

    let vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//tips: just read the compilator error
fn fill_vec_2(vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

#[test]
fn test_easy_fill_vec_2() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec_2(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        // That'll help you to understand ownership
        println!("Dropping a Foobar: {:?}", self);
    }
}

fn uses_foobar(foobar: Foobar) { //TODO: fix my header/signature
    println!("I consumed a Foobar: {:?}", foobar);
}

#[test]
fn test_fix_consuming_ownership() {
    let x = Foobar(1);
    uses_foobar(x);
    uses_foobar(x);
}

//DONT TOUCH ME
fn uses_foobar2(foobar: Foobar) {
    println!("I consumed a Foobar: {:?}", foobar);
}

#[test]
fn test_fix_by_implementing_clone() {
    //TODO: Add the auto implementation of the trait: Clone on the Foobar structure.
    //TODO: use the clone() method: https://doc.rust-lang.org/std/clone/trait.Clone.html#required-methods
    let x = Foobar(1);
    uses_foobar(x);
    uses_foobar(x);
}

impl Foobar {
    fn use_it(self) { //TODO: fix my header
        println!("I consumed a Foobar: {:?}", self);
    }
}

#[test]
fn test_fix_trait_ownership() {
    let x = Foobar(1);
    x.use_it();
    x.use_it();
}

fn concat(subject: &String) {
    subject.push_str("You're great!");
}

#[test]
fn test_fix_mut_in_concat() {
    let s = "yolo".to_string();
    concat(s);
}