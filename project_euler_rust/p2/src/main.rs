fn main() {
    println!("Program to find sum of even Fibonacci number < 4000000");
    
    let n = 4000000 ;
//    let n = 20 ;
    
    //create new mutable vector to store fibonacci sequence
    //initialize first two elements 0,1
    let mut fib_sum: u64 = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    while c < n {
        c = a + b;
        if c % 2 == 0 {
            fib_sum = fib_sum + c;
        }
        a = b;
        b = c;
    } 
    println!("Fibonacci sum of even terms is: {}", fib_sum);
}
