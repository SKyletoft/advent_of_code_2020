fn main() {
	let input: Vec<usize> = include_str!("input.txt")
		.lines()
		.map(|x| x.parse::<usize>().unwrap())
		.collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(sol1, &input);
	println!("{} {}", sol1, sol2);
}

fn solve1(nums: &[usize]) -> usize {
	for i in 0..(nums.len() - 26) {
		let nums = &nums[i..];
		if !is_sum_of(nums[25], &nums[..25]) {
			return nums[25];
		}
	}
	unreachable!();
}

fn solve2(target: usize, nums: &[usize]) -> usize {
	for i in 0..nums.len() {
		let mut sum = 0;
		for j in 0.. {
			sum += nums[i + j];
			if sum == target && j >= 2 {
				let range = &nums[i..=i + j];
				let max = *range.iter().max().unwrap_or(&0);
				let min = *range.iter().min().unwrap_or(&0);
				assert!(range.iter().sum::<usize>() == target);
				return max + min;
			}
			if sum > target {
				break;
			}
		}
	}
	0
}

fn is_sum_of(n: usize, nums: &[usize]) -> bool {
	nums.iter()
		.enumerate()
		.any(|(i, &lhs)| nums.iter().skip(i + 1).any(|&rhs| lhs + rhs == n))
}
