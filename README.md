# bee-script
this is a rust interperter for bee script
https://esolangs.org/wiki/BeeScript


AVIATE n	Pushes the ascii code of the nth character in this version of the bee movie script
BEE	        Duplicates the top value of the stack
BLACK	    Pops the top value of the stack and prints it as an ascii character
BARRY	    Pops A and then B from the stack and pushes B - A to the stack
FLY n	    Pops the top of the stack, if it is non-zero this instruction jumps to the nth line
ROTATE	    Removes the bottom value of the stack and pushes it to the top
ROTAT	    Pops the top of the stack and pushes it to the bottom
YELLOW	    Takes one character of input and pushes it's ascii code to the stack
