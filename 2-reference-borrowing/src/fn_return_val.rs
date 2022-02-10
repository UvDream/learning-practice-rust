pub fn test_fn() {
    let s1 = String::from("hello");
    get_string(s1);
    // 在这里s1已经被移除了,所有权已经被转移
    // println!("{}", s1);
    let a = 2;
    get_number(a);
    println!("{}", a)
}
fn get_string(s: String) {
    println!("{}", s)
}
fn get_number(a: i32) {
    println!("{}", a)
}
