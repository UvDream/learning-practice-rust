fn main() {
    let x = 5;
    println!("x==={}", x);
    let x = 6;
    println!("x==={}", x);
    let mut a=1;
    println!("a==={}",a);
    a=2;
    println!("a==={}",a);
    let guess:i32= ("42").parse().expect("不是一个数字");
    println!("guess==={}",guess);
//     数据类型
//     浮点类型
    let _b=2.0;
    let _y:f32 =3.0;
//     运算符
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;
//     布尔
    let _t=true;
    let f:bool = false;
//     字符类型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
//     复合类型
//     元祖
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // 访问元组方式1
    println!("_tup: {}", _tup.1);
    //访问元组方式2
    let (x,y,z)=_tup;
    println!("y==={}",y);
//     数组
    let a=[1,2,3,4,5];
    //如果数组里面的各项都是一样
    let a=[3,5];
    println!("a={}",a[0]);
}
