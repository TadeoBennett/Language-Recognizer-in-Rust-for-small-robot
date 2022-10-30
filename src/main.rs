//Self note: do "rustc <file name and extenstion> to rebuild project. EX: rustc main.rs"
//Self note: do "press F5 to start debugging project"
//--------------for debugging-------------------//

//Self note: use "cargo build" or "cargo run"
//run
// use std::io;
use std::io::{self, stdin, stdout, Write};

use crate::functions::pause;

mod derivations;
mod functions;
mod parsetree;
mod bsp;

fn main() {
    functions::print_grammar();
    let mut input = String::from(functions::get_and_return_input().to_string());

    while input != "HALT" {
        input = functions::remove_spaces(input);

        let new_input: Vec<char> = input.chars().collect(); //turning the input string to a vector; allows parsing by index; link: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
        if functions::check_for_errors(new_input.clone()) != false {
            loop{
                let new_input = functions::remove_prefix_and_suffix(new_input[0..new_input.len()].iter().collect::<String>()); //removes ENTER and EXIT from the string
                let instructions_group: Vec<&str> = new_input.split_inclusive(";").collect();                                      //create group of instructions sepatated by a semicolon
                if derivations::print_derivations(instructions_group.clone(), instructions_group.len()) == false {
                    break;
                }
                println!("Derivation successful\n");
                functions::pause();
                if parsetree::print_parsetree(instructions_group.clone(), instructions_group.len()) == false {
                    break;
                }
                println!("Parse tree done\n");
                functions::pause();
                bsp::generate_bsp_file(instructions_group.clone(), instructions_group.len());
                functions::pause();
                break;
            }
        }else{
            functions::pause();
        }
        functions::print_grammar();
        input = functions::get_and_return_input();
    }

    println!("End of program.");
}
