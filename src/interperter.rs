use std::fmt::{Display, self};
use std::io::Read;
use std::mem::swap;
use std::str::FromStr;

use crate::stack::*;


const BEEMOVIESCRIPTFILE:&'static str = "static/bee_movie_script.txt";


#[derive(Debug, PartialEq,Clone, Copy)]
pub enum Commands {
    AVIATE(usize),
    BEE,
    BLACK,
    BARRY,
    FLY(usize),
    ROTATE,
    ROTAT,
    YELLOW
}
impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::AVIATE(n) => write!(f, "AVIATE {}",n),
            Commands::BARRY => write!(f,"BARRY"),
            Commands::BEE => write!(f,"BEE"),
            Commands::BLACK => write!(f,"BLACK"),
            Commands::FLY(n) => write!(f,"FLY {}",n),
            Commands::ROTATE => write!(f,"ROTATE"),
            Commands::ROTAT => write!(f,"ROTAT"),
            Commands::YELLOW => write!(f,"YELLOW"),
           
        }
    }
}


#[derive(Debug)]
pub struct Interperter {
    bee_script: String,
    stack : Stack<u8>,
    instructions: Vec<Commands>,
    current_instruction: usize

}

impl Interperter {
    pub fn new() -> Self {
        Self {
            bee_script: std::fs::read_to_string(BEEMOVIESCRIPTFILE).
            unwrap(),
            stack : Stack::new(),
            instructions: Vec::new(),
            current_instruction: 0
        }
    }

    fn run_cmd(&mut self,cmd:Commands) {
        match cmd {
            Commands::AVIATE(n) => {
                // AVIATE n	Pushes the ascii code of the nth character in this version of the bee movie script
                
                self.stack.push(self.bee_script.as_str().as_bytes()[n]);
                self.current_instruction += 1;
            },
            Commands::BEE => {
                //BEE	        Duplicates the top value of the stack
                if let Some(value) = self.stack.peek() {
                    self.stack.push(*value);
                    self.current_instruction += 1;
                }
            },
            Commands::BARRY => {
                //BARRY	    Pops A and then B from the stack and pushes B - A to the stack

                let a = self.stack.pop().unwrap();
                let b: u8 = self.stack.pop().unwrap();
                self.stack.push(b);
                self.stack.push(a);
                self.current_instruction += 1;
            },
            Commands::BLACK => {
                //BLACK	    Pops the top value of the stack and prints it as an ascii character
                if let Some(ch) = self.stack.pop() {
                    print!("{}",ch as char);
                    self.current_instruction += 1;    
                }
            },
            Commands::FLY(n) => {
                //FLY n	    Pops the top of the stack, if it is non-zero this instruction jumps to the nth line
                // AH FUCK A GO TO
                if let Some(v) = self.stack.pop() {
                    if v != 0 {
                        self.current_instruction = n-1;
                    } else {
                        self.current_instruction += 1;
                    }  
                } 
            },
            Commands::ROTAT => {
                //ROTAT	    Pops the top of the stack and pushes it to the bottom
                let p = self.stack.pop().unwrap();
                self.stack.push(p);
                self.current_instruction += 1;
            },
            Commands::ROTATE => {
                //ROTATE	    Removes the bottom value of the stack and pushes it to the top
                let bottom = *self.stack.last().unwrap();
                self.stack.remove_first();
                self.stack.push(bottom);
                self.current_instruction += 1;
            },
            Commands::YELLOW => {
                //YELLOW	    Takes one character of input and pushes it's ascii code to the stack
                self.stack.push(std::io::stdin().bytes().next().unwrap().unwrap());
                self.current_instruction += 1;
            },
            
        }
    }

    fn exec_instructions(&mut self) {
       
        let len = self.instructions.len();
        while self.current_instruction < len {
            self.run_cmd(self.instructions[self.current_instruction]);
        }
    }

    pub fn exec(&mut self,code:String) {
        let lines = code.split('\n').collect::<Vec<&str>>().iter().map(|&s| s.into()).collect::<Vec<String>>();
        for line in lines {
            let bline = line.split(' ').collect::<Vec<&str>>();
            let cmd:Option<Commands> = match bline[0] {
                "AVIATE" => Some(Commands::AVIATE(bline[1].parse::<usize>().expect("AVIATE MUST BE WITH A NUMBER"))),
                "BARRY" => Some(Commands::BARRY),
                "BEE" => Some(Commands::BEE),
                "BLACK" => Some(Commands::BLACK),
                "FLY" => Some(Commands::FLY(bline[1].parse::<usize>().expect("FLY MUST BE WITH A NUMBER AFTER IT"))),
                "ROTATE" => Some(Commands::ROTATE),
                "ROTAT" => Some(Commands::ROTAT),
                "YELLOW" => Some(Commands::YELLOW),
                _ => None,
            };
            if let Some(op) = cmd {
                self.instructions.push(op);
            }
        }
        self.exec_instructions();   
    }
}