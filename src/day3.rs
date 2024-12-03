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
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut muls = mul_re
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (
                c.get(0).unwrap().start(),
                a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap(),
            )
        })
        .peekable();
    let mut dos = do_re
        .captures_iter(input)
        .map(|c| c.get(0).unwrap().start())
        .peekable();
    let mut donts = dont_re
        .captures_iter(input)
        .map(|c| c.get(0).unwrap().start())
        .peekable();

    let mut dont = donts.next().unwrap();

    let mut total = 0;
    while muls.peek().is_some() {
        while let Some((idx, val)) = muls.next()
            && idx < dont
        {
            total += val;
        }
        while let Some(idx) = dos.peek()
            && *idx < dont
        {
            dos.next();
        }

        if let Some(do_) = dos.peek() {
            while let Some((idx, _)) = muls.peek()
                && idx < do_
            {
                muls.next();
            }
            while let Some(idx) = donts.peek()
                && *idx < *do_
            {
                donts.next();
            }
            if let Some(dont_) = donts.peek() {
                dont = *dont_;
            } else {
                dont = usize::MAX;
            }
            donts.next();
        } else {
            break;
        }
        dos.next();
    }
    total
}
