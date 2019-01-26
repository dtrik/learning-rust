use std::io;

fn main() {
    
    let mut n = String::new();
    println!("Enter upper limit:");
    io::stdin().read_line(&mut n)
        .expect("Failed to read");
    let n: u32 = n.trim().parse()
        .expect("Unable to convert to unsigned int");
    println!("Calculating sum of multiples of 3 and 5 till {}", n);
    
    let mut to_check: u32 = 6;
    let mut sum_n: u32 = 8;
    while to_check < n{
        if to_check % 3 == 0 || to_check % 5 == 0 {
            sum_n = sum_n + to_check;
        }
        to_check = to_check + 1;
    }
    println!("{}", sum_n);
}
