use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;

pub fn generate_bsp_file(instructions_group: Vec<&str>, instructions_count: usize) {
   println!("\n************* SHOWING BSP FILE FOR IZEBOT ****************\n");
    let mut file = File::create("IZEBOT.BSP").expect("Error encountered while creating file!");

    let mut izebot: String = "'{$STAMP BS2p}
'{$PBASIC 2.5}
KEY\t\tVAR\t\tByte
Main:\t\tDO
     \t\tSERIN 3,2063,250,Timeout,[KEY]\n"
        .to_owned(); //header code

    for instruction in instructions_group {
        let new_instruction: Vec<char> = instruction.chars().collect(); //convert the instruction to a vector of chars

        let index1_for_direction = return_char_index('=', new_instruction.clone());
        let index2_for_direction = return_char_index(';', new_instruction.clone());

        let variable = new_instruction[return_char_index('=', new_instruction.clone()) - 1];
        let direction = new_instruction[index1_for_direction.clone() + 1..index2_for_direction.clone()].iter().collect::<String>().clone();

        izebot.push_str("IF KEY = \"");
        izebot.push(variable);
        izebot.push_str("\" OR KEY = \"");
        izebot.push_str(&variable.to_lowercase().to_string());
        izebot.push_str("\" THEN GOSUB ");
        izebot.push_str(&direction.to_string());
        izebot = izebot + "\n";
    }
    //IF KEY = “x" OR KEY = “y" THEN GOSUB routine

    izebot.push_str(
        "\n\t\t\tLOOP
Timeout:\tGOSUB Motor_OFF
\t\t\tGOTO Main
'+++++ Movement Procedure ++++++++++++++++++++++++++++++",
    ); //footer 1 code

    izebot.push_str(
        "\nForward: HIGH 13 : LOW 12 : HIGH 15 : LOW 14 : RETURN
Backward: HIGH 12 : LOW 13 : HIGH 14 : LOW 15 : RETURN
TurnLeft: HIGH 13 : LOW 12 : LOW 15 : LOW 14 : RETURN
TurnRight: LOW 13 : LOW 12 : HIGH 15 : LOW 14 : RETURN
SpinLeft: HIGH 13 : LOW 12 : HIGH 14 : LOW 15 : RETURN
SpinRight: HIGH 12 : LOW 13 : HIGH 15 : LOW 14 : RETURN",
    ); //subroutine code

    izebot.push_str(
        "\nMotor_OFF: LOW 13 : LOW 12 : LOW 15 : LOW 14 : RETURN
'+++++++++++++++++++++++++++++++++++++++++++++++++++++++",
    ); //footer 2 code

    write!(file, "{}", izebot).expect("Error encountered while writing to file!");

    print!("{}", izebot);
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