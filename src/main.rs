//Self note: do "rustc <file name and extenstion> to rebuild project. EX: rustc main.rs"
//Self note: do "press F5 to start debugging project"
//--------------for debugging-------------------//

//Self note: use "cargo build" or "cargo run"

// use std::io;
use std::io::{self, Write};

pub fn print_grammar() {
    println!(
        "BNF GRAMMAR is as follows:
***********************************************************************
     <program>-> ENTER <instructions> EXIT
<instructions>-> <instruction>
               | <instruction>; <instructions>
 <instruction>-> <buttons> = <direction>
   <direction>-> FORWARD | BACKWARD | LEFT | RIGHT | SRIGHT | SLEFT
     <buttons>-> <button> <key>
      <button>-> Button
         <key>-> W | A | S | D | Q | E
***********************************************************************
Input String Form: Button A = FORWARD; Button B = BACKWARD
***********************************************************************");
}

pub fn get_and_return_input() -> String {
    let mut input = String::new();
    print!("Input String(HALT to end): ");
    let _ = io::stdout().flush(); //allows the input to be on the same line as the prompt; link: https://www.folkstalk.com/2022/07/rust-get-input-on-the-same-line-as-question-with-code-examples.html

    io::stdin().read_line(&mut input).expect("Error reading from STDIN"); //reads line or shows error

    input.trim().to_string() //returns the input 
}

pub fn remove_spaces(string : String) -> String{
    string.replace(" ", "")
}

fn main() {
    print_grammar();
    let mut input = String::from(get_and_return_input().to_string());

    while input != "HALT"{
        input = remove_spaces(input);

        let new_input: Vec<char> = input.chars().collect(); //turning the input string to a vector; allows parsing by index; link: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
        
        for x in 0..new_input.len(){
            println!("{}", new_input[x]);


        }

        // ------------  showing input prompt again to restart or terminate program ------------//
        let mut option = String::new();
        print!("---Restarting---\nSee Grammar Again(Y)\nSkip(Enter)\nEnd Program(HALT)\n---------------\noption: "); let _ = io::stdout().flush();
        io::stdin().read_line(&mut option).expect("Error reading from STDIN for option");

        if option.trim() == "Y"{
            print_grammar();   
        }else if option.trim() == "HALT"{
            break;
        }
        input = get_and_return_input();
    }

    println!("End of program.");
}