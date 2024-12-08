use std::slice::Iter;

pub struct FlatMatrix<T> {
    matrix: Vec<T>,
    width: usize,
    height: usize,
}

impl FlatMatrix<char> {
    pub fn from_string(input: &str) -> Self {
        let matrix = input
            .chars()
            .filter_map(|c| if c == '\n' { None } else { Some(c) })
            .collect();
        Self {
            matrix,
            width: input.find('\n').unwrap(),
            height: input.lines().count(),
        }
    }
}

impl<T: Copy> FlatMatrix<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.matrix.iter()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (FlatMatrixCoord, &T)> {
        self.iter()
            .enumerate()
            .map(|(idx, c)| (FlatMatrixCoord::new(idx, self.width, self.height), c))
    }

    pub fn get(&self, coord: FlatMatrixCoord) -> T {
        self.matrix[coord.index()]
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

impl Direction {
    const fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::UpRight => (1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (-1, 1),
            Direction::UpLeft => (-1, -1),
        }
    }

    pub const fn all() -> [Self; 8] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
            Direction::UpLeft,
        ]
    }
}

#[derive(Copy, Clone)]
pub struct FlatMatrixCoord {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn checked_sub(x: usize, dx: isize) -> Option<usize> {
    if dx < 0 {
        x.checked_sub(dx.abs() as usize)
    } else {
        x.checked_add(dx as usize)
    }
}

impl FlatMatrixCoord {
    pub fn new(idx: usize, width: usize, height: usize) -> Self {
        Self {
            x: idx % height,
            y: idx / height,
            width,
            height,
        }
    }

    pub fn index(&self) -> usize {
        self.x + self.y * self.height
    }

    pub fn neighbor(&self, dir: Direction) -> Option<Self> {
        let (dx, dy) = dir.offset();
        let (x, y) = (checked_sub(self.x, dx), checked_sub(self.y, dy));
        if let Some(x) = x
            && let Some(y) = y
            && x < self.width
            && y < self.height
        {
            Some(Self {
                x,
                y,
                width: self.width,
                height: self.height,
            })
        } else {
            None
        }
    }

    pub fn all_neighbors(&self) -> [Option<(Self, Direction)>; 8] {
        let mut result = [None; 8];
        for (i, dir) in Direction::all().iter().enumerate() {
            result[i] = self.neighbor(*dir).and_then(|s| Some((s, *dir)));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_matrix_from_string() {
        let input = "..#\n#..\n...";
        let matrix = FlatMatrix::from_string(input);
        assert_eq!(matrix.width, 3);
        assert_eq!(matrix.height, 3);
        assert_eq!(matrix.get(FlatMatrixCoord { x: 0, y: 0 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 1, y: 0 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 2, y: 0 }), '#');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 0, y: 1 }), '#');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 1, y: 1 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 2, y: 1 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 0, y: 2 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 1, y: 2 }), '.');
        assert_eq!(matrix.get(FlatMatrixCoord { x: 2, y: 2 }), '.');
    }
}
