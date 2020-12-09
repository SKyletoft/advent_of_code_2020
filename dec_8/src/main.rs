#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
	Nop(isize),
	Jmp(isize),
	Acc(isize),
}

use Instruction::*;

fn parse_instruction(i: &str) -> Instruction {
	let val = i.split_whitespace().nth(1).unwrap().parse().unwrap();
	match i.split_whitespace().next().unwrap() {
		"nop" => Nop(val),
		"jmp" => Jmp(val),
		"acc" => Acc(val),
		_ => panic!(),
	}
}

fn fix(lines: &mut [Instruction], itf: usize) {
	let instruction_to_fix = itf;
	let res = match lines[instruction_to_fix] {
		Nop(v) => Jmp(v),
		Jmp(v) => Nop(v),
		x => x,
	};
	lines[instruction_to_fix] = res;
}

#[derive(Clone, Debug, PartialEq)]
struct CPU {
	acc: isize,
	pc: isize,
	visited: Vec<usize>,
	end: bool,
}

impl CPU {
	fn new() -> Self {
		CPU {
			acc: 0,
			pc: 0,
			visited: Vec::new(),
			end: false,
		}
	}

	fn cycle(&mut self, instructions: &[Instruction]) -> Option<()> {
		if self.end {
			return None;
		}
		let pc = self.pc as usize;
		if pc >= instructions.len() {
			self.end = true;
			return None;
		}
		if self.visited.contains(&pc) {
			return None;
		}
		self.visited.push(pc);
		match instructions[pc] {
			Nop(_) => {}
			Acc(res) => {
				self.acc += res;
			}
			Jmp(res) => {
				self.pc += res - 1;
			}
		}
		self.pc += 1;
		Some(())
	}
}

fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(parse_instruction)
		.collect::<Vec<_>>();
	let mut cpu = CPU::new();
	while cpu.cycle(&input).is_some() {}
	dbg!(cpu.acc);

	let mut wip = input.clone();
	for i in 0..input.len() {
		fix(&mut wip, i);
		let mut cpu = CPU::new();
		while cpu.cycle(&wip).is_some() {}
		if cpu.end {
			dbg!(cpu.acc);
			break;
		}
		fix(&mut wip, i);
	}
}
