//[package]
//name = "Rock–paper–scissors"
//version = "0.1.0"
//authors = ["hzqd <hzqelf@yeah.net>"]
//edition = "2018"
//# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
//[dependencies]
//rand = "0.7.0"
extern crate rand;

use rand::Rng;
use std::io::stdin;

fn scan() -> String {
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    a.trim().to_string()
}

fn main() {
    loop {
        println!("请输入 石头/剪刀/布");
        let rand = rand::thread_rng().gen_range(1, 4);
        let computer = if rand == 1 { "石头" } else if rand == 2 { "剪刀" } else { "布" };
        match &*scan() {
            "石头" =>
                match computer {
                    "石头" => println!("计算机为{}，平局\n", computer),
                    "剪刀" => println!("计算机为{}，获胜\n", computer),
                    _ => println!("计算机为{}，败北\n", computer)
                }
            "剪刀" =>
                match computer {
                    "石头" => println!("计算机为{}，败北\n", computer),
                    "剪刀" => println!("计算机为{}，平局\n", computer),
                    _ => println!("计算机为{}，获胜\n", computer)
                }
            "布" =>
                match computer {
                    "石头" => println!("计算机为{}，获胜\n", computer),
                    "剪刀" => println!("计算机为{}，败北\n", computer),
                    _ => println!("计算机为{}，平局\n", computer)
                }
            _ => ()
        }
    }
}
