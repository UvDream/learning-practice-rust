/*
 * @Author: wangzhongjie
 * @Date: 2022-02-18 11:25:27
 * @LastEditors: wangzhongjie
 * @LastEditTime: 2022-02-18 13:54:36
 * @Description:元祖
 * @Email: UvDream@163.com
 */
fn main() {
    // # 用模式匹配结构元祖
    match_tuple();
    // # 用.来访问元祖
    access_tuple();
    // # 元祖的使用
    use_tuple();
}
fn match_tuple() {
    println!("-----模式匹配------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={},y={},z={}", x, y, z);
}
fn access_tuple() {
    println!("------用点访问------");
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let a = x.0;
    let b = x.1;
    let c = x.2;
    println!("a={},b={},c={}", a, b, c)
}
fn use_tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2={},len={}", s2, &len);
}

fn calculate_length(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}
