use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(s: &str) -> String;
}


#[wasm_bindgen]
pub fn start_adventure() {
    alert("welcome to boston adventure!");

    let mut current_scene = 0;

    loop {
        let _choice = match current_scene {
        0 => {
            alert("you wake up at fenway.");
            alert("what do you want to do?");
            alert("1. take the t to newbury");
            alert("2. follow a rat up the street");
            alert("3. take the t west to newton");
            alert("4. quit");
            alert("> choose 1, 2, 3, or 4: ");

            
let input = prompt("> choose 1, 2, 3, or 4:");
            current_scene = match input.trim() {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => {
                    alert("you decide to quit. goodbye!");
                    break;
                }
                _ => {
                    alert("invalid choice. try again.");
                    0
                }
            };
        }

        1 => {
            alert("you arrive at newbury. there's a boba shop.");
            alert("1. get a boba");
            alert("2. keep walking");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 4,
                "2" => 5,
                _ => {
                    alert("invalid choice. try again.");
                    1
                }
            };
        }

        2 => {
            alert("you follow the rat into an alley.");
            alert("1. chase it");
            alert("2. ignore it and explore the alley");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 6,
                "2" => 7,
                _ => {
                    alert("invalid choice. try again.");
                    2
                }
            };
        }

        3 => {
            alert("you arrive in newton.");
            alert("1. visit a local cafe");
            alert("2. walk through the park");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 8,
                "2" => 9,
                _ => {
                    alert("invalid choice. try again.");
                    3
                }
            };
        }

        4 => {
            alert("you enjoy the boba, but notice someone suspicious.");
            alert("1. confront them");
            alert("2. ignore and sip");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 10,
                "2" => 11,
                _ => {
                    alert("invalid choice. try again.");
                    4
                }
            };
        }

        5 => {
            alert("walking past the boba shop, you see a bookstore.");
            alert("1. enter the bookstore");
            alert("2. keep walking");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 12,
                "2" => 13,
                _ => {
                    alert("invalid choice. try again.");
                    5
                }
            };
        }

        6 => {
            alert("the rat leads you to a hidden door.");
            alert("1. open it");
            alert("2. stay outside");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 14,
                "2" => 15,
                _ => {
                    alert("invalid choice. try again.");
                    6
                }
            };
        }

        7 => {
            alert("exploring the alley, you find a strange graffiti.");
            alert("1. take a picture");
            alert("2. ignore it");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 16,
                "2" => 17,
                _ => {
                    alert("invalid choice. try again.");
                    7
                }
            };
        }

        8 => {
            alert("in the cafe, you see an old friend.");
            alert("1. say hi");
            alert("2. keep to yourself");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 18,
                "2" => 19,
                _ => {
                    alert("invalid choice. try again.");
                    8
                }
            };
        }

        9 => {
            alert("walking in the park, you find a lost dog.");
            alert("1. help the dog");
            alert("2. ignore it");
            alert("> choose 1 or 2: ");
            

            
            let input = prompt("> choose 1 or 2:");
            current_scene = match input.trim() {
                "1" => 20,
                "2" => 21,
                _ => {
                    alert("invalid choice. try again.");
                    9
                }
            };
        }

        10 => {
            alert("you confront the suspicious person and they smile kindly.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        11 => {
            alert("you ignore them and finish your drink peacefully.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        12 => {
            alert("inside the bookstore, you find a rare edition.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        13 => {
            alert("you keep walking and enjoy the fresh air.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        14 => {
            alert("opening the door, you discover a hidden speakeasy.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        15 => {
            alert("you decide to stay outside and rest on the steps.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        16 => {
            alert("the graffiti inspires you to create art.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        17 => {
            alert("you ignore the graffiti and move on.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        18 => {
            alert("your friend invites you to join a book club.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        19 => {
            alert("you keep to yourself and finish your coffee quietly.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        20 => {
            alert("the lost dog becomes your loyal companion.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        21 => {
            alert("you ignore the dog and enjoy your walk alone.");
            alert("press enter to continue.");
            let _ = prompt("press enter to continue.");

            break;
        }

        _ => {
            alert("something went wrong. returning to fenway.");
            current_scene = 0;
        }
        };
}
}