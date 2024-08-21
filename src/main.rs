enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),

}

enum  Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ColorRGB(u8, u8, u8)
}

impl Message {
    fn some_function(&self) {
        println!("Ok Blud.");
    }
}

fn main() {
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0 ,0, 1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("121.12132.sd23e3r.3r32r"));
    let user_move: Message = Message::Move { x: 2, y: 4 };
    user_move.some_function();

    let some_number: Option<i32> = Some(5);
    let some_char:Option<char> = Some('e');

    let absent_number: Option<i32>= None;

    let sum_numbers: i32 = some_number.unwrap_or(0) + absent_number.unwrap_or(0);

    println!("The Sum is {}", sum_numbers)

}

