fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
    println!("s is {}", s);
}

fn change(s_ref: &mut String) {
    s_ref.push_str(", world");
}
