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
		.map(|line| (line, rules[0].match_str_iter(&rules, line)))
		.filter(|(_, r)| r.is_ok())
		.map(|(i, r)| (i, r.unwrap()))
		.filter(|(_, r)| r.contains(&""))
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
			Two(r1, r2) => rules[*r1].match_str_iter(rules, s).and_then(|v| {
				let new_res = v
					.iter()
					.map(|n_s| rules[*r2].match_str_iter(rules, n_s))
					.collect::<Vec<_>>();
				if new_res.iter().all(|r| r.is_err()) {
					return Err(s);
				}
				Ok(new_res
					.into_iter()
					.filter(Result::is_ok)
					.map(move |r| r.unwrap().into_iter())
					.flatten()
					.collect())
			}),
			Three(r1, r2, r3) => rules[*r1].match_str_iter(rules, s).and_then(|v| {
				let new_res = v
					.iter()
					.map(|n_s| rules[*r2].match_str_iter(rules, n_s))
					.collect::<Vec<_>>();
				if new_res.iter().all(|r| r.is_err()) {
					return Err(s);
				}
				let new_new_res = new_res
					.into_iter()
					.filter(Result::is_ok)
					.map(move |r| r.unwrap().into_iter())
					.flatten()
					.map(|nn_s| rules[*r3].match_str_iter(rules, nn_s))
					.collect::<Vec<_>>();
				if new_new_res.iter().all(|r| r.is_err()) {
					return Err(s);
				}
				Ok(new_new_res
					.into_iter()
					.filter(Result::is_ok)
					.map(move |r| r.unwrap().into_iter())
					.flatten()
					.collect())
			}),
			Either(r1, r2) => {
				let s1 = r1.match_str_iter(rules, s);
				let s2 = r2.match_str_iter(rules, s);
				if s2.is_err() {
					s1
				} else if s1.is_err() {
					s2
				} else if let (Ok(mut s1), Ok(mut s2)) = (s1, s2) {
					s1.append(&mut s2);
					Ok(s1)
				} else {
					unreachable!()
				}
			}
			Nothing => {
				dbg!(self);
				panic!()
			}
		}
	}
}

fn optimise_rules(rules: &mut Vec<Rule>) {
	loop {
		let mut made_changes = false;
		for i in 0..rules.len() {
			let copy = rules[i].clone();
			let optimised = optimise_rule(copy, rules);
			if rules[i] != optimised {
				made_changes = true;
			}
			rules[i] = optimised;
		}
		if !made_changes {
			break;
		}
	}
}

fn optimise_rule(rule: Rule, rules: &[Rule]) -> Rule {
	match rule {
		Either(r1, r2) => {
			let o1 = optimise_rule(*r1, rules);
			let o2 = optimise_rule(*r2, rules);
			if o1 == o2 {
				o1
			} else {
				Either(Box::new(o1), Box::new(o2))
			}
		}
		One(idx) => rules[idx].clone(),
		Two(idx, jdx) => match (&rules[idx], &rules[jdx]) {
			(Token(a), Token(b)) => Tokens(vec![*a, *b]),
			(Token(a), Tokens(v)) => {
				let mut clone = v.clone();
				clone.insert(0, *a);
				Tokens(clone)
			}
			(Tokens(v), Token(b)) => {
				let mut clone = v.clone();
				clone.push(*b);
				Tokens(clone)
			}
			(Tokens(a), Tokens(b)) => {
				let mut clone = a.clone();
				clone.append(&mut b.clone());
				Tokens(clone)
			}
			(Two(a, b), One(c)) => Three(*a, *b, *c),
			(One(a), Two(b, c)) => Three(*a, *b, *c),
			(One(a), One(b)) => Two(*a, *b),
			_ => rule,
		},
		Three(idx, jdx, kdx) => match (&rules[idx], &rules[jdx], &rules[kdx]) {
			(Token(a), Token(b), Token(c)) => Tokens(vec![*a, *b, *c]),
			(Tokens(a), Token(b), Token(c)) => {
				let mut clone = a.clone();
				clone.push(*b);
				clone.push(*c);
				Tokens(clone)
			}
			(Token(a), Tokens(b), Token(c)) => {
				let mut clone = b.clone();
				clone.insert(0, *a);
				clone.push(*c);
				Tokens(clone)
			}
			(Token(a), Token(b), Tokens(c)) => {
				let mut clone = c.clone();
				clone.insert(0, *b);
				clone.insert(0, *a);
				Tokens(clone)
			}
			(Tokens(a), Tokens(b), Token(c)) => {
				let mut clone = a.clone();
				clone.append(&mut b.clone());
				clone.push(*c);
				Tokens(clone)
			}
			(Tokens(a), Token(b), Tokens(c)) => {
				let mut clone = a.clone();
				clone.push(*b);
				clone.append(&mut c.clone());
				Tokens(clone)
			}
			(Token(a), Tokens(b), Tokens(c)) => {
				let mut clone = b.clone();
				clone.insert(0, *a);
				clone.append(&mut c.clone());
				Tokens(clone)
			}
			(Tokens(a), Tokens(b), Tokens(c)) => {
				let mut clone = a.clone();
				clone.append(&mut b.clone());
				clone.append(&mut c.clone());
				Tokens(clone)
			}
			_ => rule,
		},
		_ => rule,
	}
}

fn parse_rules(input: &[&str]) -> Vec<Rule> {
	let rule_count = input
		.iter()
		.take_while(|s| s.chars().next().map(|c| c.is_ascii_digit()) == Some(true))
		.count();
	let mut rules = Vec::new();
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
