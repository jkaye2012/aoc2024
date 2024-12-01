use std::collections::BinaryHeap;

type Input = (Vec<i32>, Vec<i32>);

#[aoc_generator(day1)]
fn generate(input: &str) -> Input {
    let mut a: BinaryHeap<i32> = Default::default();
    let mut b: BinaryHeap<i32> = Default::default();
    input.lines().for_each(|l| {
        let (au, bu) = l.split_once("   ").unwrap();
        a.push(au.parse().unwrap());
        b.push(bu.parse().unwrap());
    });

    (a.into_sorted_vec(), b.into_sorted_vec())
}

#[aoc(day1, part1)]
pub fn total_distance(input: &Input) -> i32 {
    let (a, b) = input;
    a.iter()
        .zip(b.iter())
        .fold(0i32, |acc, (a, b)| acc + (a - b).abs())
}

#[aoc(day1, part2)]
pub fn similarity(input: &Input) -> i32 {
    let (a, b) = input;
    let mut scores = [0i32; 100000];
    b.iter().for_each(|b| scores[*b as usize] += 1);
    a.iter().fold(0i32, |acc, a| acc + scores[*a as usize] * a)
}
