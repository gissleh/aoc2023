use common::aoc::AOC;

mod day01;
mod day02;

fn main() {
    let aoc = AOC::new(2023);

    aoc.run_day(01, day01::main);
    aoc.run_day(02, day02::main);
}