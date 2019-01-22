fn main() {
    //below call to dangle will compiler error
    //let reference_to_nothing = dangle();
    let reference_to_string = no_dangle();
    println!("{}", reference_to_string);
}

//below function compile errors out
//fn dangle() -> &String {
//    let s = String::from("hello");          //s comes into scope
//    &s                                      //reference to s is returned
//}//s goes out of scope but reference to it is returned which results in a 
//pointer to a freed memory. ERROR: this is a dangling reference

fn no_dangle() -> String {
    let s_no_dangle = String::from("hello");//s_no_dangle comes into scope
    s_no_dangle                             //actual string is returned
}//string is returned so no issue 
