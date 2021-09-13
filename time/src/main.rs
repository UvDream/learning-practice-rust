use chrono::prelude::*;
// mod math {
//     include!("math.rs");
// }
mod format_time;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("{}", utc);
    println!("{}", local);
    format_time::format_function();
}
