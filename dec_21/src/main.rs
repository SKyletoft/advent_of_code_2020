use std::{collections::HashMap, vec};
use Ingredient::*;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let solved = solve_allergen_ingredients(&input);
	let sol1 = solve1(&solved);
	let sol2 = solve2(&solved);
	println!("{} {}", sol1, sol2);
}

fn solve1((all_ingredients, map): &(Vec<&str>, HashMap<&str, Ingredient>)) -> usize {
	all_ingredients
		.iter()
		.filter(|&&i| map.values().all(|x| *x != Solved(i)))
		.count()
}

fn solve2((_, map): &(Vec<&str>, HashMap<&str, Ingredient>)) -> String {
	let mut list = map.iter().collect::<Vec<_>>();
	list.sort_unstable_by_key(|(s, _)| **s);
	let mut string = String::new();
	for (_, item) in list {
		if let Solved(s) = item {
			string.push_str(s);
			string.push(',');
		}
	}
	string.remove(string.len() - 1);
	string
}

#[derive(Debug, Clone, PartialEq)]
enum Ingredient<'a> {
	Solved(&'a str),
	Suspects(Vec<Vec<&'a str>>),
}

fn solve_allergen_ingredients<'a>(
	input: &[&'a str],
) -> (Vec<&'a str>, HashMap<&'a str, Ingredient<'a>>) {
	let mut map: HashMap<&str, Ingredient<'a>> = HashMap::new();
	let mut all_ingredients: Vec<&str> = Vec::new();

	for &line in input.iter() {
		let end = line
			.char_indices()
			.find(|&(_, c)| c == '(')
			.map(|(i, _)| i)
			.unwrap();
		let ingredients = line[..end].split_whitespace().collect::<Vec<_>>();

		for &ingredient in ingredients.iter() {
			all_ingredients.push(ingredient);
		}

		for allergen in line[end + 10..line.len() - 1].split(", ") {
			if map.contains_key(allergen) {
				if let Some(Suspects(v)) = map.get_mut(allergen) {
					v.push(ingredients.clone());
				} else {
					unreachable!();
				}
			} else {
				map.insert(allergen, Suspects(vec![ingredients.clone()]));
			}
		}
	}

	'outer: loop {
		for ingredient in map.values_mut() {
			if let Suspects(v) = ingredient {
				if v[0].len() == 1 {
					let confirmed = v[0].remove(0);
					*ingredient = Solved(confirmed);
					for other in map.values_mut() {
						if let Suspects(v) = other {
							v[0].retain(|&i| i != confirmed);
						}
					}
					continue 'outer;
				}
				let (fst, rest) = v.split_first_mut().unwrap();
				for filter in rest.iter_mut() {
					fst.retain(|i| filter.contains(i));
					filter.retain(|i| fst.contains(i));
				}
				if fst.len() == 1 {
					let confirmed = fst.remove(0);
					*ingredient = Solved(confirmed);
					for other in map.values_mut() {
						if let Suspects(v) = other {
							v[0].retain(|&i| i != confirmed);
						}
					}
					continue 'outer;
				}
			}
		}
		break;
	}
	(all_ingredients, map)
}
