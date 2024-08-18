fn main() {
        // ベクタ
    // 単体のデータ構造でありながら複数の値を保持できる
    // ベクタには同じ型の値しか保持できない

    // 空の i32 型のベクタ
    let vector1: Vec<i32> = Vec::new();
    // mut ではないため push は不可
    // vector1.push(1);

    // 1, 2, 3 を含む i32 のベクタ
    let mut vector2 = vec![1, 2, 3];
    vector2.push(4);
    vector2.push(5);
    vector2.push(6);

    // ベクタの要素取り出し
    let vector3 = vec![1, 2, 3, 4, 5];
    let third = &vector3[2];
    // println!("{}", third); // 3

    // // ベクタの不可能な取り出し
    // let vector4 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &vector4[100];
    // let does_not_exist = vector4.get(100);

    // ベクタ要素の for 文
    let vector5 = vec![100, 32, 57];
    for i in &vector5 {
        // println!("{}", i);
    }

    // ベクタと enum を組み合わせて複数の値を保持できる
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 様々な String
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // String の拡張(push_str)
    let mut extended_string = String::from("foo");
    extended_string.push_str("bar");
    // println!("{}", extended_string); // foobar

    // String のスライス
    for c in "नमस्ते".chars() {
        // println!("{}", c);
    }

    // ハッシュマップ
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores)  // {"Blue": 10, "Yellow": 50}

    // ハッシュマップに格納した後の所有権
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // println!("before hash: {}", field_name);  // ここではハッシュマップに格納される前であるため参照可能
    map.insert(field_name, field_value);
    // println!("after hash: {}", field_name);  // ハッシュマップに格納されるため参照できない

    // ハッシュマップの取り出し
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}