use common::aoc::{BothParts, Day};
use common::geo::Point;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Universe::parser().parse(input).unwrap());

    day.note("Galaxies", input.galaxies.len());
    day.note("Row Gaps", input.empty_rows.len());
    day.note("Col Gaps", input.empty_cols.len());

    day.part("Both Parts", || input.both_parts::<999999>());
}

struct Universe {
    galaxies: Vec<Point<i32>>,
    empty_rows: Vec<i32>,
    empty_cols: Vec<i32>,
}

impl Universe {
    fn both_parts<const N: i64>(&self) -> BothParts<i32, i64> {
        let (sum, total_exps) = self.distances();

        #[cfg(test)]
        println!("{} {}", sum, total_exps);

        BothParts(sum + total_exps, sum as i64 + (total_exps as i64 * N))
    }

    fn distances(&self) -> (i32, i32) {
        let mut sum = 0;
        let mut total_exps = 0;

        for i in 0..self.galaxies.len() {
            let [ix, iy] = *self.galaxies[i].coords();
            for j in (i + 1)..self.galaxies.len() {
                let [jx, jy] = *self.galaxies[j].coords();

                let (min_x, max_x) = if jx > ix { (ix, jx) } else { (jx, ix) };
                let (min_y, max_y) = if jy > iy { (iy, jy) } else { (jy, iy) };

                let exps = self.empty_rows.iter().filter(|y| **y > min_y && **y < max_y).count()
                    + self.empty_cols.iter().filter(|x| **x > min_x && **x < max_x).count();

                let dist = (jx - ix).abs() + (jy - iy).abs();

                #[cfg(test)]
                println!("{}-{}: {} ({},{} -> {},{} + {})", i + 1, j + 1, dist, ix, iy, jx, jy, exps);

                sum += dist;
                total_exps += exps;
            }
        }

        (sum, total_exps as i32)
    }

    fn new() -> Self {
        Self{
            galaxies: Vec::with_capacity(64),
            empty_rows: Vec::with_capacity(64),
            empty_cols: Vec::with_capacity(64),
        }
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::everything()
            .map(|grid| {
                let mut universe = Self::new();
                let mut x = 0;
                let mut y = 0;
                let mut w = 0;
                let mut empty_row = true;

                for b in grid {
                    match b {
                        b'.' => {
                            x += 1
                        }
                        b'#' => {
                            universe.galaxies.push(Point::new(x, y));
                            empty_row = false;
                            x += 1;
                        }
                        b'\n' => {
                            if empty_row {
                                universe.empty_rows.push(y);
                            }

                            empty_row = true;
                            w = x;
                            y += 1;
                            x = 0;
                        }

                        _ => panic!("Unrecognized character {}", b)
                    }
                }

                universe.empty_cols.extend(
                    (0..w).filter(|x| universe.galaxies.iter().find(|g| g.coords()[0] == *x).is_none())
                );

                universe
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn p1_works_on_example() {
        let input = Universe::parser().parse(P1_EXAMPLE).unwrap();

        assert_eq!(input.galaxies.len(), 9);
        assert_eq!(&input.empty_rows, &[3, 7]);
        assert_eq!(&input.empty_cols, &[2, 5, 8]);
        assert_eq!(input.both_parts::<99>(), BothParts(374, 8410));
        assert_eq!(input.both_parts::<9>(), BothParts(374, 1030));
    }
}

