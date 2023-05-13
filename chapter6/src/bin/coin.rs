#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("{:?}!", UsState);
            25
        }
    }
}

fn main() {
    let a = if true { "aa" } else { "bb" };
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)))
}
