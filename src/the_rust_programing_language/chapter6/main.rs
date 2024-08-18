use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    //enumが有用でこの場合、構造体よりも適切である理由を確認しましょう。
    // IPアドレスを扱う必要が出たとしましょう。現在、IPアドレスの規格は二つあります:
    // バージョン4とバージョン6です。 これらは、プログラムが遭遇するIPアドレスのすべての可能性です:
    // 列挙型は、取りうる値をすべて列挙でき、 これが列挙型の名前の由来です。

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_type: IpAddrKind) {}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_string = Some("String");
    // let absent_number: Option<i32> = None;

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => todo!(),
            Coin::Nickel => todo!(),
            Coin::Dime => todo!(),
            Coin::Quarter => todo!(),
        }
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ を記述することで、上のどの場合にも当てはまらない場合の処理が書ける
    }

    // case 1
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // case 2
    if let Some(3) = some_u8_value {
        println!("three")
    }

    // case1, case2 は同値である

    // case 1
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // case 2
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // case1, case2 は同値である
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体

// Option で null を表現することも可能
enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
