use crate::geo::Point;
use crate::parse::{everything, line, ParseResult, Parser};
use crate::utils::gather_target::GatherTarget;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
pub use storage::GridStorage;

mod storage;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Grid<T, S = Vec<T>> {
    storage: S,
    default: T,
    width: usize,
    height: usize,
}

impl<T, S> Hash for Grid<T, S>
where
    S: Hash,
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.storage.hash(state)
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
{
    #[inline]
    fn point_to_index(&self, p: &Point<usize>) -> usize {
        let [x, y] = p.coords();
        (y * self.width) + x
    }

    #[inline]
    pub fn row(&self, y: usize) -> &[T] {
        self.storage
            .cell_range(self.width * y, self.width * (y + 1))
    }

    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = (usize, &[T])> {
        (0..self.height).map(|y| (y, self.row(y)))
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Point<usize>, &T)> {
        self.storage
            .cell_range(0, self.width * self.height)
            .iter()
            .enumerate()
            .map(|(i, v)| (Point::new(i % self.width, i / self.width), v))
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
{
    #[inline]
    pub fn count_by<F>(&self, pred: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        self.storage
            .cell_range(0, self.width * self.height)
            .iter()
            .filter(|c| pred(*c))
            .count()
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
    T: Eq,
{
    #[inline]
    pub fn find(&self, needle: &T) -> Option<Point<usize>> {
        self.storage
            .cell_range(0, self.width * self.height)
            .iter()
            .position(|c| c.eq(needle))
            .map(|p| Point::new(p % self.width, p / self.width))
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T> + GatherTarget<T>,
    T: Copy + Default,
{
    #[inline]
    pub fn parser<'i, P>(cell_parser: P) -> impl Parser<'i, Self>
    where
        P: Parser<'i, T>,
    {
        Self::parser_with_default(T::default(), cell_parser)
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T> + GatherTarget<T>,
    T: Copy,
{
    #[inline]
    pub fn parser_with_default<'i, P>(def: T, cell_parser: P) -> impl Parser<'i, Self>
    where
        P: Parser<'i, T>,
    {
        cell_parser
            .count_repetitions()
            .capped_by(b'\n')
            .rewind()
            .and(everything().capped_by(b"\n\n").or(everything()))
            .map(move |(width, mut body)| {
                let line_parser = line();

                let mut height = body.iter().filter(|v| **v == b'\n').count();
                if *body.last().unwrap() != b'\n' {
                    height += 1;
                }

                let mut storage = S::create(width * height, def);
                let mut offset = 0;

                while let ParseResult::Good(mut line, new_body) = line_parser.parse(body) {
                    if line.len() == 0 {
                        break;
                    }

                    let mut index = 0;
                    while let ParseResult::Good(v, new_line) =
                        cell_parser.parse_at_index(line, index)
                    {
                        storage[offset + index] = v;
                        line = new_line;
                        index += 1;
                    }

                    body = new_body;
                    offset += width;
                }

                Grid::new_from_storage_and_default(width, height, storage, def)
            })
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
{
    #[inline]
    pub fn new_from_storage_and_default(
        width: usize,
        height: usize,
        storage: S,
        default: T,
    ) -> Self {
        Self {
            width,
            height,
            storage,
            default,
        }
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
    T: Default,
{
    #[inline]
    pub fn new_from_storage(width: usize, height: usize, storage: S) -> Self {
        Self {
            width,
            height,
            storage,
            default: T::default(),
        }
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
    T: Copy,
{
    #[inline]
    pub fn fill(&mut self, v: T) {
        self.storage
            .cell_range_mut(0, self.width * self.height)
            .fill(v);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage
            .cell_range_mut(0, self.width * self.height)
            .fill(self.default);
    }

    #[inline]
    pub fn new_with_value(width: usize, height: usize, default: T) -> Self {
        Self {
            storage: S::create(width * height, default),
            width,
            height,
            default,
        }
    }
}

impl<T, S> Index<Point<usize>> for Grid<T, S>
where
    S: GridStorage<T>,
{
    type Output = T;

    #[inline]
    fn index(&self, index: Point<usize>) -> &Self::Output {
        let index = self.point_to_index(&index);
        &self.storage[index]
    }
}

impl<T, S> IndexMut<Point<usize>> for Grid<T, S>
where
    S: GridStorage<T>,
{
    #[inline]
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        let index = self.point_to_index(&index);
        &mut self.storage[index]
    }
}

impl<T, S> Index<(usize, usize)> for Grid<T, S>
where
    S: GridStorage<T>,
{
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let index = (y * self.width) + x;
        &self.storage[index]
    }
}

impl<T, S> IndexMut<(usize, usize)> for Grid<T, S>
where
    S: GridStorage<T>,
{
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let index = (y * self.width) + x;
        &mut self.storage[index]
    }
}

impl<S> Grid<u8, S>
where
    S: GridStorage<u8>,
{
    pub fn parse_padded(input: &[u8], padding: u8) -> Grid<u8, S> {
        Self::parse_padded_map(input, padding, |v| v)
    }
}

impl<T, S> Grid<T, S>
where
    S: GridStorage<T>,
    T: Copy,
{
    pub fn parse_padded_map<F>(input: &[u8], padding: T, f: F) -> Grid<T, S>
    where
        F: Fn(u8) -> T,
    {
        let width = input.iter().position(|v| *v == b'\n').unwrap();
        let height = input.len() / (width + 1);
        let mut grid = Grid::new_with_value(width + 2, height + 2, padding);

        let mut x = 1;
        let mut y = 1;
        for v in input.iter() {
            if *v == b'\n' {
                y += 1;
                x = 1;
            } else {
                grid[(x, y)] = f(*v);
                x += 1;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{any_byte, choice, unsigned_int};

    const GRID_01: &[u8] = include_bytes!("./test_fixtures/grid_01.txt");

    #[test]
    fn can_parse_grid() {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum Maze {
            Ground,
            Wall,
            Entrance,
            Door(u8),
            Key(u8),
        }
        use Maze::*;

        let cell_parser = choice((
            b'#'.map_to(Wall),
            b'.'.map_to(Ground),
            b'@'.map_to(Entrance),
            any_byte().in_range(b'a'..b'z').map(Key),
            any_byte()
                .in_range(b'A'..b'Z')
                .map(|d| Door(d + (b'a' - b'A'))),
        ));

        let grid = Grid::parser_with_default(Wall, cell_parser)
            .parse(GRID_01)
            .unwrap();

        assert_eq!(
            grid,
            Grid {
                width: 9,
                height: 5,
                default: Wall,
                storage: vec![
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Key(b'b'),
                    Ground,
                    Door(b'a'),
                    Ground,
                    Entrance,
                    Ground,
                    Key(b'a'),
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Door(b'b'),
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Key(b'd'),
                    Ground,
                    Door(b'c'),
                    Ground,
                    Ground,
                    Ground,
                    Key(b'c'),
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                    Wall,
                ],
            }
        )
    }

    #[test]
    fn can_parse_csv_grid() {
        let grid_arr: Grid<u16, [_; 16]> = Grid::parser(unsigned_int().delimited_by(b','))
            .parse(b"1,2,3\n4,55,6\n7,8,9\n")
            .unwrap();

        let grid_vec: Grid<u16, Vec<u16>> = Grid::parser(unsigned_int().delimited_by(b','))
            .parse(b"1,2,3\n4,55,6\n7,8,9\n")
            .unwrap();

        assert_eq!(
            grid_arr,
            Grid {
                width: 3,
                height: 3,
                storage: [1, 2, 3, 4, 55, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0],
                default: 0,
            }
        );

        assert_eq!(grid_arr[Point::new(1, 1)], 55);
        assert_eq!(grid_vec[Point::new(1, 1)], 55);

        assert_eq!(grid_vec.storage, vec![1, 2, 3, 4, 55, 6, 7, 8, 9]);
    }
}
