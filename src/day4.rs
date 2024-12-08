use crate::util::{Direction, FlatMatrix, FlatMatrixCoord};

type Input = FlatMatrix<char>;

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Input {
    Input::from_string(input)
}

enum WordSearch<'a> {
    Initial(&'a Input, FlatMatrixCoord),
    Mismatch,
    PartialMatch(&'a Input, [Option<(FlatMatrixCoord, Direction)>; 8]),
}

impl<'a> WordSearch<'a> {
    fn new(input: &'a Input, coord: FlatMatrixCoord) -> Self {
        Self::Initial(input, coord)
    }

    fn partial_match(self, target: char) -> Self {
        match self {
            Self::PartialMatch(matrix, mut neighbors) => {
                for i in 0..neighbors.len() {
                    neighbors[i] = if let Some((c, d)) = neighbors[i]
                        && matrix.get(c) == target
                    {
                        c.neighbor(d).and_then(|c| Some((c, d)))
                    } else {
                        None
                    }
                }
                Self::PartialMatch(matrix, neighbors)
            }
            _ => Self::Mismatch,
        }
    }

    fn final_match(self, target: char) -> usize {
        match self {
            Self::PartialMatch(matrix, neighbors) => neighbors
                .iter()
                .map(|o| {
                    o.and_then(|(c, _)| {
                        if matrix.get(c) == target {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default()
                })
                .sum(),
            _ => 0,
        }
    }

    fn x(self) -> Self {
        match self {
            Self::Initial(matrix, coord) => {
                let c = matrix.get(coord);
                if c == 'X' {
                    Self::PartialMatch(matrix, coord.all_neighbors())
                } else {
                    Self::Mismatch
                }
            }
            _ => Self::Mismatch,
        }
    }
    fn m(self) -> Self {
        self.partial_match('M')
    }

    fn a(self) -> Self {
        self.partial_match('A')
    }

    fn s(self) -> usize {
        self.final_match('S')
    }
}

#[aoc(day4, part1)]
pub fn xmas_word_search(input: &Input) -> usize {
    input
        .iter_coords()
        .map(|(coord, _)| WordSearch::new(input, coord).x().m().a().s())
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
