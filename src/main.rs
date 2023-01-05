use std::io;
use rand::Rng;
fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密的数字是: {}",secret_number);

    println!("猜测一个数!");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("读取行失败");
    println!("你猜测的数是: {}",guess);
}
