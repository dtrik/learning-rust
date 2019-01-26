fn main() {
//    let mut num: u64 = 13195;
    let mut num: u64 = 600851475143;
    let mut factor = 2;
    let mut last_factor = 1;
    let end = (num as f64).sqrt() as u64;
    while num > 1 || factor <= end {
//        println!("Current last_factor {}", last_factor);
        if num % factor == 0 {
            last_factor = factor;
            num = num/factor;
            while num % factor == 0 {
                num = num/factor;
            }
        }
        factor = factor + 1;
    }
    println!("{}", last_factor);
}
