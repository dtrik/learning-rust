fn main() {
    let s = String::from("hello");              //s comes into scope

    let len = calculate_length(&s);             //reference & to s is passed to
                                                //function, s remains in scope
                                                //function uses s without owner-
                                                //ship

    println!("Length of {} is {}", s, len);
}

fn calculate_length(s_ref: &String) -> usize {  //s_ref is a reference to s
                                                //comes into scope above
    s_ref.len()                                 //statement returned to main
} //s_ref goes out of scope here but it doesn't own s so nothing happens
                        

