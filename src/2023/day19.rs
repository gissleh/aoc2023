use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Input::parse(input));

    day.note(
        "Workflows",
        input
            .workflows
            .iter()
            .filter(|w| !w.rules.is_empty())
            .count(),
    );
    day.note("Parts", input.parts.len());

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(input: &Input) -> u32 {
    let mut sum = 0;
    let start = name_to_index(b"in");

    for part in input.parts.iter() {
        let mut curr = start;

        'part_loop: loop {
            for rule in input.workflows[curr].rules.iter() {
                let matched = match rule.condition {
                    Condition::None => true,
                    Condition::Less(i, v) => part.values[i] < v,
                    Condition::Greater(i, v) => part.values[i] > v,
                };

                if matched {
                    match rule.outcome {
                        Outcome::Accept => {
                            sum += part.values.iter().map(|v| *v as u32).sum::<u32>();
                            break 'part_loop;
                        }
                        Outcome::Reject => {
                            break 'part_loop;
                        }
                        Outcome::SendTo(next) => {
                            curr = next;
                            continue 'part_loop;
                        }
                    }
                }
            }
        }
    }

    sum
}

fn p2(input: &Input) -> u64 {
    let mut total = 0;
    let mut stack = Vec::with_capacity(128);
    stack.push((name_to_index(b"in"), Range{
        min: [1,1,1,1],
        max: [4001,4001,4001,4001],
    }));

    #[cfg(test)]
    assert_eq!(stack[0].1.volume(), 4000*4000*4000*4000);

    while let Some((curr, mut range)) = stack.pop() {
        for rule in input.workflows[curr].rules.iter() {
            let (matched, remainder) = range.split(rule.condition);

            #[cfg(test)]
            println!("cond={:?} matched={:?} rem={:?}", rule.condition, matched, remainder);

            if let Some(matched) = matched {
                match rule.outcome {
                    Outcome::Accept => {
                        #[cfg(test)]
                        println!("\taccepted={:?} volume={:?}", matched, matched.volume());

                        total += matched.volume()
                    }
                    Outcome::Reject => {}
                    Outcome::SendTo(next) => { stack.push((next, matched)) }
                }
            }

            if let Some(remainder) = remainder {
                range = remainder
            } else {
                break;
            }
        }
    }

    total
}

#[derive(Copy, Clone, Debug)]
struct Range {
    min: [u16; 4],
    max: [u16; 4],
}

impl Range {
    fn volume(&self) -> u64 {
        self.min.iter()
            .zip(self.max.iter())
            .map(|(min, max)| (*max - *min) as u64)
            .product()
    }

    fn split(&self, cond: Condition) -> (Option<Range>, Option<Range>) {
        match cond {
            Condition::None => (Some(*self), None),
            Condition::Greater(i, v) => {
                if v >= self.max[i] {
                    (None, Some(*self))
                } else if v <= self.min[i] {
                    (Some(*self), None)
                } else {
                    let mut a = *self;
                    let mut b = *self;
                    a.min[i] = v + 1;
                    b.max[i] = v + 1;
                    (Some(a), Some(b))
                }
            }
            Condition::Less(i, v) => {
                if v <= self.min[i] {
                    (None, Some(*self))
                } else if v >= self.max[i] {
                    (Some(*self), None)
                } else {
                    let mut a = *self;
                    let mut b = *self;
                    a.max[i] = v;
                    b.min[i] = v;
                    (Some(a), Some(b))
                }
            }
        }
    }
}

struct Input {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl Input {
    fn parse(input: &[u8]) -> Self {
        Self::parser().parse(input).unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        Workflow::parser()
            .delimited_by(b'\n')
            .repeat_fold(
                || vec![Workflow::default(); 26 * 26 * 26],
                |mut workflows, (i, workflow)| {
                    workflows[i] = workflow;
                    workflows
                },
            )
            .then_skip_all(b'\n')
            .and(Part::parser().delimited_by(b'\n').repeat())
            .map(|(workflows, parts)| Input { workflows, parts })
    }
}

#[derive(Default, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parser<'i>() -> impl Parser<'i, (usize, Self)> {
        parse::word()
            .map(name_to_index)
            .and(
                Rule::parser()
                    .delimited_by(b',')
                    .repeat()
                    .quoted_by(b'{', b'}'),
            )
            .map(|(index, rules)| (index, Workflow { rules }))
    }
}

#[derive(Clone, Default)]
struct Rule {
    condition: Condition,
    outcome: Outcome,
}

impl Rule {
    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::choice([
            b'x'.map_to(0),
            b'm'.map_to(1),
            b'a'.map_to(2),
            b's'.map_to(3),
        ])
        .and(b'>'.or(b'<'))
        .and(parse::unsigned_int())
        .and_discard(b':')
        .map(|((i, op), v)| {
            if op == b'>' {
                Condition::Greater(i, v)
            } else {
                Condition::Less(i, v)
            }
        })
        .or_return(Condition::None)
        .and(parse::choice((
            b'A'.map_to(Outcome::Accept),
            b'R'.map_to(Outcome::Reject),
            parse::word().map(|name| Outcome::SendTo(name_to_index(name))),
        )))
        .map(|(condition, outcome)| Self { condition, outcome })
    }
}

#[derive(Copy, Clone, Default, Debug)]
enum Condition {
    #[default]
    None,
    Greater(usize, u16),
    Less(usize, u16),
}

#[derive(Copy, Clone, Default)]
enum Outcome {
    Accept,
    #[default]
    Reject,
    SendTo(usize),
}

struct Part {
    values: [u16; 4],
}

impl Part {
    fn parser<'i>() -> impl Parser<'i, Self> {
        b"{x="
            .and_instead(parse::unsigned_int())
            .and_discard(b",m=")
            .and(parse::unsigned_int())
            .and_discard(b",a=")
            .and(parse::unsigned_int())
            .and_discard(b",s=")
            .and(parse::unsigned_int())
            .and_discard(b'}')
            .map(|(((x, m), a), s)| Self {
                values: [x, m, a, s],
            })
    }
}

#[inline]
fn name_to_index(name: &[u8]) -> usize {
    name.iter().fold(0, |c, b| c * 26 + (*b - b'a') as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn p2_works_on_example() {
        assert_eq!(p2(&Input::parse(P1_EXAMPLE)), 167409079868000);
    }
}
