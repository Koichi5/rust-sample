use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user1の値をそのまま流用できる
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // 構造体から特定の値を得るには、ドット記法が使えます。このユーザのEメールアドレスだけが欲しいなら、
    // この値を使いたかった場所全部でuser1.emailが使えます。インスタンスが可変であれば、
    // ドット記法を使い特定のフィールドに代入することで値を変更できます。リスト5-3では、
    // 可変なUserインスタンスのemailフィールド値を変更する方法を示しています。
    // Rustでは、一部のフィールドのみを可変にすることはできないのです。 また、あらゆる式同様、
    // 構造体の新規インスタンスを関数本体の最後の式として生成して、 そのインスタンスを返すことを暗示できます。
    user1.email = String::from("anotheremail@example.com");

    // 構造体を用いた実装例
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area())
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)] // この implemention でデバッグする際の println! が使えるようになる
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

