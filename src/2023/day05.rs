use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Almanac::parser().parse(input).unwrap());

    day.note("Amount of seeds", input.seeds.len());
    day.note("Amount of ranges", input.ranges.len());

    day.part("Part 1", || input.lowest_location());
    day.part("Part 2", || input.lowest_location_range_cpu_go_brr());
}


struct Almanac {
    seeds: Vec<u32>,
    ranges: Vec<Range>,
}

impl Almanac {
    fn lowest_location(&self) -> u32 {
        let mut winner = u32::MAX;

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

    fn lowest_location_range_cpu_go_brr(&self) -> u32 {
        let mut winner = u32::MAX;

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

    fn parser<'i>() -> impl Parser<'i, Self> {
        let range_parser = parse::unsigned_int()
            .delimited_by(b' ')
            .repeat_n::<[u32; 3]>(3)
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

struct Range (bool, u32, u32, u32);

#[cfg(test)]
mod tests {
    use super::*;
}