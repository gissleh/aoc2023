#![feature(array_chunks)]

use common::aoc::AOC;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn main() {
    let aoc = AOC::new(2023);

    aoc.run_day(1, day01::main);
    aoc.run_day(2, day02::main);
    aoc.run_day(3, day03::main);
    aoc.run_day(4, day04::main);
    aoc.run_day(5, day05::main);
    aoc.run_day(6, day06::main);
    aoc.run_day(7, day07::main);
    aoc.run_day(8, day08::main);
    aoc.run_day(9, day09::main);
    aoc.run_day(10, day10::main);
    aoc.run_day(11, day11::main);
    aoc.run_day(12, day12::main);
    aoc.run_day(13, day13::main);
    aoc.run_day(14, day14::main);
    aoc.run_day(15, day15::main);
    //aoc.run_day(16, day16::main);
    //aoc.run_day(17, day17::main);
    //aoc.run_day(18, day18::main);
    //aoc.run_day(19, day19::main);
    //aoc.run_day(20, day20::main);
    //aoc.run_day(21, day21::main);
    //aoc.run_day(22, day22::main);
    //aoc.run_day(23, day23::main);
    //aoc.run_day(24, day24::main);
    //aoc.run_day(25, day25::main);
}
