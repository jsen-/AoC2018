#![allow(dead_code)]
static INPUT: &'static str = include_str!("input");
use combine::{many1, Parser, token, Stream, ParseError};
use combine::char::{spaces, char, digit};

#[derive(Debug, Eq, Hash)]
struct Coords {
    id: String,
    left_top: (usize, usize),
    width_height: (usize, usize),
}
impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        self.id == other.id
    }
}

fn digits<I>() -> impl Parser<Input = I, Output = usize>
where I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many1(digit()).map(|digits: String| -> usize {
        digits.parse().unwrap()
    })
}

// pun intended
fn get_rect(shape: &Coords) -> (usize, usize, usize, usize) {
    let left = shape.left_top.0;
    let right = left + shape.width_height.0;

    let top = shape.left_top.1;
    let bottom = top + shape.width_height.1;
    
    (left, right, top, bottom)
}


use std::cmp::{min, max};
fn calculate_overlap(a: &Coords, b: &Coords) -> usize {
    let (aleft, aright, atop, abottom) = get_rect(a);
    let (bleft, bright, btop, bbottom) = get_rect(b);

    let x = max(0, min(aright, bright) as isize - max(aleft, bleft) as isize);
    let y = max(0, min(abottom, bbottom) as isize - max(atop, btop) as isize);
    x as usize * y as usize
}

fn part1_but_something_else() -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut overlap = 0;
    INPUT.lines().for_each(|line| {
        let id = token('#').with(many1(digit())).skip(spaces()).skip(char('@')).skip(spaces());
        let left_top = digits().skip(char(',')).and(digits()).skip(char(':')).skip(spaces());
        let width_height = digits().skip(char('x')).and(digits());
        let mut line_parser = id.and(left_top).and(width_height).map(|((id, left_top), width_height)| {
            Coords {
                id,
                left_top,
                width_height,
            }
        });
        let (coords, rest) = line_parser.easy_parse(line).unwrap();
        if rest != "" {
            panic!(r#"remaining input {} on line "{}""#, rest, line);
        }
        for x in &seen {
            let x = calculate_overlap(x, &coords);
            overlap += x;
        }
        seen.insert(coords);
    });
    overlap
}

fn overlap(a: &Coords, b: &Coords) -> (usize, usize, usize, usize) {
    let (aleft, aright, atop, abottom) = get_rect(a);
    let (bleft, bright, btop, bbottom) = get_rect(b);

    max(0, min(aright, bright) as isize - max(aleft, bleft) as isize);
    
    let x = max(0, min(aright, bright) as isize - max(aleft, bleft) as isize);
    let y = max(0, min(abottom, bbottom) as isize - max(atop, btop) as isize);

    if x > 0 && y > 0 {
        (max(aleft, bleft), min(aright, bright), max(atop, btop), min(abottom, bbottom))
    } else {
        (0,0,0,0)
    }
}


fn part1() -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut grid = Vec::<Vec<usize>>::new();
    let mut inc = |x, y| {
        for _ in grid.len() ..= x {
            grid.push(vec![0])
        }
        let row = &mut grid[x];
        for _ in row.len() ..= y {
            row.push(0);
        }
        row[y] += 1;
    };
    INPUT.lines().for_each(|line| {
        let id = token('#').with(many1(digit())).skip(spaces()).skip(char('@')).skip(spaces());
        let left_top = digits().skip(char(',')).and(digits()).skip(char(':')).skip(spaces());
        let width_height = digits().skip(char('x')).and(digits());
        let mut line_parser = id.and(left_top).and(width_height).map(|((id, left_top), width_height)| {
            Coords {
                id,
                left_top,
                width_height,
            }
        });
        let (coords, rest) = line_parser.easy_parse(line).unwrap();
        if rest != "" {
            panic!(r#"remaining input {} on line "{}""#, rest, line);
        }
        for x in &seen {
            let (left, right, top, bottom) = overlap(&coords, x);
            for row in top .. bottom {
                for col in left .. right {
                    inc(row, col)
                }
            }
        }
        seen.insert(coords);
    });

    let mut sum = 0;
    for row in &grid {
        for &cell in row {
            if cell >= 2 {
                sum += 1;
            }
        }
    }
    sum
}

/*
........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........
*/
fn part2() {

}

fn main() {
    println!("{}", part1());
    part2();
}