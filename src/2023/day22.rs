use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Slab::parse_list(input));
    let input = day.prep("Settle", || settle(&input));

    day.note("Amount of slabs", input.len());

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn settle(slabs: &[Slab]) -> Vec<Slab> {
    let mut slabs = slabs.to_vec();
    slabs.sort_by(|a, b| a.2.cmp(&b.2));
    for i in 0..slabs.len() {
        let mut fall_height = slabs[i].2;
        for j in 0..i {
            if let FallResult::Over(height) = slabs[i].check_fall(slabs[j]) {
                if height < fall_height {
                    fall_height = height;
                }
            }

            if fall_height == 0 {
                break;
            }
        }

        if fall_height > 0 {
            slabs[i].2 -= fall_height;
        }
    }

    slabs.sort_by(|a, b| a.2.cmp(&b.2));

    slabs
}

fn p1(slabs: &[Slab]) -> usize {
    let mut cannot_disintegrate = vec![false; slabs.len()];
    let mut can_disintegrate = slabs.len();

    for (i, slab) in slabs.iter().copied().enumerate() {
        let mut single_support: Option<usize> = None;

        for (j, other) in slabs.iter().copied().enumerate() {
            if j == i || other.z_above() != slab.2 {
                continue
            }

            if let FallResult::Over(height) = slab.check_fall(other) {
                if height == 0 {
                    if let Some(_) = single_support {
                        single_support = None;
                        break;
                    } else {
                        single_support = Some(j)
                    }
                }
            }
        }

        if let Some(index) = single_support {
            let cannot_disintegrate = &mut cannot_disintegrate[index];
            if !*cannot_disintegrate {
                *cannot_disintegrate = true;
                can_disintegrate -= 1;
            }
        }
    }

    #[cfg(test)]
    println!("{:?}", cannot_disintegrate.iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| (i as u8 + b'A') as char)
        .collect::<Vec<_>>()
    );

    can_disintegrate
}

fn p2(slabs: &[Slab]) -> usize {
    let mut graph = vec![Vec::new(); slabs.len()];
    for i in 0..slabs.len() {
        let si = slabs[i];
        let top = si.z_above();
        for j in (i+1)..slabs.len() {
            let sj = slabs[j];
            if sj.2 < top {
                continue;
            } else if sj.2 > top {
                break;
            }

            if let FallResult::Over(v) = sj.check_fall(si) {
                if v == 0 {
                    graph[i].push(j);
                }
            }
        }
    }

    let floor_len = slabs.iter().position(|s| s.2 > 0).unwrap();
    let total_other_bricks = slabs.len() - 1;

    let mut stack = Vec::with_capacity(16);
    let mut covered = vec![false; slabs.len()];
    let mut total = 0;
    for i in 0..slabs.len() {
        let mut seen = 0;

        stack.clear();
        stack.extend(0..floor_len);
        covered.fill(false);
        covered[i] = true;

        while let Some(curr) = stack.pop() {
            if covered[curr] {
                continue;
            }

            seen += 1;
            covered[curr] = true;
            stack.extend_from_slice(&graph[curr]);
        }

        #[cfg(test)]
        println!("{} {}", (i as u8 + b'A') as char, total_other_bricks - seen);

        total += total_other_bricks - seen;
    }

    total
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Slab(u16, u16, u16, u16, Shape);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Shape {
    Wide,
    Deep,
    Tall,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum FallResult {
    Over(u16),
    Passes,
    NotAbove,
}

impl Slab {
    #[inline]
    fn z_above(&self) -> u16 {
        match self.4 {
            Shape::Wide | Shape::Deep => self.2 + 1,
            Shape::Tall => self.2 + self.3 + 1,
        }
    }

    #[inline]
    fn check_fall(self, other: Self) -> FallResult {
        let Slab(x1, y1, z1, l1, s1) = self;
        let Slab(x2, y2, z2, l2, s2) = other;

        match (s1, s2) {
            (Shape::Wide, Shape::Wide) => {
                if z1 > z2 {
                    if y1 == y2 && (x1 + l1 >= x2 && x1 <= x2 + l2) {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Wide, Shape::Deep) => {
                if z1 > z2 {
                    if x1 <= x2 && x1 + l1 >= x2 && y1 >= y2 && y1 <= y2+l2 {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Wide, Shape::Tall) => {
                if z1 > z2 + l2 {
                    if y1 == y2 && x2 >= x1 && x2 <= x1 + l1 {
                        FallResult::Over(z1 - (z2 + l2) - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Deep, Shape::Wide) => {
                if z1 > z2 {
                    if y1 <= y2 && y1 + l1 >= y2 && x1 >= x2 && x1 <= x2+l2 {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Deep, Shape::Deep) => {
                if z1 > z2 {
                    if x1 == x2 && (y1 + l1 >= y2 && y1 <= y2 + l2) {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Deep, Shape::Tall) => {
                if z1 > z2 + l2 {
                    if x1 == x2 && y2 >= y1 && y2 <= y1 + l1 {
                        FallResult::Over(z1 - (z2 + l2) - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Tall, Shape::Wide) => {
                if z1 > z2 {
                    if y1 == y2 && x1 >= x2 && x1 <= x2 + l2 {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Tall, Shape::Deep) => {
                if z1 > z2 {
                    if x1 == x2 && y1 >= y2 && y1 <= y2 + l2 {
                        FallResult::Over(z1 - z2 - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
            (Shape::Tall, Shape::Tall) => {
                if z1 > z2 + l2 {
                    if x1 == x2 && y1 == y2 {
                        FallResult::Over(z1 - (z2 + l2) - 1)
                    } else {
                        FallResult::Passes
                    }
                } else {
                    FallResult::NotAbove
                }
            }
        }
    }

    fn parse_list(input: &[u8]) -> Vec<Self> {
        Self::parser()
            .delimited_by(b'\n')
            .repeat()
            .parse(input)
            .unwrap()
    }

    #[cfg(test)]
    fn parse(input: &[u8]) -> Self {
        Self::parser().parse(input).unwrap()
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::unsigned_int()
            .delimited_by(b',')
            .repeat_n(3)
            .delimited_by(b'~')
            .repeat_n(2)
            .map(|(a, b)| Self::new(a, b))
    }

    #[inline]
    fn new(a: [u16; 3], b: [u16; 3]) -> Self {
        if a[0] != b[0] {
            Self(a[0], a[1], a[2], b[0] - a[0], Shape::Wide)
        } else if a[1] != b[1] {
            Self(a[0], a[1], a[2], b[1] - a[1], Shape::Deep)
        } else {
            Self(a[0], a[1], a[2], b[2] - a[2], Shape::Tall)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FallResult::*;
    use super::Shape::*;
    use super::*;

    #[test]
    fn parse_does_the_thing() {
        assert_eq!(Slab::parse(b"0,0,0~4,0,0"), Slab(0, 0, 0, 4, Wide));
        assert_eq!(Slab::parse(b"0,0,0~0,4,0"), Slab(0, 0, 0, 4, Deep));
        assert_eq!(Slab::parse(b"0,0,0~0,0,4"), Slab(0, 0, 0, 4, Tall));
        assert_eq!(Slab::parse(b"0,0,4~0,0,4"), Slab(0, 0, 4, 0, Tall));
    }

    #[test]
    fn fall_check_works() {
        let check = |a, b| Slab::check_fall(Slab::parse(a), Slab::parse(b));

        // Wide, Wide
        assert_eq!(check(b"0,0,1~4,0,1", b"0,0,2~4,0,2"), NotAbove);
        assert_eq!(check(b"0,0,4~4,0,4", b"0,0,2~4,0,2"), Over(1));
        assert_eq!(check(b"0,1,4~4,1,4", b"0,0,2~4,0,2"), Passes);
        assert_eq!(check(b"4,0,6~8,0,6", b"0,0,2~4,0,2"), Over(3));
        assert_eq!(check(b"5,0,6~8,0,6", b"0,0,2~4,0,2"), Passes);
        assert_eq!(check(b"5,0,8~8,0,8", b"8,0,2~12,0,2"), Over(5));
        assert_eq!(check(b"5,0,6~8,0,6", b"9,0,2~12,0,2"), Passes);

        // Wide, Deep
        assert_eq!(check(b"0,0,2~4,0,2", b"2,1,0~2,4,0"), Passes);
        assert_eq!(check(b"0,1,2~4,1,2", b"2,1,0~2,4,0"), Over(1));
        assert_eq!(check(b"0,4,2~4,4,2", b"2,1,0~2,4,0"), Over(1));
        assert_eq!(check(b"0,4,2~4,4,2", b"2,1,3~2,4,3"), NotAbove);
        assert_eq!(check(b"0,5,2~4,5,2", b"2,1,0~2,4,0"), Passes);

        // Wide, Tall
        assert_eq!(check(b"0,0,0~5,0,0", b"3,3,0~3,3,3"), NotAbove);
        assert_eq!(check(b"0,0,3~5,0,3", b"3,3,0~3,3,3"), NotAbove);
        assert_eq!(check(b"0,0,4~5,0,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"0,0,4~5,0,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"0,2,4~5,2,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"0,4,4~5,4,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"0,3,4~5,3,4", b"3,3,0~3,3,3"), Over(0));

        // Deep, Wide
        assert_eq!(check(b"0,0,2~0,4,2", b"1,2,0~4,2,0"), Passes);
        assert_eq!(check(b"1,0,2~1,4,2", b"1,2,0~4,2,0"), Over(1));
        assert_eq!(check(b"4,0,2~4,4,2", b"1,2,0~4,2,0"), Over(1));
        assert_eq!(check(b"4,0,2~4,4,2", b"1,2,3~4,2,3"), NotAbove);
        assert_eq!(check(b"5,0,2~5,4,2", b"1,2,0~4,2,0"), Passes);

        // Deep, Deep
        assert_eq!(check(b"0,0,1~0,4,1", b"0,0,2~0,4,2"), NotAbove);
        assert_eq!(check(b"0,0,4~0,4,4", b"0,0,2~0,4,2"), Over(1));
        assert_eq!(check(b"1,0,4~1,4,4", b"0,0,2~0,4,2"), Passes);
        assert_eq!(check(b"0,4,6~0,8,6", b"0,0,2~0,4,2"), Over(3));
        assert_eq!(check(b"0,5,6~0,8,6", b"0,0,2~0,4,2"), Passes);
        assert_eq!(check(b"0,5,8~0,8,8", b"0,8,2~0,12,2"), Over(5));
        assert_eq!(check(b"0,5,6~0,8,6", b"0,9,2~0,12,2"), Passes);

        // Deep, Tall
        assert_eq!(check(b"0,0,0~0,5,0", b"3,3,0~3,3,3"), NotAbove);
        assert_eq!(check(b"0,0,3~0,5,3", b"3,3,0~3,3,3"), NotAbove);
        assert_eq!(check(b"0,0,4~0,5,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"0,0,4~0,5,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"2,0,4~2,5,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"4,0,4~4,5,4", b"3,3,0~3,3,3"), Passes);
        assert_eq!(check(b"3,0,4~3,5,4", b"3,3,0~3,3,3"), Over(0));

        // Tall, Wide
        assert_eq!(check(b"0,0,0~0,0,9", b"3,1,0~8,1,0"), NotAbove);
        assert_eq!(check(b"0,0,1~0,0,9", b"3,1,0~8,1,0"), Passes);
        assert_eq!(check(b"3,1,3~3,1,9", b"3,1,0~8,1,0"), Over(2));
        assert_eq!(check(b"8,1,3~8,1,9", b"3,1,0~8,1,0"), Over(2));
        assert_eq!(check(b"2,1,3~2,1,9", b"3,1,0~8,1,0"), Passes);
        assert_eq!(check(b"9,1,3~9,1,9", b"3,1,0~8,1,0"), Passes);
        assert_eq!(check(b"3,0,3~3,0,9", b"3,1,0~8,1,0"), Passes);
        assert_eq!(check(b"3,2,3~3,2,9", b"3,1,0~8,1,0"), Passes);

        // Tall, Deep
        assert_eq!(check(b"0,0,0~0,0,9", b"1,3,0~1,8,0"), NotAbove);
        assert_eq!(check(b"0,0,1~0,0,9", b"1,3,0~1,8,0"), Passes);
        assert_eq!(check(b"1,3,3~1,3,9", b"1,3,0~1,8,0"), Over(2));
        assert_eq!(check(b"1,8,3~1,8,9", b"1,3,0~1,8,0"), Over(2));
        assert_eq!(check(b"1,2,3~1,2,9", b"1,3,0~1,8,0"), Passes);
        assert_eq!(check(b"1,9,3~1,9,9", b"1,3,0~1,8,0"), Passes);
        assert_eq!(check(b"0,3,3~0,3,9", b"1,3,0~1,8,0"), Passes);
        assert_eq!(check(b"2,3,3~2,3,9", b"1,3,0~1,8,0"), Passes);

        // Tall, Tall
        assert_eq!(check(b"0,0,0~0,0,9", b"1,1,0~1,1,4"), NotAbove);
        assert_eq!(check(b"0,0,4~0,0,9", b"1,1,0~1,1,4"), NotAbove);
        assert_eq!(check(b"0,0,5~0,0,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"1,0,5~1,0,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"2,0,5~2,0,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"0,1,5~0,1,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"1,1,5~1,1,9", b"1,1,0~1,1,4"), Over(0));
        assert_eq!(check(b"2,1,5~2,1,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"0,2,5~0,2,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"1,2,5~1,2,9", b"1,1,0~1,1,4"), Passes);
        assert_eq!(check(b"2,2,5~2,2,9", b"1,1,0~1,1,4"), Passes);
    }

    #[test]
    fn p1_works_on_example() {
        assert_eq!(p1(&settle(&Slab::parse_list(P1_EXAMPLE))), 5);
    }

    #[test]
    fn p2_works_on_example() {
        assert_eq!(p2(&settle(&Slab::parse_list(P1_EXAMPLE))), 7);
    }

    const P1_EXAMPLE: &[u8] = b"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
}
