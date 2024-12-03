use regex::Regex;

#[aoc(day3, part1)]
pub fn uncorrupt(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn uncorrupt_conditional(input: &str) -> u64 {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();

    let mut toggle = 1;
    re.captures_iter(input)
        .map(|c| {
            if c.name("do").is_some() {
                toggle = 1;
                0
            } else if c.name("dont").is_some() {
                toggle = 0;
                0
            } else {
                let a = c.name("a").unwrap().as_str();
                let b = c.name("b").unwrap().as_str();
                a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap() * toggle
            }
        })
        .sum()
}
