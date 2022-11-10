# Language-Recognizer-in-Rust-for-small-robot

This was a team collaboration for a school project:
Team members: Abel Blanco, Michael Sanchez, Levi Coc, Francis Mejia and myself Tadeo Bennett

View our presentation here: https://cutt.ly/tMyemYW

Program Objectives:
Our objective was to create a BNF grammar meta language for an iZEBOT that will be remotely controlled via infrared light. The program is responsible for accepting an input sentence representing a potential program that will recognize a derivation whether the input is a valid sentence of the meta- language grammar. If the input sentence is valid, the output will display an intermediate code in the form of a PBASIC  program . The compiler will then process and upload the machine code portrayal of the outputted PBASIC program to the iZEBOT through the BASIC stamp editor. Once that is completed, the program will then execute on the robot, and the iZEBOT meta-language will then be programmed to four buttons which are A,B,C, and D. This allows the iZEBOT  to move forward, backwards, left, right, spin left and spin right. The meta language will be structured to “ENTER Button x=y EXIT '' where x is the letter A,B,C or D of the button being programmed and y being movement which are forward, backward, left, right, sleft, and sright.


Program Process Flow:
Step 1).  Display the BNF grammar.
Step 2). Request that a string be inputted according to the BNF grammar. 
Step 3). Verify if the string meets the BNF grammar, if QUIT is entered the program terminates. 
Step 4). If the string meets the BNF requirements, produce a LEFTMOST derivation of the string and display the output. 
Step5). If the string does not meet the BNF requirements, generate an error indicating that an error was found “ preview error” and request for another string. 
Step 6). If the string is valid, draw the PARSE TREE for the derivation and display the tree. 
Step 7). Generate a PBASIC file, display its output and proceed to save to a text file name IZEBOT.BSP.  
Step 8). Once the text file has been saved, loop back to step one and start over if needed. If not proceed to configure the iZEBOT. 
Step 9). Upload the BSP file to the IZEBOT via the BASIC Stamp Editor. 
Step 10).  Ensure that the iZEBOT executes the commands uploaded. 



