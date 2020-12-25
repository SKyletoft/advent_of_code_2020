fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> u64 {
	input.iter().map(eval).sum::<u64>()
}

fn solve2(input: &[&str]) -> u64 {
	input
		.iter()
		.map(|l| order_of_operations_parse(&split(l)))
		.sum::<u64>()
}

fn eval(s: &&str) -> u64 {
	let tokens = split(s);
	assert!(!tokens.is_empty());
	let first = tokens[0];
	if tokens.len() == 1 {
		if has_parentheses(first) {
			return eval(&remove_parentheses(first));
		} else {
			let res = first.parse().unwrap();
			return res;
		}
	}
	//dbg!(first);
	let res = tokens
		.iter()
		.skip(1)
		.step_by(2)
		.zip(tokens.iter().step_by(2).skip(1))
		.fold(eval(&first), |acc, (op, curr)| {
			let parsed = eval(curr);
			match *op {
				"+" => acc + parsed,
				"*" => acc * parsed,
				_ => panic!(),
			}
		});
	res
}

fn split(s: &'_ str) -> Vec<&'_ str> {
	let keep_closure = |slice: &str| slice.chars().any(|c| !c.is_whitespace());
	let mut vec = Vec::new();
	let mut parentheses = 0;
	let mut start = 0;
	for (i, c) in s.char_indices() {
		match (parentheses, c) {
			(0, '(') => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i;
				parentheses += 1;
			}
			(_, '(') => {
				parentheses += 1;
			}
			(1, ')') => {
				let slice = &s[start..=i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
				parentheses -= 1;
			}
			(_, ')') => {
				parentheses -= 1;
			}

			(0, _) if c.is_whitespace() => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
			}
			_ => {}
		}
	}
	let slice = &s[start..];
	if keep_closure(slice) {
		vec.push(slice);
	}
	if parentheses == 0 {
		vec
	} else {
		panic!()
	}
}

pub fn remove_parentheses(s: &'_ str) -> &'_ str {
	let b = s.as_bytes();
	let l = s.len();
	let last = l.wrapping_sub(1);
	if b.get(0) == Some(&b'(') && b.get(last) == Some(&b')') {
		let l = s.len();
		&s[1..l - 1]
	} else {
		s
	}
}

fn has_parentheses(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	let last = l.wrapping_sub(1);
	b.get(0) == Some(&b'(') && b.get(last) == Some(&b')')
}

#[derive(Clone, Debug, PartialEq)]
enum Op<'a> {
	Add(Box<Op<'a>>, Box<Op<'a>>),
	Mul(Box<Op<'a>>, Box<Op<'a>>),
	Val(u64),
	Unparsed(&'a str),
}
use Op::*;
type OpFnPtr<'a> = fn(Box<Op<'a>>, Box<Op<'a>>) -> Op<'a>;

fn evaluate_float(num: &str) -> u64 {
	if !num.bytes().all(|x| x.is_ascii_digit()) {
		panic!()
	}
	parse_int(num)
}

fn parse_int(string: &str) -> u64 {
	string
		.bytes()
		.filter(u8::is_ascii_digit)
		.fold(0, |acc, curr| acc * 10 + (curr - b'0') as u64)
}

fn get_left_and_right<'a>(idx: &mut usize, words: &mut Vec<Op<'a>>) -> (Op<'a>, Op<'a>) {
	if *idx == 0 {
		panic!()
	}
	let left = match words.remove(*idx - 1) {
		Unparsed(s) => Val(parse_or_get(s)),
		x => x,
	};
	*idx -= 1;
	let right = match words.remove(*idx + 1) {
		Unparsed(s) => Val(parse_or_get(s)),
		x => x,
	};
	(left, right)
}

pub fn parse_or_get(s: &str) -> u64 {
	if has_parentheses(s) {
		order_of_operations_parse(&split(remove_parentheses(s)))
	} else {
		evaluate_float(s)
	}
}

fn eval_op(op: Op) -> u64 {
	match op {
		Add(l, r) => eval_op(*l) + eval_op(*r),
		Mul(l, r) => eval_op(*l) * eval_op(*r),
		Val(x) => x,
		Unparsed(s) => parse_or_get(s),
	}
}

fn perform_all_of_operation<'a>(
	words: &mut Vec<Op<'a>>,
	operator: &str,
	operation_function: OpFnPtr<'a>,
) {
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed(operator)) {
		let (left, right) = get_left_and_right(&mut idx, words);
		words[idx] = operation_function(Box::new(left), Box::new(right));
	}
}

fn order_of_operations_parse(words: &[&str]) -> u64 {
	let mut words: Vec<Op> = words.iter().map(|x| Unparsed(x)).collect();

	let operator_fn_pair: [(&str, OpFnPtr); 2] = [
		("+", |lhs, rhs| Add(lhs, rhs)),
		("*", |lhs, rhs| Mul(lhs, rhs)),
	];
	for (operator, node_type) in operator_fn_pair.iter() {
		perform_all_of_operation(&mut words, operator, *node_type);
	}

	if words.len() != 1 {
		panic!()
	}

	eval_op(words.remove(0))
}
