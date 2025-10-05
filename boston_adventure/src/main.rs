use std::io::{self, Write};

fn main() {
    loop {
        println!("you wake up at fenway.");
        println!("what do you want to do?");
        println!("1. take the t to newbury?");
        println!("2. follow a rat up the street?");
        println!("3. take the t west to newton?");
        println!("4. quit");
        print!("> Choose 1, 2, 3, or 4: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        match input.trim() {
            "1" => println!("you arrive at newbury. you see a boba shop"),
            "2" => println!("you find a secret stash of burritos."),
            "3" => println!("the train gets stuck!."),
            "4" => {
                println!("you decide to quit. goodbye!");
                break;
            }, 
            _ => println!("invalid choice. a pigeon attacks you."),
        }
    }
}
