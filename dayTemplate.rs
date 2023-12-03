use common::aoc::Day;
use common::parse;

pub fn main(day: &mut Day, input: &[u8]) {
    let list = day.prep("Parse", || parse_stuff(input));

    day.note("Amount of games", list.len());
    day.note("Amount of ops", list.iter().map(|g| g.cubes.len()).sum::<usize>());

    //day.part("Part 1", || p1(&list));
    //day.part("Part 2", || p2(&list));
}

fn parse_stuff(input: &[u8]) -> u32 {
    b"Some Text "
        .and_instead(parse::digit())
}

#[cfg(test)]
mod tests {
    use super::*;
}