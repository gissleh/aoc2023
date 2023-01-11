use std::thread::sleep;
use std::time::Duration;
use common::aoc::AOC;

fn main() {
    let aoc = AOC::new(2022);
    aoc.run_day(18, |day, input| {
        day.run("Part 1", || {
            sleep(Duration::new(0, 14433444));
            input.len()
        });
    });
    aoc.run_day(17, |day, input| {
        day.run("Part 1", || {
            sleep(Duration::new(0, 14433444));
            input.len()
        });
    });
    aoc.run_day(19, |day, input| {
        day.note("Input length", input.len());

        day.run("Part 1", || {
            sleep(Duration::new(0, 16533444));
            input.len()
        });

        day.run("Part 2", || {
            sleep(Duration::new(0, 85133444));
            input.len()
        });

        day.branch_off("Part 1");
        day.run("Part 2 (Alt)", || {
            sleep(Duration::new(0, 45133444));
            input.len()
        });

        day.branch_off("Part 1");
        day.run("Part 2 (Alt 2)", || {
            sleep(Duration::new(0, 35133444));
            input.len()
        });
    });
}