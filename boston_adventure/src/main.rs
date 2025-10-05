use std::io::{self, Write};

struct Scene {
    description: &'static str,
    options: Vec(& 'static str, usize)>,
}

fn main() {
    println!("you wake up at fenway.");
    println!("what do you want to do?");
    println!("1. take the t to newbury?");
    println!("2. follow a rat up the street?");
    println!("3. take the t west to newton?");
    print!("> Choose 1, 2, or 3: ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1" => println!("you arrive at newbury. you see a boba shop"),
        "2" => println!("you find a secret stash of burritos."),
        "3" => println!("the train gets stuck!."),
        _ => println!("invalid choice. a pigeon attacks you."),
    }
}
