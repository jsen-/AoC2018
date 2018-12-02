static INPUT: &'static str = include_str!("input");

fn part1() -> isize {
    use std::collections::hash_map::{HashMap, Entry};
    let mut twos = 0;
    let mut threes = 0;

    INPUT.lines().for_each(|line| {
        let mut map = HashMap::new();
        line.chars().for_each(|ch| {
            match map.entry(ch) {
                Entry::Vacant(entry) => {entry.insert(1);},
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            };
        });
        map.iter().find(|(_ch, &counts)| counts == 2).map(|_| twos += 1);
        map.iter().find(|(_ch, &counts)| counts == 3).map(|_| threes += 1);
    });
    return twos * threes;
}

fn part2() -> () {
    use std::collections::HashSet;
    let mut set = HashSet::<&str>::new();

    INPUT.lines().for_each(|line| {
        set.iter().for_each(|stored_line: &&str| {
            let distance: isize = stored_line.chars().zip(line.chars())
                .map(|(ch1, ch2)| {
                    if ch1 == ch2 {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            if distance == 1 {
                println!("{}\n{}", line, stored_line);
            }
        });
        set.insert(line);
    })
}

fn main() {
    println!("{}", part1());
    part2();
}