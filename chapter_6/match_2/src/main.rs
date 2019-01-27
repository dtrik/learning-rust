#![allow(unused_variables)]

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let ten = Some(10);
    let eleven = plus_one(ten);
    let none = plus_one(None);

    println!("{:?} plus one is {:?}", ten, eleven);
}
