mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 使用use 将函数的父级模块引入作用域
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
