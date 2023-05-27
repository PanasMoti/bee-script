# By VilgotanL
import math
import sys
import os

lines = []
try:
	fileName = sys.argv[1]
	file = open(fileName)
	lines = file.read().split("\n")
	file.close()
except Exception as e:
	print(f"Error while opening file:\n{e}")
	sys.exit(0)

beemoviescript = ""
try:
	file = open("bee_movie_script.txt")
	beemoviescript = file.read()
	file.close()
except Exception as e:
	print("Unable to open the bee movie script")
	sys.exit(0)


stack = []
pc = 0

def err(str):
	print("\n" + str + f" at line {pc}")
	sys.exit(0)

def pop(index = -1):
	if len(stack) < 1:
		err("Error: Stack underflow")
	return stack.pop(index)


while pc >= 0 and pc < len(lines):
	parts = lines[pc].split(" ")
	instr = parts[0]
	if instr == "AVIATE":
		index = 0

		if len(parts) < 2:
			err("Error: Expected instruction argument for PUSH")
		try:
			index = int(parts[1])
		except:
			err("Error: Invalid instruction argument for PUSH")

		if(index < 0 or index >= len(beemoviescript)):
			err("Error: Out of range bee movie script character index")
		stack.append(ord(beemoviescript[index]))
	elif instr == "BEE":
		a = pop()
		stack.append(a)
		stack.append(a)
	elif instr == "BLACK":
		print(chr(pop()), end="", flush=True)
	elif instr == "BARRY":
		a = pop()
		b = pop()
		stack.append(b - a)
	elif instr == "FLY":
		a = pop()
		if len(parts) < 2:
			err("Error: Expected instruction argument for FLY")
		try:
			line = int(parts[1]) - 1 # - 1 because list indexes start at 0
			if a != 0:
				pc = line - 1 # - 1 again because we're incrementing pc each instruction
		except:
			err("Error: Invalid instruction argument for FLY")
	elif instr == "ROTATE":
		a = pop(0)
		stack.append(a)
	elif instr == "ROTAT":
		a = pop()
		stack.insert(0, a)
	elif instr == "YELLOW":
		stack.append(ord(input("")[0]))

	pc += 1

