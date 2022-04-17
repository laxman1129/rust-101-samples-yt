use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // first arg is path of the executable
    // if we execute cargo run hello => args : ["target/debug/cargo", "hello"]
    println!("args : {:?}", args);

    let command = args[1].clone();
    println!("command : {:?}", command);
}
