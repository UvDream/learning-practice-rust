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

fn main() {
    let value1 = value_in_cents(Coin::Penny);
    println!("{}", value1);

    let value2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", value2);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?},{:?}", six, none);
    all_data(7);
    all_data(3);
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state=={:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x),
    }
}

fn all_data(x: u8) {
    match x {
        1 => println!("1"),
        3 => println!("2"),
        5 => println!("3"),
        _ => println!("_"),
    }
}
