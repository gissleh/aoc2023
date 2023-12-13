use common::aoc::Day;
use common::parse;
use common::parse::Parser;
use rustc_hash::FxHashMap;

const FUNCTIONAL: u8 = b'.';
const BROKEN: u8 = b'#';

pub fn main(day: &mut Day, input: &[u8]) {
    let lines = day.prep("Parse", || Line::parse(input));

    day.note("Input length", lines.len());

    day.part("Part 1", || p1(&lines));
    let lines: Vec<Line> = day.prep("Unfold", || lines.iter().map(|l| l.unfold()).collect());
    day.part("Part 2", || p1(&lines));
}

fn p1(lines: &[Line]) -> u64 {
    let mut checker = Checker::new();

    lines
        .iter()
        .map(move |l| checker.run(&l.template, &l.rules, 0))
        .sum()
}

struct Line {
    template: Vec<u8>,
    rules: Vec<u8>,
}

impl Line {
    fn unfold(&self) -> Line {
        let mut line = Line {
            template: Vec::with_capacity(self.template.len() * 5 + 5),
            rules: Vec::with_capacity(self.rules.len() * 5),
        };

        for _ in 0..4 {
            line.template.extend_from_slice(&self.template);
            line.template.push(b'?');
            line.rules.extend_from_slice(&self.rules);
        }
        line.template.extend_from_slice(&self.template);
        line.rules.extend_from_slice(&self.rules);

        line
    }

    fn parse(input: &[u8]) -> Vec<Line> {
        parse::everything()
            .capped_by(b' ')
            .map(|l| l.iter().copied().collect())
            .and(parse::unsigned_int().delimited_by(b',').repeat())
            .map(|(template, rules)| Self { template, rules })
            .and_discard(b'\n')
            .repeat()
            .parse(input)
            .unwrap()
    }
}

struct Checker {
    key_buf: Vec<u8>,
    cache: FxHashMap<Vec<u8>, u64>,
}

impl Checker {
    fn satisfies(arr: &[u8], rule: u8) -> MatchResult {
        let rule = rule as usize;
        if arr.len() < rule {
            return MatchResult::Lost;
        }

        let mut broken = false;
        for i in 0..rule {
            if arr[i] == FUNCTIONAL {
                return if broken {
                    MatchResult::Lost
                } else {
                    MatchResult::None
                };
            }
            if arr[i] == BROKEN {
                broken = true;
            }
        }

        let mut offset = rule;
        if rule < arr.len() {
            if arr[rule] == BROKEN {
                return if arr[0] == BROKEN {
                    MatchResult::Lost
                } else {
                    MatchResult::None
                };
            }

            offset += 1;
        }

        if arr[0] == BROKEN {
            MatchResult::Locked(&arr[offset..])
        } else {
            MatchResult::Open(&arr[offset..])
        }
    }

    #[allow(dead_code)]
    fn run_isolated(curr: &[u8], rules: &[u8]) -> u64 {
        Self::new().run(curr, rules, 0)
    }

    fn new() -> Self {
        Self {
            cache: FxHashMap::default(),
            key_buf: Vec::with_capacity(128),
        }
    }

    fn run(&mut self, curr: &[u8], rules: &[u8], level: u32) -> u64 {
        if level > 2 {
            self.key_buf.clear();
            self.key_buf.extend_from_slice(curr);
            self.key_buf.push(0);
            self.key_buf.extend_from_slice(rules);
            if let Some(v) = self.cache.get(&self.key_buf) {
                return *v;
            }
        }

        let v = if rules.len() == 0 {
            if curr.contains(&BROKEN) {
                0
            } else {
                1
            }
        } else if (curr.len() as u8) < rules[0] {
            0
        } else {
            let mut count = 0;
            let mut curr = curr;

            while !curr.is_empty() {
                match Self::satisfies(curr, rules[0]) {
                    MatchResult::Open(next) => {
                        count += self.run(next, &rules[1..], level + 1);
                        curr = &curr[1..];
                    }
                    MatchResult::Locked(next) => {
                        count += self.run(next, &rules[1..], level + 1);
                        break;
                    }
                    MatchResult::None => {
                        curr = &curr[1..];
                    }
                    MatchResult::Lost => {
                        break;
                    }
                }
            }
            count
        };

        if level > 2 {
            self.key_buf.clear();
            self.key_buf.extend_from_slice(curr);
            self.key_buf.push(0);
            self.key_buf.extend_from_slice(rules);
            self.cache.insert(self.key_buf.clone(), v);
        }

        v
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MatchResult<'a> {
    Locked(&'a [u8]),
    Open(&'a [u8]),
    None,
    Lost,
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn satisfies() {
        assert_eq!(
            Checker::satisfies(b"???.###", 1),
            MatchResult::Open(b"?.###")
        );
        assert_eq!(Checker::satisfies(b"?.###", 1), MatchResult::Open(b"###"));
        assert_eq!(Checker::satisfies(b"###", 3), MatchResult::Locked(b""));
        assert_eq!(Checker::satisfies(b"?###????????", 3), MatchResult::None);
        assert_eq!(
            Checker::satisfies(b"###????????", 3),
            MatchResult::Locked(b"???????".as_slice())
        );
        assert_eq!(
            Checker::satisfies(b"???????", 2),
            MatchResult::Open(b"????".as_slice())
        );

        assert_eq!(Checker::satisfies(b".##.?#??.#.?#", 2), MatchResult::None);
        assert_eq!(
            Checker::satisfies(b"##.?#??.#.?#", 2),
            MatchResult::Locked(b"?#??.#.?#".as_slice())
        );
        assert_eq!(Checker::satisfies(b"?#??.#.?#", 1), MatchResult::None);
        assert_eq!(
            Checker::satisfies(b"#??.#.?#", 1),
            MatchResult::Locked(b"?.#.?#".as_slice())
        );
        assert_eq!(
            Checker::satisfies(b"?.#.?#", 1),
            MatchResult::Open(b"#.?#".as_slice())
        );
        assert_eq!(
            Checker::satisfies(b"#.?#", 1),
            MatchResult::Locked(b"?#".as_slice())
        );
        assert_eq!(Checker::satisfies(b"?#", 1), MatchResult::None);
        assert_eq!(
            Checker::satisfies(b"#", 1),
            MatchResult::Locked(b"".as_slice())
        );
    }

    #[test]
    fn counting_works_on_example() {
        assert_eq!(Checker::run_isolated(b"???.###", &[1, 1, 3]), 1);
        assert_eq!(Checker::run_isolated(b".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(Checker::run_isolated(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(Checker::run_isolated(b"????.#...#...", &[4, 1, 1]), 1);
        assert_eq!(Checker::run_isolated(b"????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(Checker::run_isolated(b"?###????????", &[3, 2, 1]), 10);
        assert_eq!(Checker::run_isolated(b"??#.?#?#???", &[1, 3, 1]), 2);
        assert_eq!(Checker::run_isolated(b"??##?#?????..", &[5, 1]), 7);
        assert_eq!(Checker::run_isolated(b"#??????????", &[1, 1, 7]), 1);
        assert_eq!(
            Checker::run_isolated(b"#??#????.?##??#????.", &[1, 4, 1, 3, 1, 3]),
            2
        );
        assert_eq!(Checker::run_isolated(b"?????#???????.", &[5, 5]), 3);
        assert_eq!(Checker::run_isolated(b".##.?#??.#.?#", &[2, 1, 1, 1]), 1);
    }

    #[test]
    fn p1_works_on_example() {
        let lines = Line::parse(P1_EXAMPLE);
        assert_eq!(p1(&lines), 21);
    }
}
