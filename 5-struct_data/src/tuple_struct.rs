/*
 * @Author: wangzhongjie
 * @Date: 2021-09-16 22:04:37
 * @LastEditors: wangzhongjie
 * @LastEditTime: 2021-09-16 22:39:26
 * @Description:
 * @Email: UvDream@163.com
 */
/// 元祖结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
pub fn tuple() {
    println!("---------------元祖结构体-------------");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?}", black);
    println!("{:#?}", origin);
    class_struct();
    calc_area();
    tuple_area();
    struct_area()
}
/// 类单元结构体
fn class_struct() {}
fn calc_area() {
    println!("---------------计算面试--------");
    let width1 = 30;
    let height1 = 50;
    println!("长方形的面积是{}", area(width1, height1));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
/// 元祖结构体优化计算面积
fn tuple_area() {
    println!("---------------元祖计算优化--------");
    let rect = (30, 50);
    println!("长方形面积是:{}", struct_area_calc(rect));
}
fn struct_area_calc(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
/// 结构体计算面积
struct Rectangle {
    width: u32,
    height: u32,
}
fn struct_area() {
    println!("---------------具体化结构体计算优化--------");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("具体的结构体计算的面积{}", struct_new_area_calc(&rect));
}
fn struct_new_area_calc(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
