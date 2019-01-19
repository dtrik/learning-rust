use std::io;

fn main() {
    println!("Program to generate nth Fibonacci number");
    
    let mut n = String::new();
    
    println!("Enter n");
    io::stdin().read_line(&mut n)
        .expect("Failed to read");

    //IMP: n is defined as usize as it is used to index collections
    let n: usize = n.trim().parse()
        .expect("Unable to convert to int");

    println!("Printing the {}th Fibonacci number", n);
    
    //create new mutable vector to store fibonacci sequence
    //initialize first two elements 0,1
    let mut fib: Vec<u64> = Vec::new();
    fib.push(0);
    fib.push(1);

    //call fibonacci function by passing reference to vector
    if n > 2 {
        fibonacci(n, &mut fib);
    } 
    println!("{}th term in the Fibonacci sequence is: {}", n, fib[n-1]);
    
}

fn fibonacci(n: usize, fib: &mut Vec<u64>) {
    let mut index: usize = 2;
    while index < n {
        fib.push(fib[index-1] + fib[index-2]);
        index = index + 1;
    }
}
