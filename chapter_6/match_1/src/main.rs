#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    let coin_test = Coin::Penny;

    println!("Value of coin_test is {:?}",  value_in_cents(coin_test));
}
