use std::io::{self, Write};

fn main() {
    let mut current_scene = 0;
    loop {
                match current_scene {
            0 => {

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
current_scene = match input.trim() {
    "1" => 1,
    "2" => {
        println!("you find a secret stash of burritos.");
        0
    },
    "3" => {
        println!("the train gets stuck!.");
        0
    },
    "4" => {
        println!("you decide to quit. goodbye!");
        break;
    }, 
    _ => {
        println!("invalid choice. a pigeon attacks you.");
        0
    }
};
            },
1 =>{
                println!("\nyou arrive at newbury. you see a boba shop.");
                println!("1. do you get the boba?");
                println!("2. do you walk past it?");
                println!("3. go back to fenway");
                print!("> choose 1, 2, or 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");

                current_scene = match input.trim() {
                    "1" => 10,
                    "2" => 20,
                    "3" => 0,
                    _ => {
                        println!("invalid choice. try again.");
                        1
                    }
                };
            }

            10 => {
                println!("\na brandy melville employee cuts in front of line.");
                println!("1. confront her");
                println!("2. wait your turn");
                println!("3. go back");
                print!("> choose 1, 2, or 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");

                current_scene = match input.trim() {
                    "1" => 11,
                    "2" => 12,
                    "3" => 1,
                    _ => {
                        println!("invalid choice. try again.");
                        10
                    }
                };
            }

            11 => {
                println!("\nshe apologizes and pays for your drink.");
                println!("press enter to go back to newbury.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 1;
            }

            12 => {
                println!("\nyou get your drink! unfortunately, a rat bumps into you and you spill it.");
                println!("press enter to go back to newbury.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 1;
            }

            20 => {
                println!("\nyou walk past it.");
                println!("1. go to newbury comics");
                println!("2. go to trident booksellers");
                println!("3. go back");
                print!("> choose 1, 2, or 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");

                current_scene = match input.trim() {
                    "1" => 21,
                    "2" => 30,
                    "3" => 1,
                    _ => {
                        println!("invalid choice. try again.");
                        20
                    }
                };
            }

            21 => {
                println!("\nyou enter newbury comics.");
                println!("1. buy a blind box");
                println!("2. buy a vinyl");
                println!("3. go back");
                print!("> choose 1, 2, or 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");

                current_scene = match input.trim() {
                    "1" => 22,
                    "2" => 23,
                    "3" => 20,
                    _ => {
                        println!("invalid choice. try again.");
                        21
                    }
                };
            }

            22 => {
                println!("\nyou buy a blind box.");
                println!("you leave the store excited! unfortunately, a rat steals it out of your bag.");
                println!("press enter to go back to newbury comics.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 21;
            }

            23 => {
                println!("\nyou buy a vinyl.");
                println!("the employee compliments your taste.");
                println!("press enter to go back to newbury comics.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 21;
            }

            30 => {
                println!("\nyou arrive at trident booksellers.");
                println!("1. try and get some work done in the cafe");
                println!("2. look for a new book");
                println!("3. go back");
                print!("> choose 1, 2, or 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");

                current_scene = match input.trim() {
                    "1" => 31,
                    "2" => 32,
                    "3" => 20,
                    _ => {
                        println!("invalid choice. try again.");
                        30
                    }
                };
            }

            31 => {
                println!("\nyou try and get some work done in the cafe.");
                println!("the cafe is closed! a rat gives you company as you sit on the curb.");
                println!("press enter to go back to trident booksellers.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 30;
            }

            32 => {
                println!("\nyou look for a new book.");
                println!("you find a great book!");
                println!("press enter to go back to trident booksellers.");
                let _ = io::stdin().read_line(&mut String::new());
                current_scene = 30;
            }

            _ => {
                println!("something went wrong. going back to fenway.");
                current_scene = 0;
            }
        }
    }
}
