use std::io::{self, Write};


pub fn print_grammar() {
   println!(
       "BNF GRAMMAR is as follows:
***********************************************************************
    <program>-> ENTER <instructions> EXIT
<instructions>-> <instruction>
              | <instruction>; <instructions>
<instruction>-> <buttons> = <direction>
  <direction>-> FORWARD | BACKWARD | SRIGHT | SLEFT
    <buttons>-> <button> <key>
     <button>-> Button
        <key>-> A | B | C | D
***********************************************************************
Input String Form: Button A = FORWARD; Button B = BACKWARD
***********************************************************************");
}

pub fn output_decision_menu(){
   println!("
***********************
1. Print Derivations
2. Print Parse Tree
3. Exit
***********************");
}

pub fn get_and_return_input() -> String {
   let mut input = String::new();
   print!("Input String(HALT to end): ");
   let _ = io::stdout().flush(); //allows the input to be on the same line as the prompt; link: https://www.folkstalk.com/2022/07/rust-get-input-on-the-same-line-as-question-with-code-examples.html

   io::stdin().read_line(&mut input).expect("Error reading from STDIN"); //reads line or shows error

   input.trim().to_string() //returns the input 
}

pub fn get_general_input() -> String {
   let mut input = String::new();
   let _ = io::stdout().flush(); 

   io::stdin().read_line(&mut input).expect("Error reading from STDIN"); //reads line or shows error

   input.trim().to_string() //returns the input 
}

pub fn get_decision() -> String {
   let mut input = String::new();
   print!("option: ");
   let _ = io::stdout().flush();
   io::stdin().read_line(&mut input).expect("Error reading decision"); //reads line or shows error
   input.trim().to_string() //returns the input 
}

pub fn remove_spaces(string : String) -> String{
   string.replace(" ", "")
}

pub fn remove_prefix_and_suffix(string: String) -> String{
   let mut _string = string.replace("ENTER", "");
   _string = _string.replace("EXIT", "");
   _string //returns the new string without the ENTER AND EXIT text
}

pub fn check_for_errors(string: Vec<char>) -> bool{
   if string.len() <= 9{
      println!("String does not meet required length for a single command.\n");
      return false
   }

   //string examples: ENTER Button A = FORWARD; EXIT
   if string[0..5].iter().collect::<String>() != "ENTER"{
       println!("String must start with the key word 'ENTER', you provided: {}\n", string[0..5].iter().collect::<String>());
       return false
   }

   if string[string.len()-4..string.len()].iter().collect::<String>() != "EXIT"{
       println!("String must end with the key word 'EXIT', you provided: {}\n", string[string.len()-4..string.len()].iter().collect::<String>());
       return false
   }

   for x in 0..string.len() {
       if (string[x] == ';') && (string[x-1] != 'T' && string[x-1] != 'D'){
          println!("Compared {} with T/D", string[x-1]);
           println!("Incorrect direction statement prior the ';'\n");
           return false
       }
       if (string[x] == '=') && !(valid_letter(string[x-1])){
           println!("Button value not recognized in assignment, you provided: {}\n", string[x-1]);
           return false
       }
   }

   true
}

pub fn valid_letter(letter: char) -> bool{
   if letter != 'A' && letter != 'B' && letter != 'C' && letter != 'D'{
      false
   }else{
      true
   }
}

pub fn print_derivations(_string: Vec<char>) -> bool{
   let _string = remove_prefix_and_suffix(_string[0.._string.len()].iter().collect::<String>()); //removes ENTER and EXIT from the string
   let instructions_group: Vec<&str> = _string.split(";").collect(); //splits string if a ";" is found
   if instructions_group.len() == 1{
      
      return true
   }else if instructions_group.len() > 1{
      
      return true
   }else{
      println!("Error reading length of instructions.\n");
      return false
   }
}

pub fn print_parse_tree(_string: Vec<char>){
   print!("parse tree printed\n");
}




