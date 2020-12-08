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

fn fix(lines: &mut [Instruction], itf: usize) -> &[Instruction] {
	let instruction_to_fix = itf;
	let res = match lines[instruction_to_fix] {
		Nop(v) => Jmp(v),
		Jmp(v) => Nop(v),
		x => x,
	};
	lines[instruction_to_fix] = res;
	lines
}

#[derive(Clone, Debug, PartialEq)]
struct CPU {
	acc: isize,
	pc: isize,
	visited: Vec<isize>,
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
		let pc = self.pc;
		if self.visited.contains(&pc) {
			return None;
		}
		self.visited.push(pc);
		match instructions.get(pc as usize) {
			Some(Nop(_)) => {}
			Some(Acc(res)) => {
				self.acc += res;
			}
			Some(Jmp(res)) => {
				self.pc += res - 1;
			}
			None => {
				self.end = true;
				return None;
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

	for i in 0..input.len() {
		let mut wip = input.clone();
		let fixed = fix(&mut wip, i);
		let mut cpu = CPU::new();
		while cpu.cycle(&fixed).is_some() {}
		if cpu.end {
			dbg!(cpu.acc);
			break;
		}
	}
}
