use Rule::*;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> usize {
	let rules = parse_rules(input);
	input
		.iter()
		.skip(rules.len())
		.filter(|l| !l.is_empty())
		.map(|line| rules[0].match_str(&rules, line))
		.filter(|r| *r == Ok(""))
		.count()
}

fn solve2(input: &[&str]) -> usize {
	let mut rules = parse_rules(input);
	rules[8] = EitherTwo((131, 42), (42, 8));
	rules[11] = EitherThree((131, 42, 31), (42, 11, 31));
	input
		.iter()
		.skip(rules.len())
		.filter(|l| !l.is_empty())
		.map(|line| rules[0].match_str(&rules, line))
		.filter(|r| *r == Ok(""))
		.count()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rule {
	Anything,
	Nothing,
	Token(char),
	One(usize),
	Two(usize, usize),
	Three(usize, usize, usize),
	EitherOne(usize, usize),
	EitherTwo((usize, usize), (usize, usize)),
	EitherThree((usize, usize, usize), (usize, usize, usize)),
}

impl Rule {
	fn match_str<'a>(&self, rules: &[Rule], s: &'a str) -> Result<&'a str, &'a str> {
		if s.is_empty() {
			return Ok(s);
		}
		match self {
			Anything => Ok(s),
			Token(c) => {
				let res = match s.chars().next() {
					Some(sc) if sc == *c => Ok(&s[1..]),
					_ => Err(s),
				};
				res
			}
			One(r1) => rules[*r1].match_str(rules, s),
			Two(r1, r2) => rules[*r1]
				.match_str(rules, s)
				.and_then(|n_s| rules[*r2].match_str(rules, n_s)),
			Three(r1, r2, r3) => rules[*r1]
				.match_str(rules, s)
				.and_then(|n_s| rules[*r2].match_str(rules, n_s))
				.and_then(|nn_s| rules[*r3].match_str(rules, nn_s)),
			EitherOne(r1, r2) => rules[*r1]
				.match_str(rules, s)
				.or_else(|n_s| rules[*r2].match_str(rules, n_s)),
			EitherTwo((r11, r12), (r21, r22)) => Two(*r11, *r12)
				.match_str(rules, s)
				.or_else(|_| Two(*r21, *r22).match_str(rules, s)),
			EitherThree((r11, r12, r13), (r21, r22, r23)) => Three(*r11, *r12, *r13)
				.match_str(rules, s)
				.or_else(|_| Three(*r21, *r22, *r23).match_str(rules, s)),
			Nothing => {
				eprintln!("HIT NOTHING RULE");
				Err(s)
			}
		}
	}
}

fn parse_rules(input: &[&str]) -> Vec<Rule> {
	let mut rules = vec![Nothing; 132];
	for line in input.iter().take(rules.len() - 1) {
		dbg!(line);
		let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();
		let idx = tokens[0][..tokens[0].len() - 1].parse::<usize>().unwrap();
		let rule = match tokens.len() {
			2 => match tokens[1].parse::<usize>() {
				Ok(n) => One(n),
				Err(_) => Token(tokens[1].chars().nth(1).unwrap()),
			},
			3 => Two(
				tokens[1].parse::<usize>().unwrap(),
				tokens[2].parse::<usize>().unwrap(),
			),
			4 => match tokens[2].parse::<usize>() {
				Ok(n) => Three(
					tokens[1].parse::<usize>().unwrap(),
					n,
					tokens[3].parse::<usize>().unwrap(),
				),
				Err(_) => EitherOne(
					tokens[1].parse::<usize>().unwrap(),
					tokens[3].parse::<usize>().unwrap(),
				),
			},
			6 => {
				assert_eq!(tokens[3], "|");
				EitherTwo(
					(
						tokens[1].parse::<usize>().unwrap(),
						tokens[2].parse::<usize>().unwrap(),
					),
					(
						tokens[4].parse::<usize>().unwrap(),
						tokens[5].parse::<usize>().unwrap(),
					),
				)
			}
			_ => panic!(),
		};
		rules[idx] = rule;
	}
	let last = rules.len() - 1;
	rules[last] = Anything;
	rules
}
