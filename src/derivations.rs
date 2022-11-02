

pub fn print_derivations(instructions_group: Vec<&str>, instructions_count: usize) -> bool {
    println!("\n************* SHOWING DERIVATION ****************\n");
    if instructions_count == 1{ //there is only 1 instruction
        let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
        println!("ENTER <instructions> EXIT");
        println!("ENTER <instruction>; EXIT");
        println!("ENTER Button <key> = <direction>; EXIT");
        println!("ENTER Button {} = <direction>; EXIT", instruction[6]);
        let index1_for_direction:usize = return_char_index('=', instruction.clone());
        let index2_for_direction:usize= return_char_index(';', instruction.clone());
        println!("ENTER Button {} = {}; EXIT", instruction[6], instruction[index1_for_direction+1..index2_for_direction].iter().collect::<String>());
    }else if instructions_count == 2{
        let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
        let instruction2: Vec<char> = instructions_group[1].chars().collect(); //convert the only instruction to a vector of 
        
        let mut index1_for_direction:usize;
        let mut index2_for_direction:usize;

        index1_for_direction = return_char_index('=', instruction.clone());
        index2_for_direction= return_char_index(';', instruction.clone());
        let direction1 = instruction[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        index1_for_direction = return_char_index('=', instruction2.clone());
        index2_for_direction= return_char_index(';', instruction2.clone());
        let direction2 = instruction2[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        println!("ENTER <instructions> EXIT");
        println!("ENTER <instruction>; <instructions> EXIT");
        println!("ENTER Button <key> = <direction>; <instructions> EXIT");
        println!("ENTER Button {} = <direction>; <instructions> EXIT", instruction[6]);
        println!("ENTER Button {} = {}; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; <instruction> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button <key> = <direction>; EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button {} = <direction>; EXIT", instruction[6], direction1, instruction2[6]);
        println!("ENTER Button {} = {}; Button {} = {}; EXIT", instruction[6], direction1, instruction2[6], direction2);
    }else if instructions_count == 3{
        let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
        let instruction2: Vec<char> = instructions_group[1].chars().collect(); //convert the only instruction to a vector of 
        let instruction3: Vec<char> = instructions_group[2].chars().collect(); //convert the only instruction to a vector of 
        
        let mut index1_for_direction:usize;
        let mut index2_for_direction:usize;

        index1_for_direction = return_char_index('=', instruction.clone());
        index2_for_direction= return_char_index(';', instruction.clone());
        let direction1 = instruction[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        index1_for_direction = return_char_index('=', instruction2.clone());
        index2_for_direction= return_char_index(';', instruction2.clone());
        let direction2 = instruction2[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();
        
        index1_for_direction = return_char_index('=', instruction3.clone());
        index2_for_direction= return_char_index(';', instruction3.clone());
        let direction3 = instruction3[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        println!("ENTER <instructions> EXIT");
        println!("ENTER <instruction>; <instructions> EXIT");
        println!("ENTER Button <key> = <direction>; <instructions> EXIT");
        println!("ENTER Button {} = <direction>; <instructions> EXIT", instruction[6]);
        println!("ENTER Button {} = {}; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; <instruction>; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button <key> = <direction>; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button {} = <direction>; <instructions> EXIT", instruction[6], direction1, instruction2[6]);
        println!("ENTER Button {} = {}; Button {} = {}; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2);

        println!("ENTER Button {} = {}; Button {} = {}; <instruction> EXIT", instruction[6], direction1, instruction2[6], direction2);
        println!("ENTER Button {} = {}; Button {} = {}; Button <key> = <direction>; EXIT", instruction[6], direction1, instruction2[6], direction2);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = <direction>; EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6]);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3);

    }else if instructions_count == 4{
        let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
        let instruction2: Vec<char> = instructions_group[1].chars().collect(); //convert the only instruction to a vector of 
        let instruction3: Vec<char> = instructions_group[2].chars().collect(); //convert the only instruction to a vector of 
        let instruction4: Vec<char> = instructions_group[3].chars().collect(); //convert the only instruction to a vector of 
        
        let mut index1_for_direction:usize;
        let mut index2_for_direction:usize;

        index1_for_direction = return_char_index('=', instruction.clone());
        index2_for_direction= return_char_index(';', instruction.clone());
        let direction1 = instruction[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        index1_for_direction = return_char_index('=', instruction2.clone());
        index2_for_direction= return_char_index(';', instruction2.clone());
        let direction2 = instruction2[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();
        
        index1_for_direction = return_char_index('=', instruction3.clone());
        index2_for_direction= return_char_index(';', instruction3.clone());
        let direction3 = instruction3[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();
        
        index1_for_direction = return_char_index('=', instruction4.clone());
        index2_for_direction= return_char_index(';', instruction4.clone());
        let direction4 = instruction4[index1_for_direction.clone()+1..index2_for_direction.clone()].iter().collect::<String>().clone();

        println!("ENTER <instructions> EXIT");
        println!("ENTER <instruction>; <instructions> EXIT");
        println!("ENTER Button <key> = <direction>; <instructions> EXIT");
        println!("ENTER Button {} = <direction>; <instructions> EXIT", instruction[6]);
        println!("ENTER Button {} = {}; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; <instruction>; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button <key> = <direction>; <instructions> EXIT", instruction[6], direction1);
        println!("ENTER Button {} = {}; Button {} = <direction>; <instructions> EXIT", instruction[6], direction1, instruction2[6]);
        println!("ENTER Button {} = {}; Button {} = {}; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2);
        println!("ENTER Button {} = {}; Button {} = {}; <instruction>; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2);
        println!("ENTER Button {} = {}; Button {} = {}; Button <key> = <direction>; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = <direction>; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6]);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; <instructions> EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3);
        
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; <instruction> EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; Button <key> = <direction>; EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; Button {} = <direction>; EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3, instruction4[6]);
        println!("ENTER Button {} = {}; Button {} = {}; Button {} = {}; Button {} = {}; EXIT", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3, instruction4[6], direction4);
    }

    true
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
