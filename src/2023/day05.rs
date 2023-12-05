use std::mem;
use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Almanac::parser().parse(input).unwrap());

    day.note("Amount of seeds", input.seeds.len());
    day.note("Amount of ranges", input.ranges.len());
    day.note("Amount of seeds (P2)", input.seeds.array_chunks::<2>().map(|[_, b]| *b).sum::<u64>());

    day.part("Part 1", || input.lowest_location());
    day.part("Part 2", || input.lowest_location_range());
}


struct Almanac {
    seeds: Vec<u64>,
    ranges: Vec<Range>,
}

impl Almanac {
    fn lowest_location(&self) -> u64 {
        let mut winner = u64::MAX;

        for seed in self.seeds.iter() {
            let mut value = *seed;
            let mut pending = false;

            for Range(next, dest, src, len) in self.ranges.iter() {
                if pending {
                    if *next {
                        pending = false;
                    } else {
                        continue
                    }
                }
                if value < *src {
                    continue
                }

                let diff = value - *src;
                if diff >= *len {
                    continue
                }

                value = *dest + diff;
                pending = true;
            }

            if value < winner {
                winner = value;
            }
        }

        winner
    }

    fn lowest_location_range(&self) -> u64 {
        let mut curr: Vec<[u64; 2]> = Vec::with_capacity(64);
        let mut next: Vec<[u64; 2]> = self.seeds.array_chunks().copied().collect();

        for r in self.ranges.iter() {
            if r.0 {
                next.extend(curr.iter().filter(|[_, l]| *l > 0).copied());
                mem::swap(&mut curr, &mut next);
                next.clear();
            }

            for seed in curr.iter_mut() {
                if seed[1] == 0 {
                    continue
                }

                *seed = r.split_to(&mut next, *seed);
            }
        }

        next.extend(curr.iter().filter(|[_, l]| *l > 0).copied());
        next.iter().map(|[start, _]| *start).min().unwrap()
    }

    #[allow(dead_code)]
    fn lowest_location_range_cpu_go_brr(&self) -> u64 {
        let mut winner = u64::MAX;

        for [seed_start, seed_count] in self.seeds.array_chunks::<2>() {
            let mut pending = false;

            for seed in *seed_start..(*seed_start + *seed_count) {
                let mut value = seed;
                for Range(next, dest, src, len) in self.ranges.iter() {
                    if pending {
                        if *next {
                            pending = false;
                        } else {
                            continue
                        }
                    }
                    if value < *src {
                        continue
                    }

                    let diff = value - *src;
                    if diff >= *len {
                        continue
                    }

                    value = *dest + diff;
                    pending = true;
                }

                if value < winner {
                    winner = value;
                }
            }
        }

        winner
    }

    #[allow(dead_code)]
    fn lowest_location_rev(&self) -> u64 {
        for location in 0.. {
            let mut value = location;
            let mut pending = false;

            for Range(next, src, dest, len) in self.ranges.iter().rev() {
                if pending {
                    if *next {
                        pending = false;
                        continue
                    } else {
                        continue
                    }
                }
                if value < *src {
                    continue
                }

                let diff = value - *src;
                if diff >= *len {
                    continue
                }

                value = *dest + diff;
                pending = true;
            }

            if self.seeds.array_chunks::<2>().find(|[n, l]| value >= *n && value < (*n + *l)).is_some() {
                return location
            }
        }

        unreachable!()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        let range_parser = parse::unsigned_int()
            .delimited_by(b' ')
            .repeat_n::<[u64; 3]>(3)
            .then_skip(b'\n');

        b"seeds: "
            .and_instead(
                parse::unsigned_int().delimited_by(b' ').repeat()
            )
            .then_skip(b'\n')
            .and(
                b'\n'
                    .and_discard(parse::line())
                    .and_instead(range_parser)
                    .map(|[a,b,c]| Range(true, a, b, c))
                    .or(
                        range_parser.map(|[a,b,c]| Range(false, a, b, c))
                    )
                    .repeat()
            )
            .map(|(seeds, ranges)| Almanac{
                seeds, ranges
            })
    }
}

#[derive(Debug)]
struct Range (bool, u64, u64, u64);

impl Range {
    #[allow(dead_code)]
    fn split(&self, [start, len]: [u64; 2]) -> ([u64; 2], Vec<[u64; 2]>) {
        let mut res = Vec::with_capacity(4);
        let rem = self.split_to(&mut res, [start, len]);

        (rem, res)
    }

    fn split_to(&self, next: &mut Vec<[u64; 2]>, [start, len]: [u64; 2]) -> [u64; 2] {
        let Range(_, dst, src, range_len) = self;

        if start + len <= *src || start >= *src + *range_len {
            // Entirely outside.

            [start, len]
        } else if start >= *src && (start - *src + len) <= *range_len {
            // Entirely inside.
            next.push([*dst + (start - *src), len]);

            [start, 0]
        } else if start <= *src && (start + len) > (*src + *range_len) {
            // Covers entirely.
            next.push([start, *src - start]);
            next.push([*dst, *range_len]);

            [*src + *range_len, len - *range_len - (*src - start)]
        } else if start <= *src {
            // Overlaps left.
            next.push([*dst, len - (*src - start)]);

            [start, *src - start]
        } else {
            // Overlaps right.
            next.push([dst + (start - *src), (*src + *range_len) - start]);

            [*src + *range_len, len - ((*src + *range_len) - start)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_split_works() {
        let r = Range(false, 200, 100, 50);

        assert_eq!(r.split([0, 40]), ([0, 40], vec![]));
        assert_eq!(r.split([50, 50]), ([50, 50], vec![]));
        assert_eq!(r.split([50, 51]), ([50, 50], vec![[200, 1]]));
        assert_eq!(r.split([50, 125]), ([150, 25], vec![[50, 50], [200, 50]]));
        assert_eq!(r.split([100, 50]), ([100,0], vec![[200, 50]]));
        assert_eq!(r.split([125, 10]), ([125,0], vec![[225, 10]]));
        assert_eq!(r.split([140, 10]), ([140,0], vec![[240, 10]]));
        assert_eq!(r.split([140, 11]), ([150,1], vec![[240, 10]]));
    }

    #[test]
    fn part2_example() {
        let almanac = Almanac::parser().parse(P1_EXAMPLE).unwrap();

        assert_eq!(almanac.lowest_location_range(), almanac.lowest_location_range_cpu_go_brr());
    }

    const P1_EXAMPLE: &[u8] = b"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}