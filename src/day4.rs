use crate::util::{Direction, FlatMatrix};

type Input = FlatMatrix<char>;

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Input {
    Input::from_string(input)
}

const ALL_QUERIES: [[(Direction, char); 3]; 8] = [
    [
        (Direction::UpLeft, 'M'),
        (Direction::UpLeft, 'A'),
        (Direction::UpLeft, 'S'),
    ],
    [
        (Direction::Up, 'M'),
        (Direction::Up, 'A'),
        (Direction::Up, 'S'),
    ],
    [
        (Direction::UpRight, 'M'),
        (Direction::UpRight, 'A'),
        (Direction::UpRight, 'S'),
    ],
    [
        (Direction::Right, 'M'),
        (Direction::Right, 'A'),
        (Direction::Right, 'S'),
    ],
    [
        (Direction::DownRight, 'M'),
        (Direction::DownRight, 'A'),
        (Direction::DownRight, 'S'),
    ],
    [
        (Direction::Down, 'M'),
        (Direction::Down, 'A'),
        (Direction::Down, 'S'),
    ],
    [
        (Direction::DownLeft, 'M'),
        (Direction::DownLeft, 'A'),
        (Direction::DownLeft, 'S'),
    ],
    [
        (Direction::Left, 'M'),
        (Direction::Left, 'A'),
        (Direction::Left, 'S'),
    ],
];

#[aoc(day4, part1)]
pub fn xmas_word_search(input: &Input) -> usize {
    input
        .iter_coords()
        .map(|(coord, c)| {
            if *c == 'X' {
                ALL_QUERIES
                    .iter()
                    .map(|q| if input.query(coord, q.iter()) { 1 } else { 0 })
                    .sum()
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn x_mas_word_search(input: &Input) -> usize {
    input
        .iter_coords()
        .map(|(coord, c)| {
            if *c == 'A'
                && let Some(nw) = coord.neighbor(Direction::UpLeft)
                && let Some(ne) = coord.neighbor(Direction::UpRight)
                && let Some(se) = coord.neighbor(Direction::DownRight)
                && let Some(sw) = coord.neighbor(Direction::DownLeft)
                && ((input.get(nw) == 'M' && input.get(se) == 'S')
                    || (input.get(nw) == 'S' && input.get(se) == 'M'))
                && ((input.get(ne) == 'M' && input.get(sw) == 'S')
                    || (input.get(ne) == 'S' && input.get(sw) == 'M'))
            {
                1
            } else {
                0
            }
        })
        .sum()
}
