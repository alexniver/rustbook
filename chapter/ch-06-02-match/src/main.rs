fn main() {
    let coin = Coin::Quater(UsState::Alabama);

    println!("a penny is {:?}", value_in_cents(coin));

    let mut num = Some(2);
    num = plus_one(num);
    let mut none = None;
    none = plus_one(none);

    {
        let dice_roll = 9;
        match dice_roll {
            3 => {
                println!("three")
            }
            7 => {
                println!("seven")
            }
            other => {
                println!("other num : {}", other)
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from : {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}
