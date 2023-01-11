use std::thread::sleep;
use std::time::Duration;
use common::aoc::AOC;

fn main() {
    let aoc = AOC::new(2022);
    aoc.run_day(18, |day, input| {
        day.part("Part 1", || {
            sleep(Duration::new(0, 14433444));
            input.len()
        });
    });
    aoc.run_day(17, |day, input| {
        day.part("Part 1", || {
            sleep(Duration::new(0, 14433444));
            input.len()
        });
    });
    aoc.run_day(19, |day, input| {
        day.note("Input length", input.len());

        day.prep("Parse", || {
            sleep(Duration::new(0, 536534));
            input.len()
        });

        day.part("Part 1", || {
            sleep(Duration::new(0, 16533444));
            input.len()
        });

        day.part("Part 2", || {
            sleep(Duration::new(0, 985133444));
            input.len()
        });

        day.branch_from("Part 1");
        day.part("Part 2 (Alt)", || {
            sleep(Duration::new(0, 445133444));
            input.len()
        });

        day.branch_from("Part 1");
        day.part("Part 2 (Alt 2)", || {
            sleep(Duration::new(0, 35133444));
            input.len()
        });
        day.part("Part 2 (Alt 2.2)", || {
            sleep(Duration::new(0, 75133444));
            input.len()
        });

        day.branch_extra();
        day.part("Something Else Not Counted", || {
            sleep(Duration::new(0, 44133444));
            input.len()
        });
    });
}