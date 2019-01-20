fn main() {
    let mut s = String::from("hello");  //s comes into scope
    {
        let r1 = &mut s;                //r1 comes into scope
        r1.push_str(" world");
        println!("r1 is {}", r1);
    }                                   //r1 goes out of scope
    let r2 = &mut s;                    //r2 comes into scope
    r2.push_str(" is good");    
    println!("r2 is '{}'", r2);
    println!("s is '{}'", s);
} //r2 and s go out of scope
