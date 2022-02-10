/*
 * @Author: wangzhongjie
 * @Date: 2022-02-10 15:41:06
 * @LastEditors: wangzhongjie
 * @LastEditTime: 2022-02-10 15:41:07
 * @Description:
 * @Email: UvDream@163.com
 */
pub fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    // 基本类型在编译的时候大小是知道大小的,所以下面这段代码所有权不会改变,程序会自动深拷贝x给y
    // 有哪些基本类型
    // 1.所有整数类型
    // 2.布尔型
    // 3.浮点型
    // 4.字符类型
    // 5.元祖
    let x = 1;
    let y = x;
    println!("{}", x);
    println!("{}", y);
}
