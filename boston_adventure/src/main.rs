use std::io::{self, Write};

fn main() {
    let mut current_scene = 0;
    loop {
        match current_scene {
            0 => {
                println!("you wake up at fenway.");
                println!("what do you want to do?");
                println!("1. take the t to newbury");
                println!("2. follow a rat up the street");
                println!("3. take the t west to newton");
                println!("4. quit");
                print!("> choose 1, 2, 3, or 4: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    "4" => {
                        println!("you decide to quit. goodbye!");
                        break;
                    }
                    _ => {
                        println!("invalid choice. try again.");
                        0
                    }
                };
            }

            1 => {
                println!("you arrive at newbury. there's a boba shop.");
                println!("1. get a boba");
                println!("2. keep walking");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 4,
                    "2" => 5,
                    _ => {
                        println!("invalid choice. try again.");
                        1
                    }
                };
            }

            2 => {
                println!("you follow the rat into an alley.");
                println!("1. chase it");
                println!("2. ignore it and explore the alley");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 6,
                    "2" => 7,
                    _ => {
                        println!("invalid choice. try again.");
                        2
                    }
                };
            }

            3 => {
                println!("you arrive in newton.");
                println!("1. visit a local cafe");
                println!("2. walk through the park");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 8,
                    "2" => 9,
                    _ => {
                        println!("invalid choice. try again.");
                        3
                    }
                };
            }

            4 => {
                println!("you enjoy the boba, but notice someone suspicious.");
                println!("1. confront them");
                println!("2. ignore and sip");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 10,
                    "2" => 11,
                    _ => {
                        println!("invalid choice. try again.");
                        4
                    }
                };
            }

            5 => {
                println!("walking past the boba shop, you see a bookstore.");
                println!("1. enter the bookstore");
                println!("2. keep walking");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 12,
                    "2" => 13,
                    _ => {
                        println!("invalid choice. try again.");
                        5
                    }
                };
            }

            6 => {
                println!("the rat leads you to a hidden door.");
                println!("1. open it");
                println!("2. stay outside");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 14,
                    "2" => 15,
                    _ => {
                        println!("invalid choice. try again.");
                        6
                    }
                };
            }

            7 => {
                println!("exploring the alley, you find a strange graffiti.");
                println!("1. take a picture");
                println!("2. ignore it");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 16,
                    "2" => 17,
                    _ => {
                        println!("invalid choice. try again.");
                        7
                    }
                };
            }

            8 => {
                println!("in the cafe, you see an old friend.");
                println!("1. say hi");
                println!("2. keep to yourself");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 18,
                    "2" => 19,
                    _ => {
                        println!("invalid choice. try again.");
                        8
                    }
                };
            }

            9 => {
                println!("walking in the park, you find a lost dog.");
                println!("1. help the dog");
                println!("2. ignore it");
                print!("> choose 1 or 2: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("failed to read line");
                current_scene = match input.trim() {
                    "1" => 20,
                    "2" => 21,
                    _ => {
                        println!("invalid choice. try again.");
                        9
                    }
                };
            }

            10 => {
                println!("you confront the suspicious person and they smile kindly.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            11 => {
                println!("you ignore them and finish your drink peacefully.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            12 => {
                println!("inside the bookstore, you find a rare edition.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            13 => {
                println!("you keep walking and enjoy the fresh air.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            14 => {
                println!("opening the door, you discover a hidden speakeasy.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            15 => {
                println!("you decide to stay outside and rest on the steps.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            16 => {
                println!("the graffiti inspires you to create art.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            17 => {
                println!("you ignore the graffiti and move on.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            18 => {
                println!("your friend invites you to join a book club.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            19 => {
                println!("you keep to yourself and finish your coffee quietly.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            20 => {
                println!("the lost dog becomes your loyal companion.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            21 => {
                println!("you ignore the dog and enjoy your walk alone.");
                println!("press enter to continue.");
                let _ = io::stdin().read_line(&mut String::new());
                break;
            }

            _ => {
                println!("something went wrong. returning to fenway.");
                current_scene = 0;
            }
        }
    }
}