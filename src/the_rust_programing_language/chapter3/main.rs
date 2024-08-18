fn main() {
    // // 代入
    // // case 1
    // let x = 5;
    // println!("The value of x is: {}", x);
    // // 不変変数xに二回代入はできないためエラーになる
    // x = 6;
    // println!("The value of x is: {}", x);

    // // case 2
    // let mut x = 5;
    // println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x); // 12
    // }

    // println!("The value of x is: {}", x); // 6

    // // これはOK → 同じ名前の変数であったとしても let をつけることで新しい変数を生成していることになる
    // let spaces = "   ";
    // let spaces = spaces.len();

    // // これはNG → 変数の型は可変にすることはできない
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // let x = 2.0;
    // let y: f32= 3.0;

    // // タプル
    // let tupple = (500, 6.4, 1);
    // let (x, y, z) = tupple;
    // println!("The value of x is: {}", x); // 500
    // println!("The value of y is: {}", y); // 6.4

    // // {tuppleの変数名}.{要素index} でタプルの要素の引き出しが可能
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;  // 500
    // let six_point_four = x.1;  // 6.4
    // let one = x.2; // 1

    // // 配列
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    // "August", "September", "October", "November", "December"];

    //  // 3 が 5つある配列。[3,3,3,3,3]と同値
    // let five_threes = [3; 5];

    // let first = a[0];  // 1
    // let second = a[1];  // 2

    // // 関数
    // another_function(5);
    // print_labeled_measurement(12, '月');

    // let x = five();
    // println!("The value of x is: {}", x);

    // // {}で囲まれたブロック内に関数として式を書くことも可能
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {}", y);

    // let z = plus_one(10);
    // println!("The value of z is: {}", z);

    // // 条件分岐
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // // 三項演算子的な書き方
    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {}", number);

    // 内側と外側の loop 文を区別
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("Remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // // while 文
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }
    // println!("LIFT OFF !")

    // // リストの長さに応じた while 文
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // // リストの数以上の回数だけ while 文を回すとエラーになる
    // // index out of bounds: the len is 5 but the index is 5
    // while index < a.len() {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }

    // // for 文
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFT OFF !");

    // // for in 文
    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("The value is {}", element);
    // }
}

// fn another_function(x: i32) {
//     println!("Another function.");
//     println!("The value of x is: {}", x);
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// fn five() -> i32 {
//     5  // Don't need "return"
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1  // この文末に ; をつけると「文」として認識され、返り値がないと判断される
// }
