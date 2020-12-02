fn main() {
    let inputs = include_str!("input.txt").lines().collect::<Vec<_>>();
    let sol = inputs.iter().filter(part_two).count();
    println!("{}", sol);
}

fn split_input(line: &str) -> Option<(usize, usize, char, &str)> {
    let mut iter = line.split(|x: char| ['-', ':', ' '].contains(&x));
    if let (Some(fst), Some(snd), Some(letter), Some(""), Some(pwd)) = (
        iter.next(),
        iter.next(),
        iter.next(),
        iter.next(),
        iter.next(),
    ) {
        if let (Ok(fst), Ok(snd), Some(letter)) = (
            fst.parse(),
            snd.parse(),
            letter.chars().next(),
        ) {
            Some((fst, snd, letter, pwd))
        } else {
            None
        }
    } else {
        None
    }
}

fn part_one(line: &&&str) -> bool {
    let (fst, snd, letter, pwd) = split_input(line).expect("invalid input");
    let count = pwd.chars().filter(|&c| c == letter).count();
    fst <= count && count <= snd
}

fn part_two(line: &&&str) -> bool {
    let (fst, snd, letter, pwd) = split_input(line).expect("invalid input");
    (pwd.chars().nth(fst - 1) == Some(letter)) ^ (pwd.chars().nth(snd - 1) == Some(letter))
}
