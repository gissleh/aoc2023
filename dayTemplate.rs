use common::aoc::Day;
use common::parse;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse_stuff(input));

    day.note("Input note", input);

    //day.part("Part 1", || 0);
    //day.part("Part 2", || 0;
}

fn parse_stuff(input: &[u8]) -> u32 {
    b"Some Text "
        .and_instead(parse::digit())
}

#[cfg(test)]
mod tests {
    use super::*;
}