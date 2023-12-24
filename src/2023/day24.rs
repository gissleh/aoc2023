use common::aoc::Day;
use common::parse;
use common::parse::Parser;

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

pub fn main(day: &mut Day, input: &[u8]) {
    let (input_i64, input_f64) = day.prep("Parse", || parse(input));

    day.note("Input length", input.len());

    let v = day.part("Part 1", || p1(&input_f64, MIN, MAX));
    //day.part("Part 2", || 0);

    assert!(v > 271);
}

fn parse(input: &[u8]) -> (Vec<Hailstone<i64>>, Vec<Hailstone<f64>>) {
    let hails = Hailstone::parse_list(input);
    let hails_f64 = hails.iter().map(|h| h.to_f64()).collect();

    (hails, hails_f64)
}

fn p1(hails: &[Hailstone<f64>], min: f64, max: f64) -> usize {
    hails
        .iter()
        .enumerate()
        .flat_map(|(i, h)| hails[i + 1..].iter().map(move |h2| (h, h2)))
        .filter_map(|(h1, h2)| h1.intersect_xy(h2, max))
        .filter(|(x, y)| *x >= min && *x <= max && *y >= min && *y <= max)
        .count()
}

#[derive(Copy, Clone)]
struct Hailstone<T>((T, T, T), (T, T, T));

impl Hailstone<i64> {
    fn to_f64(&self) -> Hailstone<f64> {
        let Hailstone((px, py, pz), (vx, vy, vz)) = self;
        Hailstone(
            (*px as f64, *py as f64, *pz as f64),
            (*vx as f64, *vy as f64, *vz as f64),
        )
    }

    #[allow(dead_code)]
    fn parse(input: &[u8]) -> Self {
        Self::parser().parse(input).unwrap()
    }

    fn parse_list(input: &[u8]) -> Vec<Self> {
        Self::parser()
            .delimited_by(b'\n')
            .repeat()
            .parse(input)
            .unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::signed_int()
            .delimited_by(b", ")
            .repeat_n(3)
            .delimited_by(b" @ ")
            .repeat_n(2)
            .map(|(pos, vel)| Self(pos, vel))
    }
}

impl Hailstone<f64> {
    fn intersect_xy(&self, other: &Hailstone<f64>, steps: f64) -> Option<(f64, f64)> {
        let (x1, y1, _) = self.0;
        let x2 = x1 + self.1 .0 * steps;
        let y2 = y1 + self.1 .1 * steps;
        let (x3, y3, _) = other.0;
        let x4 = x3 + other.1 .0 * steps;
        let y4 = y3 + other.1 .1 * steps;

        let s1x = x2 - x1;
        let s1y = y2 - y1;
        let s2x = x4 - x3;
        let s2y = y4 - y3;

        let d = -s2x * s1y + s1x * s2y;
        if d == 0. {
            return None;
        }

        let s = (-s1y * (x1 - x3) + s1x * (y1 - y3)) / d;
        let t = (s2x * (y1 - y3) - s2y * (x1 - x3)) / d;

        if s >= 0. && s <= 1. && t >= 0. && t <= 1. {
            Some((x1 + (t * s1x), y1 + (t * s1y)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersectional() {
        let h = |s1, s2| {
            Hailstone::parse(s1)
                .to_f64()
                .intersect_xy(&Hailstone::parse(s2).to_f64(), 40.)
                .map(|(x, y)| ((x * 100.).round() / 100., (y * 100.).round() / 100.))
                .filter(|(x, y)| *x >= 7. && *x <= 27. && *y >= 7. && *y <= 27.)
        };

        assert_eq!(
            h(b"19, 13, 30 @ -2, 1, -2", b"18, 19, 22 @ -1, -1, -2"),
            Some((14.33, 15.33))
        );
        assert_eq!(
            h(b"19, 13, 30 @ -2, 1, -2", b"20, 25, 34 @ -2, -2, -4"),
            Some((11.67, 16.67))
        );
        assert_eq!(
            h(b"19, 13, 30 @ -2, 1, -2", b"12, 31, 28 @ -1, -2, -1"),
            None,
        );
        assert_eq!(
            h(b"19, 13, 30 @ -2, 1, -2", b"20, 19, 15 @ 1, -5, -3"),
            None,
        );
        assert_eq!(
            h(b"18, 19, 22 @ -1, -1, -2", b"20, 25, 34 @ -2, -2, -4"),
            None,
        );
        assert_eq!(
            h(b"18, 19, 22 @ -1, -1, -2", b"12, 31, 28 @ -1, -2, -1"),
            None,
        );
        assert_eq!(
            h(b"20, 25, 34 @ -2, -2, -4", b"12, 31, 28 @ -1, -2, -1"),
            None,
        );
        assert_eq!(
            h(b"20, 25, 34 @ -2, -2, -4", b"20, 19, 15 @ 1, -5, -3"),
            None,
        );
        assert_eq!(
            h(b"12, 31, 28 @ -1, -2, -1", b"20, 19, 15 @ 1, -5, -3"),
            None,
        );
    }
}
