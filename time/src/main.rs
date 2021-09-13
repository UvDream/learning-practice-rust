use chrono::prelude::*;
// mod math {
//     include!("math.rs");
// }
mod format_time;
mod math;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    math::add(1, 2);
    println!("{}", utc);
    println!("{}", local);
    format_time::format_function();
}
