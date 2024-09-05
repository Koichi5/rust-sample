use core::num;
use rand::Rng;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    // ジェネリクス
    // リスト内の最大値を求めるプログラム
    fn max_number() -> i32 {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut max_number = &number_list[0];

        for number in &number_list {
            if number > max_number {
                max_number = number;
            }
        }

        // println!("The largest number is {}", max_number);
        return *max_number;
    }

    // 上記の max_number から最大値を求める部分を抽出して使い回す
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list1 = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list1);
    // println!("The largest number is {}", result);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list2);
    // println!("The largest number is {}", result);

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // // ジェネリクスを使って表現
    // // このままでは item > largest でエラーになる
    // fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];

    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // fn main() {
    //     let number_list = vec![34, 50, 25, 100, 65];

    //     let result = largest(&number_list);
    //     println!("The largest number is {}", result);

    //     let char_list = vec!['y', 'm', 'a', 'q'];

    //     let result = largest(&char_list);
    //     println!("The largest char is {}", result);
    // }

    // トレイト
    // 型の振る舞いは、その方に対して呼び出せるメソッドから構成される。
    // 異なる型は、それらの方全てに対して同じメソッドを呼び出せるのであれば、同じ振る舞いを共有する
    // トレイトの定義は、メソッドシグニチャをあるグループにまとめ、何らかの目的を
    // 達成するのに必要な一連の振る舞いを定義する手段である
    pub trait Summary {
        fn summarize(&self) -> String {
            // デフォルトの値を代入可能
            String::from("(Read more...)")
        }
    }

    pub trait Authror {
        fn author(&self) -> String;
    }

    // トレイトを方に実装する
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    impl Authror for NewsArticle {
        fn author(&self) -> String {
            format!("The author of this article is {}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Authror for Tweet {
        fn author(&self) -> String {
            format!("The author of this tweet is {}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people,"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", tweet.author());

    // // Dart で書くとすると abctract を使うことになる
    //     final newsArticle = NewsArticle(
    //       headline: "Today's weather",
    //       location: 'Osaka',
    //       author: 'Dart boy',
    //       content: "Today's weather in Osaka is cloudy",
    //     );

    //     final tweet = Tweet(
    //       username: 'Dart boy',
    //       content: 'Hello World !',
    //       reply: false,
    //       retweet: false,
    //     );

    //     print('News summary: ${newsArticle.summarize()}');
    //     print("News writer: ${newsArticle.writer()}");
    //     print('Tweet summary: ${tweet.summarize()}');
    //     print("Tweet writer: ${tweet.writer()}");
    //   }

    //   // Summary トレイトの代わりに抽象クラスを使用
    //   abstract class Summary {
    //     String summarize();
    //   }

    //   abstract class Writer {
    //     String writer();
    //   }

    //   class NewsArticle implements Summary, Writer {
    //     final String headline;
    //     final String location;
    //     final String author;
    //     final String content;

    //     NewsArticle({
    //       required this.headline,
    //       required this.location,
    //       required this.author,
    //       required this.content,
    //     });

    //     @override
    //     String summarize() {
    //       return '$headline, by $author ($location)';
    //     }

    //     @override
    //     String writer() {
    //       return 'This article is written by $author';
    //     }
    //   }

    //   class Tweet implements Summary, Writer {
    //     final String username;
    //     final String content;
    //     final bool reply;
    //     final bool retweet;

    //     Tweet({
    //       required this.username,
    //       required this.content,
    //       required this.reply,
    //       required this.retweet,
    //     });

    //     @override
    //     String summarize() {
    //       return '$username: $content';
    //     }

    //     @override
    //     String writer() {
    //       return 'This tweet is writtern by $username';
    //     }
    //   }
}
