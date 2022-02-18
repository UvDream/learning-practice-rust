fn main() {
    // let name="张三";
    let name = String::from("张三");
    greet(name);
    // # 切片
    slice_fn();
    // # 其它切片
    other_slice();
    // # 操作字符串
    operate_string();
    // # 字符串拼接
    string_str();
    // # 字符串索引
    string_get();
    // # 操作utf8字符串
    operate_utf8();
}
fn greet(name: String) {
    println!("{}", name);
}
fn slice_fn() {
    println!("-------切片相关-------");
    let s = String::from("hello");
    // # 取第一位和第二位
    // 写法1
    let slice = &s[0..2];
    println!("{}", slice);
    // 写法2
    let slice = &s[..2];
    println!("{}", slice);
    // # 获取最后一个字节
    let len = s.len();
    println!("长度是={}", len);
    // 写法1
    let slice = &s[4..len];
    println!("获取最后一个字节={}", slice);
    // 写法2
    let slice = &s[4..];
    println!("获取最后一个字节={}", slice);
    // # 获取完整String
    // 写法1
    let slice = &s[0..len];
    println!("获取完整切片={}", slice);
    // 写法2
    let slice = &s[..];
    println!("获取完整切片={}", slice);
    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是UTF8字符的边界，例如中文在UTF8中占用三个字节
    // let s = "中国人";
    // let a = &s[0..1];
    // println!("{}", &a)
    // 以上取得是0和1位上的字符,但是中文占3个字节,也就是中没取完整,所以程序会崩溃
    let word = first_word(&s);
    println!("word={}", word);
    // 仅仅借用了不能改变
    // s.clear();
}
fn first_word(s: &String) -> &str {
    &s[0..1]
}
fn other_slice() {
    println!("--------其它切片--------");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
fn operate_string() {
    println!("--------操作字符串--------");
    let mut s = String::new();
    s.push_str("hello world");
    s.push_str("!");
    println!("s={}", s);

    // 从现有的&str切片创建String类型
    let mut s = "hello world".to_string();
    s.push_str("!");
    println!("s={}", s);
    let mut s = String::from("hello world");
    s.push_str("!");
    println!("s={}", s);
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 这个地方把s1的所有权转移走了,因此后面不能再使用s1
    let s3 = s1 + &s2;
    println!("s3={}", s3);
    // println!("s1={}", s1);
}
// &str==>String
// String::from("hello,world")
// "hello,world".to_string()

// String=>&str
// 取引用

fn string_str() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}
fn say_hello(s: &str) {
    println!("{}", s);
}
// 字符串索引
fn string_get() {
    println!("字符串索引");
    let s = String::from("hello,world!");
    // # 取第一个字符
    let first = s.chars().nth(0).unwrap();
    println!("{}", first);
}
fn operate_utf8() {
    println!("---------操作utf8字符串--------");
    for c in "中国人".chars() {
        println!("{}", c);
    }
    for c in "中国人".bytes() {
        println!("{}", c);
    }
}
