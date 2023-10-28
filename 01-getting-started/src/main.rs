use std::env;

fn main() {
    dotenv::dotenv().ok();

    let message = env::var("MESSAGE").unwrap();

    println!("Hello, {}", message);
}
