#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    enum UsState {
        Alaska,
        Alabama,
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Minted for {:?}", state);
                25
            },
        }
    }
    let coin_penny = Coin::Penny;
    let coin_quarter_alaska = Coin::Quarter(UsState::Alaska);

    println!("Value of coin_test is {:?}",  value_in_cents(coin_penny));
    println!("Coin is worth {:?} cents", value_in_cents(coin_quarter_alaska));
}
