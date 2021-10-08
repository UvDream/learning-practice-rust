fn main() {
    update_string()
}
fn update_string() {
    let mut s = String::from("哈哈");
    // println!("{}", &s);
    // 更新字符串
    s.push_str("嘻嘻");
    println!("{}", &s);

    // push 方法被定义为获取一个单独的字符作为参数
    s.push('1');
    println!("{}", &s);
    // 拼接字符串
    let s1 = String::from("这是s2");
    let s2 = s + &s1;
    println!("{}", &s2);
}
