use std::io;

fn main() {
    println!("Hello, Please enter your name!");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
        
    println!("The name you entered: {}", name);
}
