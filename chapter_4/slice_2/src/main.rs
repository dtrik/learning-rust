fn main() {
    let s = String::from("hello world");
    let hello_1 = &s[..5];
    let world_1 = &s[6..];
    let hello_2 = &s[..=4];
    let world_2 = &s[6..=10];
    println!("{} + {}", hello_1, world_1);
    println!("Also {} + {}", hello_2, world_2);
}
