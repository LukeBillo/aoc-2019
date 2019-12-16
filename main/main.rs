use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file_name = &args[1];

    println!("{:?}", file_name);
    day_1::run(file_name);
}