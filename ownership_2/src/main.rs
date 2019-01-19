fn main() {
    let s1 = gives_ownership();                 //s1 comes into scope because
                                                //gives_ownership returns its 
                                                //return value to s1

    println!("s1 {}", s1);

    let s2 = String::from("hello");             //s2 comes into scope
    println!("s2 {}", s2);

    let s3 = takes_and_gives_back(s2);          //s2 is moved into function
                                                //s3 comes into scope like s1
    println!("s3 {}", s3);
} //first s3 goes out of scope, then s2 goes out of scope but already moved 
//so nothing happens, then s1 goes out of scope

fn gives_ownership() -> String {                //creates a string and retursn
                                                //ownership also passed
    let some_string = String::from("hello");    //some_string comes into scope
    println!("Some string {}", some_string);
    some_string                                 //last statement returned
                                                //moves to calling function
}

fn takes_and_gives_back(a_string: String) -> String { //a_string comes 
                                                      //into scope
    println!("a_string {}", a_string);
    a_string                                    //last statement returned
                                                //a_string moves to caller
}
