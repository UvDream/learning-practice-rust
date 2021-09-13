/*
 * @Author: wangzhongjie
 * @Date: 2021-09-13 17:40:52
 * @LastEditors: wangzhongjie
 * @LastEditTime: 2021-09-13 17:44:44
 * @Description:
 * @Email: UvDream@163.com
 */
use chrono::prelude::*;

pub fn format_function() {
    let utc: DateTime<Utc> = Utc::now();
    let dt = utc.format("%Y-%m-%d %H:%M:%S");
    println!("{}", dt);
}
