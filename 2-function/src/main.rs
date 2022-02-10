fn main() {
    println!("Hello, world!");
    another_function();
    let x = five(2);
    println!("这个值是:{}", x);
    let y = five_another(5);
    println!("{}", y)
}
fn another_function() {
    println!("Hello, another_function!");
}
// 具有返回值的函数
fn five(x: i32) -> i32 {
    return x + 1;
}
fn five_another(a: i32) -> i32 {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x=={}", x);
    println!("y=={}", y);
    a + 1
}
