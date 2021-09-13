mod dangling_references;
fn main() {
    let mut s1 = String::from("hello");
    // &代表引用,可以拿到值,但是拿不到所有权
    let len = calculate_length(&s1);
    println!("{}", len);
    change_s(&mut s1);
    print!("{}", s1);
    dangling_references::dangling_pointer();
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
