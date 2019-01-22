fn main() {
    let mut s = String::from("hello");  //mutable s comes into scope
    let r1 = &s;                        //r1 borrows s immutably
    println!("r1 is {}", r1);
    let r2 = &s;                        //r2 borrows s immutably
    println!("r2 is {}", r2);
    {
        let r3 = &mut s;                //r3 borrows s mutably, previous 
                                        //immutable borrow
                                        //can't be used after this
                                        
        r3.push_str(" world");
        println!("r3 is {}", r3);
    }                                   //r3 goes out of scope
    let r4 = &s;                        //new immutable reference is fine
    println!("r4 is {}", r4);

    //commenting out below line compiles, otherwise it doesn't
    //immutable borrow can't be used later if mutable happens 
    //println!("r1 is '{}', r2 is '{}', s is '{}'", r1, r2, s);
    println!("s is '{}'", s);
}
