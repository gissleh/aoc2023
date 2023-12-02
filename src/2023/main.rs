use common::aoc::AOC;

mod day01;
mod day02;

fn main() {
    let aoc = AOC::new(2023);

    aoc.run_day(1, day01::main);
    aoc.run_day(2, day02::main);
}