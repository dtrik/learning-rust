fn main() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct flag(Color);

    let india = flag(Color(0,0,0));
    println!("India's flag has {:?}", india.0);
}
