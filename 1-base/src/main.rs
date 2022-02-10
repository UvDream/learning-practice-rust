fn main() {
    // let 用来声明变量的,进行绑定,a是不可变的
    //此处a没有申明类型,编译器会默认根据a的值为a推断类型:i32,有符号32位整数
    //语句的末尾必须分好结尾,否则编译器会报错
    let a = 10;
    // 主动给b指定类型
    let b: i32 = 20;
    //这里两点注意的
    //1.这里数值中带上类型,30i32表示数据32,类型是i32
    // 2.c是可变的,mut是mutable缩写
    let mut c = 30i32;
    // c = 30;
    // 还能在数值和类型之间添加下划线,让可读性更好
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("add(add(a,b),add(c,d))={}", e);
    structure()
}
fn add(i: i32, j: i32) -> i32 {
    // 返回值相加,这里可以省略return
    i + j
    // return i+j;
}
// 变量结构
fn structure() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
    // 会引起程序崩溃
    // assert!(0.1 + 0.2 == 0.3)
    // 如果非要比较的话
    // (0.1 + 0.2 - 0.3).abs() < 0.00001 具体小于多少取决于你的精度
}
