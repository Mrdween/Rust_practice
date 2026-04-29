#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Напиши функцию, которая:
// - для Penny печатает "Копейка"
// - для Nickel печатает "5 центов"
// - для Dime печатает "10 центов"
// - для Quarter печатает "" и возвращает 25 
fn process_coin(coin: Coin) -> u32 {
    // TODO: match с печатью
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter из шатата {:?}", state);
            25
        }
    }
}

fn main() {
    let q = Coin::Quarter(UsState::California);
    let value = process_coin(q);
    println!("Ценность: {}", value);
}
