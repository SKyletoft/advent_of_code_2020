fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(input);
	let sol2 = solve2(input);
	println!("{} {}", sol1, sol2);
}

const PASSPORT_FIELDS: [Option<&str>; 7] = [
	Some("byr:"),
	Some("iyr:"),
	Some("eyr:"),
	Some("hgt:"),
	Some("hcl:"),
	Some("ecl:"),
	Some("pid:"),
];

fn solve1(input: &str) -> u64 {
	let mut buf = PASSPORT_FIELDS.to_vec();
	input
		.split("\n\n")
		.filter(|passport| {
			buf = PASSPORT_FIELDS.to_vec();
			passport.split_ascii_whitespace().for_each(|field| {
				if let Some(idx) = buf.iter().position(|x| x == &field.get(..4)) {
					buf.remove(idx);
				}
			});
			buf.is_empty()
		})
		.count() as u64
}

fn solve2(input: &str) -> u64 {
	let mut buf = PASSPORT_FIELDS.to_vec();
	input
		.split("\n\n")
		.filter(|passport| {
			buf = PASSPORT_FIELDS.to_vec();
			for field in passport.split_ascii_whitespace() {
				if let Some(idx) = buf.iter().position(|x| x == &field.get(..4)) {
					match field {
						s if &s[..4] == "byr:" && valid_year(&s[4..], 1920, 2002) => {
							buf.remove(idx);
						}
						s if &s[..4] == "iyr:" && valid_year(&s[4..], 2010, 2020) => {
							buf.remove(idx);
						}
						s if &s[..4] == "eyr:" && valid_year(&s[4..], 2020, 2030) => {
							buf.remove(idx);
						}
						s if &s[..4] == "hgt:" && valid_height(&s[4..]) => {
							buf.remove(idx);
						}
						s if &s[..4] == "hcl:" && valid_hair_colour(&s[4..]) => {
							buf.remove(idx);
						}
						s if &s[..4] == "ecl:" && valid_eye_colour(&s[4..]) => {
							buf.remove(idx);
						}
						s if &s[..4] == "pid:" && valid_pid(&s[4..]) => {
							buf.remove(idx);
						}
						s if &s[..4] == "cid:" => {
							buf.remove(idx);
						}
						_ => {}
					}
				}
			}
			buf.is_empty()
		})
		.count() as u64
}

fn valid_year(s: &str, min: u16, max: u16) -> bool {
	if s.len() != 4 {
		false
	} else if let Ok(n) = s.parse::<u16>() {
		min <= n && n <= max
	} else {
		false
	}
}

fn valid_height(s: &str) -> bool {
	if s.len() < 2 {
		false
	} else if let Ok(n) = s[..s.len() - 2].parse::<u16>() {
		if &s[s.len() - 2..] == "in" {
			n >= 59 && n <= 76
		} else if &s[s.len() - 2..] == "cm" {
			n >= 150 && n <= 193
		} else {
			false
		}
	} else {
		false
	}
}

fn valid_hair_colour(s: &str) -> bool {
	let b = s.as_bytes();
	b.len() == 7 && b[0] == b'#' && b.iter().skip(1).all(|x| x.is_ascii_hexdigit())
}

fn valid_eye_colour(s: &str) -> bool {
	["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
}

fn valid_pid(s: &str) -> bool {
	s.len() == 9 && s.bytes().all(|d| d.is_ascii_digit())
}
