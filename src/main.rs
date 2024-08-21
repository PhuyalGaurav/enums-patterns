fn main(){
    let manish_money: Coin = Coin::Dime;
    let himal_money: Coin = Coin::Penny;

    if(value_in_cents(manish_money) > value_in_cents(himal_money)){
        println!("Manish is Richer than himal");
    }
    else {
        println!("Himal is richer than Manish");
    }
}

enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

