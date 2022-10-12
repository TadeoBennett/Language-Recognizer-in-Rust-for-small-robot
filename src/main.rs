//Self note: do "rustc <file name and extenstion> to rebuild project. EX: rustc main.rs"
//Self note: do "press F5 to start debugging project"
//--------------for debugging-------------------//

//Self note: use "cargo build" or "cargo run"
//run
// use std::io;
use std::io::{self, Write};
mod functions;
mod classes;

fn main() {
    functions::print_grammar();
    let mut input = String::from(functions::get_and_return_input().to_string());

    while input != "HALT"{
        input = functions::remove_spaces(input);

        let new_input: Vec<char> = input.chars().collect(); //turning the input string to a vector; allows parsing by index; link: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
        
        if functions::check_for_errors(new_input.clone()) != false{
            functions::output_decision_menu();
            let mut decision = String::from(functions::get_decision().to_string());
            loop {
                if decision == "1"{
                    if functions::print_derivations(new_input.clone()) == false{
                        break;
                    }
                } else if decision == "2"{
                    functions::print_parse_tree(new_input.clone());
                } else if decision == "3"{
                    break;
                } else {
                    println!("Not a valid option");
                }
                functions::output_decision_menu();
                decision = functions::get_decision();
            }
    
            // ------------  showing input prompt again to restart or terminate program ------------//
            let mut option = String::new();
            print!("\n---Restarting---\nSee Grammar Again(Y)\nSkip(Enter)\nEnd Program(HALT)\n---------------\noption: "); let _ = io::stdout().flush();
            io::stdin().read_line(&mut option).expect("Error reading option from STDIN for option");
    
            if option.trim() == "Y"{
                functions::print_grammar();   
            }else if option.trim() == "HALT"{
                break;
            }else{
                println!("skipped...\n");
            }
        }
        input = functions::get_and_return_input();
    }

    println!("End of program.");
}