use chrono::prelude::*;
fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("{}", utc);
    println!("{}", local);
    // 格式化
    let dt = utc.format("%Y-%m-%d %H:%M:%S");
    println!("{}", dt);
}
