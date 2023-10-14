extern crate gessing_game;

use std::io;
use rand::Rng;

fn main() {
    println!("あててね");

    let secret_number = rand::thread_rng().gen_range(1..11);
    //println!("答えは{}", secret_number);
    
    loop {
        println!("予想を入力");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込み失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値を入力してください");
                continue;
            }
        };

        println!("次のように予想しました: {}", guess);

        let result = gessing_game::judge(&secret_number, &guess);
        println!("{}", result.0);
        if result.1 {break;}

        // 練習のため外だししてテストを追加します。
        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("小さい"),
        //     Ordering::Greater => println!("大きい"),
        //     Ordering::Equal => {
        //         println!("あたり");
        //         break;
        //     }
        // }
    }
}
