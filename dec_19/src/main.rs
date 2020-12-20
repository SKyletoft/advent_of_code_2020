use Rule::*;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let rules = parse_rules(&input);
	let sol1 = solve1(&input, &rules);
	let sol2 = solve2(&input, rules);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str], rules: &[Rule]) -> usize {
	let rules = {
		let mut r = rules.to_vec();
		optimise_rules(&mut r);
		r
	};
	input
		.iter()
		.skip_while(|s| s.chars().next().map(|c| c.is_ascii_digit()) == Some(true))
		.filter(|l| !l.is_empty())
		.map(|line| rules[0].match_str(&rules, line))
		.filter(|r| *r == Ok(""))
		.count()
}

fn solve2(input: &[&str], mut rules: Vec<Rule>) -> usize {
	rules[8] = Either(Box::new(One(42)), Box::new(Two(42, 8)));
	rules[11] = Either(Box::new(Two(42, 31)), Box::new(Three(42, 11, 31)));
	optimise_rules(&mut rules);

	input
		.iter()
		.skip_while(|s| s.chars().next().map(|c| c.is_ascii_digit()) == Some(true))
		.filter(|l| !l.is_empty())
		.map(|line| rules[0].match_str_iter(&rules, line))
		.filter_map(|r| r.ok())
		.filter(|r| r.contains(&""))
		.count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
	Nothing,
	Token(char),
	Tokens(Vec<char>),
	One(usize),
	Two(usize, usize),
	Three(usize, usize, usize),
	Either(Box<Rule>, Box<Rule>),
}

impl Rule {
	fn match_str<'a>(&self, rules: &[Rule], s: &'a str) -> Result<&'a str, &'a str> {
		if s.is_empty() {
			return Err(s);
		}
		match self {
			Token(c) => match s.chars().next() {
				Some(sc) if sc == *c => Ok(&s[1..]),
				_ => Err(s),
			},
			Tokens(v) => {
				if s.chars().zip(v.iter()).all(|(a, &b)| a == b) {
					Ok(&s[v.len()..])
				} else {
					Err(s)
				}
			}
			One(r1) => rules[*r1].match_str(rules, s),
			Two(r1, r2) => rules[*r1]
				.match_str(rules, s)
				.and_then(|n_s| rules[*r2].match_str(rules, n_s)),
			Three(r1, r2, r3) => rules[*r1]
				.match_str(rules, s)
				.and_then(|n_s| rules[*r2].match_str(rules, n_s))
				.and_then(|nn_s| rules[*r3].match_str(rules, nn_s)),
			Either(r1, r2) => r1.match_str(rules, s).or_else(|_| r2.match_str(rules, s)),
			Nothing => {
				dbg!(self);
				panic!()
			}
		}
	}

	fn match_str_iter<'a>(&self, rules: &[Rule], s: &'a str) -> Result<Vec<&'a str>, &'a str> {
		if s.is_empty() {
			return Err(s);
		}
		match self {
			Token(c) => match s.chars().next() {
				Some(sc) if sc == *c => Ok(vec![&s[1..]]),
				_ => Err(s),
			},
			Tokens(v) => {
				if s.chars().zip(v.iter()).all(|(a, &b)| a == b) {
					Ok(vec![&s[v.len()..]])
				} else {
					Err(s)
				}
			}
			One(r1) => rules[*r1].match_str_iter(rules, s),
			Two(r1, r2) => rules[*r1].match_str_iter(rules, s).map(|v| {
				v.iter()
					.filter_map(|n_s| rules[*r2].match_str_iter(rules, n_s).ok())
					.flat_map(move |r| r.into_iter())
					.collect()
			}),
			Three(r1, r2, r3) => rules[*r1].match_str_iter(rules, s).map(|v| {
				v.iter()
					.filter_map(|n_s| rules[*r2].match_str_iter(rules, n_s).ok())
					.flat_map(move |r| r.into_iter())
					.filter_map(|nn_s| rules[*r3].match_str_iter(rules, nn_s).ok())
					.flat_map(move |r| r.into_iter())
					.collect()
			}),
			Either(r1, r2) => match (r1.match_str_iter(rules, s), r2.match_str_iter(rules, s)) {
				(a, Err(_)) => a,
				(Err(_), b) => b,
				(Ok(mut s1), Ok(mut s2)) => {
					s1.append(&mut s2);
					Ok(s1)
				}
			},
			Nothing => {
				dbg!(self);
				panic!()
			}
		}
	}

	fn optimise_rule(&mut self, rules: &[Rule]) {
		match self {
			Either(r1, r2) => {
				r1.optimise_rule(rules);
				r2.optimise_rule(rules);
				if r1 == r2 {
					*self = *r1.clone();
				} else {
					*self = Either(Box::new(*r1.clone()), Box::new(*r2.clone()));
				}
			}
			One(idx) => *self = rules[*idx].clone(),
			Two(idx, jdx) => match (&rules[*idx], &rules[*jdx]) {
				(Token(a), Token(b)) => *self = Tokens(vec![*a, *b]),
				(Token(a), Tokens(v)) => {
					let mut clone = v.clone();
					clone.insert(0, *a);
					*self = Tokens(clone);
				}
				(Tokens(v), Token(b)) => {
					let mut clone = v.clone();
					clone.push(*b);
					*self = Tokens(clone);
				}
				(Tokens(a), Tokens(b)) => {
					let mut clone = a.clone();
					clone.append(&mut b.clone());
					*self = Tokens(clone);
				}
				(Two(a, b), One(c)) => *self = Three(*a, *b, *c),
				(One(a), Two(b, c)) => *self = Three(*a, *b, *c),
				(One(a), One(b)) => *self = Two(*a, *b),
				_ => {}
			},
			Three(idx, jdx, kdx) => match (&rules[*idx], &rules[*jdx], &rules[*kdx]) {
				(Token(a), Token(b), Token(c)) => *self = Tokens(vec![*a, *b, *c]),
				(Tokens(a), Token(b), Token(c)) => {
					let mut clone = a.clone();
					clone.push(*b);
					clone.push(*c);
					*self = Tokens(clone);
				}
				(Token(a), Tokens(b), Token(c)) => {
					let mut clone = b.clone();
					clone.insert(0, *a);
					clone.push(*c);
					*self = Tokens(clone);
				}
				(Token(a), Token(b), Tokens(c)) => {
					let mut clone = c.clone();
					clone.insert(0, *b);
					clone.insert(0, *a);
					*self = Tokens(clone);
				}
				(Tokens(a), Tokens(b), Token(c)) => {
					let mut clone = a.clone();
					clone.append(&mut b.clone());
					clone.push(*c);
					*self = Tokens(clone);
				}
				(Tokens(a), Token(b), Tokens(c)) => {
					let mut clone = a.clone();
					clone.push(*b);
					clone.append(&mut c.clone());
					*self = Tokens(clone);
				}
				(Token(a), Tokens(b), Tokens(c)) => {
					let mut clone = b.clone();
					clone.insert(0, *a);
					clone.append(&mut c.clone());
					*self = Tokens(clone);
				}
				(Tokens(a), Tokens(b), Tokens(c)) => {
					let mut clone = a.clone();
					clone.append(&mut b.clone());
					clone.append(&mut c.clone());
					*self = Tokens(clone);
				}
				_ => {}
			},
			_ => {}
		}
	}
}

fn optimise_rules(rules: &mut Vec<Rule>) {
	loop {
		let mut made_changes = false;
		for i in 0..rules.len() {
			let mut copy = rules[i].clone();
			copy.optimise_rule(rules);
			if rules[i] != copy {
				made_changes = true;
			}
			rules[i] = copy;
		}
		if !made_changes {
			return;
		}
	}
}

fn parse_rules(input: &[&str]) -> Vec<Rule> {
	let rule_count = input
		.iter()
		.take_while(|s| s.chars().next().map(|c| c.is_ascii_digit()) == Some(true))
		.count();
	let mut rules = Vec::with_capacity(input.len());
	for line in input.iter().take(rule_count) {
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
				Err(_) => Either(
					Box::new(One(tokens[1].parse::<usize>().unwrap())),
					Box::new(One(tokens[3].parse::<usize>().unwrap())),
				),
			},
			6 => {
				assert_eq!(tokens[3], "|");
				Either(
					Box::new(Two(
						tokens[1].parse::<usize>().unwrap(),
						tokens[2].parse::<usize>().unwrap(),
					)),
					Box::new(Two(
						tokens[4].parse::<usize>().unwrap(),
						tokens[5].parse::<usize>().unwrap(),
					)),
				)
			}
			_ => panic!(),
		};
		while rules.len() <= idx {
			rules.push(Nothing);
		}
		rules[idx] = rule;
	}
	rules
}
