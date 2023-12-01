use common::aoc::Day;
use common::parse;
use common::parse::{any_byte, choice, digit, Parser};

pub fn main(day: &mut Day, input: &[u8]) {
    let list = day.prep("Parse", || parse_list(input));

    day.part("P1", || p1(&list));
    day.part("P2", || p2(&list));
}

fn p1(list: &[&[u8]]) -> u32 {
    list.iter()
        .map(|l| l.iter()
            .filter(|c| **c > b'0' && **c <= b'9')
            .fold([u32::MAX, 0], |v, c| {
                let c = (c - b'0') as u32;
                if v[0] == u32::MAX {
                    [c * 10, c]
                } else {
                    [v[0], c]
                }
            }))
        .map(|a| a[0] + a[1])
        .sum()
}

fn p2(list: &[&[u8]]) -> u64 {
    let num_parser = digit()
        .or(choice((
            b"on".as_slice().and_discard(b'e'.rewind()).map_to(1),
            b"tw".as_slice().and_discard(b'o'.rewind()).map_to(2),
            b"thre".as_slice().and_discard(b'e'.rewind()).map_to(3),
            b"four".as_slice().map_to(4),
            b"fiv".as_slice().and_discard(b'e'.rewind()).map_to(5),
            b"six".as_slice().map_to(6),
            b"seve".as_slice().and_discard(b'n'.rewind()).map_to(7),
            b"eigh".as_slice().and_discard(b't'.rewind()).map_to(8),
            b"nin".as_slice().and_discard(b'e'.rewind()).map_to(9),
        )))
        .or(any_byte().map_to(0));

    list.iter()
        .map(|l| num_parser.parse_iter(l)
            .filter(|d| *d > 0)
            .fold([0, 0], |v, d| {
                if v[0] == 0 {
                    [d * 10, d]
                } else {
                    [v[0], d]
                }
            }))
        .map(|a| {
            a[0] + a[1]
        })
        .sum()
}

fn parse_list(input: &[u8]) -> Vec<&[u8]> {
    parse::line()
        .only_if(|v| !v.is_empty())
        .repeat()
        .parse(input)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const P2_EXAMPLE: &[u8]= b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    const P2_REDDIT_EXAMPLE: &[u8]= b"eighthree
sevenine
";


    #[test]
    fn p1_testcase() {
        let parsed = parse_list(P1_EXAMPLE);
        assert_eq!(p1(&parsed), 142)
    }

    #[test]
    fn p2_with_p1_testcase() {
        let parsed = parse_list(P1_EXAMPLE);
        assert_eq!(p2(&parsed), 142)
    }

    #[test]
    fn p2_testcase() {
        let parsed = parse_list(P2_EXAMPLE);
        assert_eq!(p2(&parsed), 281)
    }

    #[test]
    fn p2_reddit_testcase() {
        let parsed = parse_list(P2_REDDIT_EXAMPLE);
        assert_eq!(p2(&parsed), 83+79)
    }
}