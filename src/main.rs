fn main(){
    let manish_money: Coin = Coin::Dime;
    let himal_money: Coin = Coin::Penny;

    if value_in_cents(&manish_money) > value_in_cents(&himal_money){
        println!("Manish is Richer than Himal");
    }
    else {
        println!("Himal is richer than Manish");
    }

    let mut gaurav_money: Coin = Coin::Quarter(Country::Nepal(String::from("Kathmandu")));
    let gaurav_money_cents: u8 = value_in_cents(&gaurav_money);
    println!("Gaurav has {gaurav_money_cents:?}. in cents");
    gaurav_money = convert_to_dollar(Some(gaurav_money));

    let shuva_money: Option<Coin> = None;
    println!("Shuva has {shuva_money:?} cents.");
    let shuva_money: Coin = convert_to_dollar(shuva_money);
    println!("After Gaurav's generous donation shuva now has {shuva_money:?}");

    check_poor(&shuva_money);
    check_poor(&gaurav_money);
}

#[derive(Debug)]
enum Country {
    Nepal(String),
    India
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter(Country),
    Dollar
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(Country) => {
            println!("The quater is from {Country:?}!");
            25
        },
        Coin::Dollar => 100
    }
}

fn convert_to_dollar (x: Option<Coin>) -> Coin {
    match x {
        None => {
            println!("Nothing? here have a penny");
            Coin::Penny
        },
        Some(x) => {
            println!("The {x:?} is now a {:?}", Coin::Dollar);
            return Coin::Dollar;
        }
    }
}

fn check_poor(money: &Coin) {
    let money_in_cents: u8 = value_in_cents(money);
    match money_in_cents {
        0..=1 => {
            println!("Poor Bastard :'( ");
        },
        2..=20 => {
            println!("You are fairly rich")
        },
        _ => {
            println!("damn you rich bastard. >:)")
        }
    }
}