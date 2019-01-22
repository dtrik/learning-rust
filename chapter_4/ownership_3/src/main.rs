fn main() {
    let s1 = String::from("hello");                 //s1 comes into scope

    let (s2, len) = calculate_length(s1);           //s1 moved to function
                                                    //s2 and len comes to scope

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
