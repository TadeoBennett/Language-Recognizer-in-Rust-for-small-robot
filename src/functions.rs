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
Input String Form: ENTER Button A = FORWARD; Button B = BACKWARD; EXIT
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

   let check_string = remove_prefix_and_suffix(string[0..string.len()].iter().collect::<String>()); //removes the ENTER and EXIT from string
   let instructions_group: Vec<&str> = check_string.split_inclusive(";").collect();                   //create group of instructions sepatated by a semicolon

   for instruction in instructions_group{
      let new_instruction: Vec<char> = instruction.chars().collect(); //convert the instruction to a vector of chars
      
      let index1_for_direction = return_char_index('=', new_instruction.clone());
      let index2_for_direction= return_char_index(';', new_instruction.clone());

      let button_text = new_instruction[0..index1_for_direction.clone()-1].iter().collect::<String>().clone();
      if button_text != "Button"{
         print!("Invalid grammar '{}'; Expected 'Button' in instruction: {}\n\n", button_text, instruction);
         return false;
      }

      let variable = new_instruction[return_char_index('=', new_instruction.clone()) - 1];
      if !valid_letter(variable){
         println!("Invalid variable '{}' in instruction: {}\n", variable, instruction);
         return false;
      }

      let direction = new_instruction[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();
      if !valid_direction(direction.clone()){
         println!("Invalid directional statement '{}' in instruction: {}\n", direction, instruction);
         return false;
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

pub fn valid_direction(direction: String) -> bool{
   if direction != "FORWARD" && direction != "BACKWARD" && direction != "LEFT" && direction != "RIGHT" && direction != "SRIGHT"  && direction != "SLEFT"{
      false
   }else{
      true
   }
}

pub fn remove_prefix_and_suffix(string: String) -> String {
   let mut _string = string.replace("ENTER", "");
   _string = _string.replace("EXIT", "");
   _string //returns the new string without the ENTER AND EXIT text
}

fn return_char_index(char_to_find: char, array: Vec<char>) -> usize {
   let mut count = 0;
   for letter in array{
      if letter == char_to_find{
         return count;
      }
      count += 1;
   }
   return 10
}

pub fn pause() {
   println!("Press any key to continue...");
   io::stdin().read_line(&mut String::new()).unwrap();
}


