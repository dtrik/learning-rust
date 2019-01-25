#![allow(unused_variables)]
fn main() {
    enum Message { //all variants of different type
        Quit,
        Move { x : i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message { //implement a method on enum
        fn call(&self) { //method can be used on enum created
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call(); //method being called on enum
}
