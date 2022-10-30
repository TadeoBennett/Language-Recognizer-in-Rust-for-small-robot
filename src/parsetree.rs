pub fn print_parsetree(instructions_group: Vec<&str>, instructions_count: usize) -> bool {
   println!("\n************* SHOWING PARSE TREE ****************\n");
   if instructions_count == 1{ //there is only 1 instruction
      let instruction: Vec<char> = instructions_group[0].chars().collect();
      let index1_for_direction:usize = return_char_index('=', instruction.clone());
      let index2_for_direction:usize= return_char_index(';', instruction.clone());

      println!("               <program>        \n");
      println!("           /       |       \\              \n   ");
      println!("         /         |         \\            \n     ");
      println!("       /           |           \\          \n     ");
      println!("     ENTER    <instructions>     EXIT      \n        ");
      println!("             /      |       \\             \n          ");
      println!("           /        |         \\           \n       ");
      println!("         /          |           \\         \n     ");
      println!("    Button        <key>    =   <direction> \n         ");
      println!("      |             |              |       \n    ");
      println!("      |             |              |         \n   ");
      println!("    Button         {}              {}    \n", instruction[6], instruction[index1_for_direction+1..index2_for_direction].iter().collect::<String>());
      
      return true;
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

      print!("                                      <program>        \n");
      print!("                                   /      |      \\           \n");
      print!("                                /         |        \\           \n");
      print!("                              /           |          \\           \n");
      print!("                         ENTER      <instructions>     EXIT       \n");
      print!("                                  /               \\             \n");
      print!("                                /                   \\            \n");
      print!("                          <instruction>   ;    <instructions>      \n");
      print!("                        /                             |        \n");
      print!("                      /                         <instruction>      \n");
      print!("                    /                                        \\         \n");
      print!("          Button <key> = <direction>      ;            Button <key> = <direction> \n");
      print!("          |        |          |                           |     |          |       \n");
      print!("          |        |          |                           |     |          |       \n");
      print!("       Button      {}        {}                    Button   {}      {}       \n", instruction[6], direction1, instruction2[6], direction2);
      return true;
   }else if instructions_count == 3{
      let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
      let instruction2: Vec<char> = instructions_group[1].chars().collect(); 
      let instruction3: Vec<char> = instructions_group[2].chars().collect(); 
      
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
      
      print!("                                             <program>                                                                               \n");
      print!("                                          /      |      \\                                                                           \n");
      print!("                                       /         |        \\                                                                         \n");
      print!("                                     /           |          \\                                                                       \n");
      print!("                                ENTER      <instructions>     EXIT                                                                   \n");
      print!("                                         /               \\                                                                          \n");
      print!("                                       /                   \\                                                                        \n");
      print!("                                 <instruction>     ;     <instructions>                                                              \n");
      print!("                              /                                  |        \\                                                         \n");   
      print!("                          /                                      |            \\                                                     \n");   
      print!("                 Button <key> = <direction>        ;     <instructions>                                       \n");   
      print!("                          |          |                 /               \\                                   \n");   
      print!("                          |          |               /                    \\                                \n");
      print!("                          |          |         <instruction>     ;       <instructions>                      \n");
      print!("                          |          |             |                           |                           \n");
      print!("                          |          |   Button <key> = <direction>  ;    <instruction>                  \n");  
      print!("                          |          |            |          |                |                         \n");         
      print!("                          |          |            |          |       <Button <key> = <direction> ;       \n");    
      print!("                          |          |            |          |                |           |             \n");         
      print!("                         {}         {}          {}         {}               {}          {}                     \n", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3);         
      return true;

      }else if instructions_count == 4{
      let instruction: Vec<char> = instructions_group[0].chars().collect(); //convert the only instruction to a vector of chars
      let instruction2: Vec<char> = instructions_group[1].chars().collect(); 
      let instruction3: Vec<char> = instructions_group[2].chars().collect(); 
      let instruction4: Vec<char> = instructions_group[3].chars().collect(); 
      
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

      print!("                                                         <program>                                                                               \n");
      print!("                                                      /      |      \\                                                                           \n");
      print!("                                                   /         |        \\                                                                         \n");
      print!("                                                 /           |          \\                                                                       \n");
      print!("                                            ENTER      <instructions>     EXIT                                                                   \n");
      print!("                                                     /               \\                                                                          \n");
      print!("                                                   /                   \\                                                                        \n");
      print!("                                             <instruction>     ;     <instructions>                                                              \n");
      print!("                                          /                                  |        \\                                                         \n");   
      print!("                                      /                                      |            \\                                                     \n");   
      print!("                             Button <key> = <direction>        ;      <instruction>   ;     <instructions>                                       \n");   
      print!("                                      |          |                     /                    /               \\                                   \n");   
      print!("                                      |          |                   /                    /                    \\                                \n");
      print!("                                      |          |                /                 <instruction>     ;      <instructions>                      \n");
      print!("                                      |          |             /                          |                          |                           \n");
      print!("                                      |          |  Button <key> = <direction>  ; Button <key> = <direction>  ;   <instruction>                  \n");  
      print!("                                      |          |           |           |                 |          |                |                         \n");         
      print!("                                      |          |           |           |                 |          |      <Button <key> = <direction> ;       \n");    
      print!("                                      |          |           |           |                 |          |                |           |             \n");         
      print!("                                      {}         {}       {}          {}              {}         {}           {}          {}                     \n", instruction[6], direction1, instruction2[6], direction2, instruction3[6], direction3, instruction4[6], direction4);        

      return true;
   }
   true
}



//RETURN the index position of where a certain character is in a given vector of chars
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