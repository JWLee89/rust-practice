enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// .
fn get_ip_addr_kind() -> (IpAddrKind, IpAddrKind) {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    (four, six)
}

#[derive(Debug)] // To inspect state
pub (crate) enum UsState {
    Alabama,
    Alaska,
}


pub (crate) enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

pub (crate) fn plus_one(item: Option<i32>) -> Option<i32> {
    match item {
        None => None,
        Some(i) => Some(i + 1),
    }
}


pub (crate) fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Got penny!!!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alabama) => 25,
        Coin::Quarter(UsState::Alaska) => 30,
    }
}

