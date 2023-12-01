use common::aoc::AOC;

mod day01;

fn main() {
    let aoc = AOC::new(2023);

    aoc.run_day(01, day01::main);
}