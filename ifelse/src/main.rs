fn main() {
    let number = 3;
    if number < 5 {
        println!("小于5")
    } else {
        println!("大于5")
    }
    // if 条件必须是bool
    // if number {
    //     println!("xxx")
    // }
    if number != 0 {
        println!("不等于0")
    }
    let_if();
    loop_function();
    while_function();
    for_function();
}
// 在 let 语句中使用 if
fn let_if() {
    println!("let 语句使用if");
    let x = true;
    let y = if x { 5 } else { 6 };
    println!("{}", y)
}
// 从循环里返回
fn loop_function() {
    // 循环
    let mut x = 1;
    let result = loop {
        x += 1;
        println!("loop循环,x=={}", x);
        if x == 5 {
            // 打断语句 break
            break x * 2;
        }
    };
    println!("打断后的结果,{}", result)
}

// while 循环
fn while_function() {
    println!("while循环");
    let mut x = 3;
    while x != 0 {
        println!("{}", x);
        x -= 1;
    }
    println!("结束了{}", x)
}
// for循环
fn for_function() {
    println!("for循环");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("这个值为{},序号是{}", a[index], index);
        index = index + 1;
    }
    for_function_simple();
    for_map();
}
fn for_function_simple() {
    println!("---for循环简洁方案");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("这个值是{}", element)
    }
}
// for 循环遍历集合中元素 rev()来反转
fn for_map() {
    for elem in 1..4 {
        // for elem in (1..4).rev() {
        println!("{}", elem)
    }
}
