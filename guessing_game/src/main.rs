use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜猜这个数字");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("随机的数字是:{}", secret_number);
    loop {
        println!("请输入你的数字");

        //  mut代表可变
        //  String代表返回一个String的新实例
        //  ::语法表名new是String类型的一个关联函数(静态属性)
        let mut guess = String::new();
        //let foo=5;//foo是不可变
        io::stdin()
            .read_line(&mut guess)
            // expect 错误捕捉
            .expect("读取失败");
        println!("你的猜测是:{}", guess);
        // 类型转换
        // let guess: u32 =  guess.trim().parse().expect("请输入一个数字");
        let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你猜小了"),
            Ordering::Greater => println!("你猜大了"),
            Ordering::Equal => {println!("猜对了!");break;},
        };
    }
}
