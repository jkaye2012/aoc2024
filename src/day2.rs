type Report = Vec<i8>;
type Input = Vec<Report>;

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_report_safe<'r, T: Iterator<Item = &'r i8>>(report: T) -> bool {
    let mut pairs = report.map_windows(|[a, b]| (*a, *b)).peekable();
    let (a, b) = pairs.peek().unwrap();
    let decreasing = a > b;
    pairs.all(|(a, b)| {
        let diff = a.abs_diff(*b);
        diff >= 1 && diff <= 3 && (a > b) == decreasing
    })
}

#[aoc(day2, part1)]
pub fn good_reports(reports: &[Report]) -> usize {
    reports
        .iter()
        .map(|r| is_report_safe(r.iter()))
        .filter(|b| *b)
        .count()
}

fn is_dampened_report_safe(report: &[i8]) -> bool {
    is_report_safe(report.iter())
        || (0..report.len()).any(|skip| {
            let it = report
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != skip)
                .map(|(_, reading)| reading);
            is_report_safe(it)
        })
}

#[aoc(day2, part2)]
pub fn dampened_good_reports(reports: &[Report]) -> usize {
    reports
        .iter()
        .map(|r| is_dampened_report_safe(&r))
        .filter(|b| *b)
        .count()
}
