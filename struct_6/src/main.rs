fn main() {
    struct Color(i32, i32, i32);
    struct flag(Color);

    let india = flag(Color(0,0,0));
    println!("India's flag color is {:?}", (india.0).0);
}
