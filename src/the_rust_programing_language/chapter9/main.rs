fn main() {
    // panic!マクロが実行されると、プログラムは失敗のメッセージを表示し、スタックを巻き戻し掃除して、終了します。
    // 何らかのバグが検出された時に使用されることが多い
    panic!("crash and burn");

    // panic をエラー処理に組み込む
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // panic を自動的に呼んでくれる unwrap
    // Ok の場合は読み取った値を、 Error の場合は panic を呼ぶ
    let f = File::open("hello.txt").unwrap();

    // エラー文をカスタムして panic を呼んでくれる expect
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // エラーを移譲する
    // Result値の直後に置かれた?は、リスト9-6でResult値を処理するために定義したmatch式とほぼ同じように動作します。
    // Resultの値がOkなら、Okの中身がこの式から返ってきて、プログラムは継続します。
    // 値がErrなら、 returnキーワードを使ったかのように関数全体からErrの中身が返ってくるので、 エラー値は呼び出し元のコードに委譲されます。
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}