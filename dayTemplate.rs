use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || input);

    day.note("Input length", input.len());

    //day.part("Part 1", || 0);
    //day.part("Part 2", || 0);
}



#[cfg(test)]
mod tests {
    use super::*;
}