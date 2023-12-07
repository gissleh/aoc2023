#![feature(array_chunks)]

use common::aoc::AOC;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let aoc = AOC::new(2023);

    aoc.run_day(1, day01::main);
    aoc.run_day(2, day02::main);
    aoc.run_day(3, day03::main);
    aoc.run_day(4, day04::main);
    aoc.run_day(5, day05::main);
    aoc.run_day(6, day06::main);
    aoc.run_day(7, day07::main);
}
