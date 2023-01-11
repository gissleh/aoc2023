use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use chrono::Datelike;
pub use day::Day;
pub use utils::BothParts;

mod day;
mod utils;

pub struct AOC {
    year: u32,
    day: u32,
    run_once: bool,
    format_table: bool,
    bench: bool,
}

impl AOC {
    pub fn run_day<F>(&self, day_number: u32, cb: F) where F: Fn(&mut Day, &[u8]) -> () {
        if self.day != 0 && self.day != day_number {
            return;
        }

        let mut buf = Vec::with_capacity(2048);
        let file_name = format!("./input/{}/day_{:02}.txt", self.year, day_number);
        match File::open(file_name.clone()) {
            Ok(mut file) => {
                file.read_to_end(&mut buf).expect("Could not read file");
            }
            Err(_) => {
                let token = env!("AOC_SESSION");
                if token == "" {
                    panic!("Env is not set")
                }

                eprintln!("Downloading input for day {}...", day_number);

                create_dir_all(format!("./input/{}", self.year)).expect("Could not create dir");
                let data = reqwest::blocking::Client::builder()
                    .build().unwrap()
                    .get(format!("https://adventofcode.com/{}/day/{}/input", self.year, day_number))
                    .header("User-Agent", "AOC Runner (github.com/gissleh/aoc2023, by dev@gisle.me)")
                    .header("Authority", "adventofcode.com")
                    .header("Cookie", format!("session={}", env!("AOC_SESSION")))
                    .send().unwrap()
                    .bytes().unwrap();

                buf.extend(data.iter());

                let mut file = OpenOptions::new().write(true).create(true).open(file_name).expect("Could not open file");
                file.write_all(&buf).expect("Could not write file");
            }
        }

        let mut day = Day::new(day_number, self.run_once);
        cb(&mut day, buf.as_slice());

        if self.format_table {
            day.print_table();
        } else {
            day.print_list(self.bench);
        }
    }

    pub fn new(year: u32) -> AOC {
        let args: Vec<String> = std::env::args().collect();
        let day = args.get(1)
            .map(|v| v.parse::<u32>().unwrap())
            .or(Some(chrono::Local::now().day()))
            .unwrap();
        let op = args.get(2).cloned().or(Some(String::from("run"))).unwrap();

        AOC {
            run_once: op == "" || op == "run" || op == "bench_once" || op == "table_once",
            bench: op == "table_once" || op == "bench" || op == "table" || op == "bench_once",
            format_table: op == "table" || op == "table_once",

            year,
            day,
        }
    }
}

