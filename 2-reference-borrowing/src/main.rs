mod dangling_references;
mod fn_return_val;
fn main() {
    // 基础
    base();
    let mut s1 = String::from("hello");
    // &代表引用,可以拿到值,但是拿不到所有权
    let len = calculate_length(&s1);
    println!("{}", len);
    change_s(&mut s1);
    print!("{}", s1);
    dangling_references::dangling_pointer();
    // 拷贝涉及到的所有权
    fn_return_val::test_fn();
}
fn base() {
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
// 这个函数里面的s就是借用
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 可变引用,可变引用很大限制,特定域内特定数据只能有一个可变引用,这样避免了数据竞争
fn change_s(s: &mut String) {
    s.push_str(", world");
    let mut x = String::from("hello");
    {
        let x1 = &mut x;
        println!("x1=={}", x1);
    }
    let x2 = &mut x;
    println!("x2=={}", x2);
    // 类似的规则也存在于同时使用可变与不可变引用中
    let mut y = String::from("这是y");
    let y1 = &y;
    let y2 = &y;
    println!("{},{}", y1, y2);
    let y3 = &mut y;
    println!("{}", y3);
}
