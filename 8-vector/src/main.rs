use std::vec;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push("张三");
    v.push("李四");
    v.push("王五");

    println!("{:?}", v);
    // 读取vector元素
    let third = &v[1];
    println!("取到的值=>{}", third);
    match v.get(1) {
        Some(element) => println!("匹配的元素是=>{}", element),
        None => println!("暂无这个元素"),
    }
    // 读取以外的元素
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("不存在的元素=>{:?}", does_not_exist);
    range_vector(v);
    enum_vector();
}
// 遍历vector元素
fn range_vector(v: Vec<&str>) {
    for i in &v {
        println!("遍历=>{}", i)
    }
    let mut a = vec![100, 30, 20];
    for i in &mut a {
        *i += 1
    }
    println!("{:?}", &a);
}
fn enum_vector() {
    println!("------枚举存储多种类型------");
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("{:?}", row)
}
