fn main() {
    //program to start understanding ownwership

    let s = String::from("hello");          //s comes into scope
    
    takes_ownership(s);                     //s used in a function
                                            //s not valid anymore
    println!("String s goes out of scope"); //compiler should error out

    let x = 5;                              //x comes into scope

    makes_copy(x);                          //x used in function
                                            //x is i32 so copy instead of
                                            //move => x can be used
    println!("x {}", x);                    //testing x

} //first x goes out of scope and then s but s already moved so nothing happens

fn takes_ownership(some_string: String) {   //some_string comes into scope
    println!("Some string s from main {}", some_string);
} //some_string goes out of scope and drop is called, memory is freed

fn makes_copy(some_integer: i32) {          //some_int comes into scope
    println!("Some integer {} from main", some_integer);
} //some_integer goes out of scope

