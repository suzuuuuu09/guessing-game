use proconio::input;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();
    let rand_num: i32 = rng.gen_range(1..101);
    print!("1~100の数字を当ててください！: ");
    io::stdout().flush().unwrap();
    compare_rand_num(rand_num);
}

fn compare_rand_num(rand_num: i32) {
    loop {
        input! {
            guess: i32
        }
        if guess == rand_num {
            println!("正解！答えは{}でした！", rand_num);
            break;
        } else if guess < rand_num {
            println!("もっと大きいです！");
        } else {
            println!("もっと小さいです！");
        }
        print!("1~100の数字を当ててください！: ");
        io::stdout().flush().unwrap();
    }
}
