static INPUT: &'static str = include_str!("input");

fn part1() -> isize {
    INPUT
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .fold(0, |acc, item| acc + item)
}

fn part2() -> isize {
    use std::collections::HashSet;
    let mut sum = 0;
    let mut sums = HashSet::new();
    sums.insert(0);
    
    loop {
        for val in INPUT.lines().map(|line| line.parse::<isize>().unwrap()) {
            sum += val;
            if sums.contains(&sum) {
                return sum;
            }
            sums.insert(sum);
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}