fn main() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 100,
        height: 200,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 30,
    };
    println!("长方形面积,{}", rect.area());

    println!("rect2,{}", rect.can_hold(&rect2));
    println!("rect3,{}", rect.can_hold(&rect3));
    let rect4 = Rectangle::square(1);
    println!("rect4,{:#?}", rect4);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    /// 带更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // /// 关联参数
    // fn square(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size,
    //     }
    // }
}
/// 多个impl块
impl Rectangle {
    /// 关联参数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
