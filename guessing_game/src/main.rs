use std::io;
fn main(){
    println!("猜猜这个数字");
    println!("请输入你的数字");
    
    //  mut代表可变
    //  String代表返回一个String的新实例
    //  ::语法表名new是String类型的一个关联函数(静态属性)
    let mut guess=String::new();
    let foo=5;//foo是不可变
    io::stdin()
        .read_line(&mut guess)
        // expect 错误捕捉
        .expect("读取失败");
        println!("你的猜测是:{}",guess)
}