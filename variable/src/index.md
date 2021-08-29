# 变量和可变性
## 不可变
`rust`中默认变量是`不可变`(`immutable`)的
```rust
fn main() {
    let x = 5;
    println!("这个值是: {}", x);
    x = 6;
    println!("这个值是: {}", x);
}
```
> 以上就是错误的,`x`值默认是不可变的,所以再次赋值就是错误的
![error](https://pic.baixiongz.com/uploads/2021/08/29/ae808b16e322a.png)

### 解决上述问题
- `mut`
> 这个关键字是这个变量成为可变变量
```rust
fn main() {
    let mut x = 5;
    println!("这个值是: {}", x);
    x = 6;
    println!("这个值是: {}", x);
}
```
- `let`
> 再次声明赋值就可以解决,这叫shadow,这和其他编程语言区别很大,也就是相同一个变量可以重新赋值赋类型,之前的变量就会隐藏

1. 可以使用相同名字声明的新的变量,新的变量就会shadow(隐藏)之前的声明的变量
2. shadow和把变量标记mut是不一样的;mut标记的变量值可以变,但是类型不可以,shadow不同,类型可以变
```rust
fn main() {
    let x = 5;
    println!("这个值是: {}", x);
    let x = 6;
    println!("这个值是: {}", x);
}
```
## 常量`const`
- 常量不可变
- 常量必须大写,多个字母`_`分割
- 常量不可以`mut`标记
- 声明使用`const`