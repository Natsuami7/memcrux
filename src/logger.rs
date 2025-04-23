use chrono::Local;

pub fn log(msg: &str){
    let now = Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"),msg);
}