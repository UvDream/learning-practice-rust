/*
 * @Author: wangzhongjie
 * @Date: 2021-09-13 22:11:54
 * @LastEditors: wangzhongjie
 * @LastEditTime: 2021-09-13 22:20:09
 * @Description:悬垂引用（Dangling References）
 * @Email: UvDream@163.com
 */
pub fn dangling_pointer() {
    println!("悬垂引用");
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
    println!("悬垂引用");
}
// 引用必须总是有效的
fn dangle() -> String {
    let s = String::from("Dangling References");
    // &s
    s
}
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！
