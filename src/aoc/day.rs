use std::fmt::Display;
use time::Instant;
use crate::aoc::utils::format_duration;
use crate::ds::Graph;
use crate::search::{Search, dfs};

pub struct Day {
    graph: Graph<&'static str, (String, i64), (), 16>,
    notes: Vec<(&'static str, String)>,
    tail: Option<usize>,
    day: u32,
    run_once: bool,
}

impl Day {
    pub fn new(day: u32, run_once: bool) -> Self {
        let mut graph = Graph::with_capacity(8);
        graph.create_node("$$root$$", ("$$run_parse$$".to_owned(), 0));
        graph.create_node("$$dead_end$$", ("$$run_parse$$".to_owned(), 0));

        Self {
            graph,
            notes: Vec::with_capacity(8),
            tail: Some(0),
            day,
            run_once,
        }
    }

    pub fn branch_from(&mut self, label: &'static str) {
        self.tail = Some(self.graph.find(&label).unwrap());
    }

    pub fn branch_extra(&mut self) {
        self.tail = None;
    }

    pub fn print_table(&self) {
        print!("Day {:02}  ", self.day);
        let (steps, total) = self.shortest_time();

        print!("{: >10} |", format_duration(total));
        for step in steps.iter() {
            print!("{: >10}", format_duration(*step));
        }

        println!();
    }

    pub fn print_list(&self, show_times: bool) {
        println!("--- Day {} ------", self.day);
        if self.notes.len() > 0 {
            println!("NOTES");
            for (label, res) in self.notes.iter() {
                if res.chars().find(|c| *c == '\n').is_some() {
                    println!("  {}:\n{}", label, res);
                } else {
                    println!("  {}: {}", label, res);
                }
            }
            println!();
        }

        println!("RESULTS");
        for i in 0..self.graph.len() {
            let (label, (res, _)) = self.graph.node(i).unwrap();
            if res.as_str() == "$$run_parse$$" {
                continue;
            }

            if res.chars().find(|c| *c == '\n').is_some() {
                println!("  {}:\n{}", label, res);
            } else {
                println!("  {}: {}", label, res);
            }
        }
        println!();

        if show_times {
            println!("TIMES");
            for i in 0..self.graph.len() {
                let (label, (_, dur)) = self.graph.node(i).unwrap();
                if label.starts_with("$$") {
                    continue;
                }

                println!("  {}: {}", label, format_duration(*dur));
            }
            let (_, shortest_time) = self.shortest_time();
            println!("  Total: {}", format_duration(shortest_time));
            println!();
        }
    }

    pub fn shortest_time(&self) -> (Vec<i64>, i64) {
        if self.graph.len() == 0 {
            return (Vec::new(), i64::MAX);
        }

        let mut search = dfs::<(usize, Vec<i64>, i64)>();
        search.push_state((0, Vec::new(), 0));

        let results: Vec<_> = search.gather(|s, (index, mut steps, total)| {
            let (label, (_, dur)) = self.graph.node(index).unwrap();
            if *label == "$$dead_end$$" {
                return None;
            }

            if index > 0 {
                steps.push(*dur);
            }
            let total_dur = total + *dur;
            let mut had_edges = false;
            for (_, next_index, _) in self.graph.edges_from(index) {
                had_edges = true;
                s.push_state((*next_index, steps.clone(), total_dur));
            }

            if !had_edges {
                Some((steps, total_dur))
            } else {
                None
            }
        });

        results.into_iter().min_by_key(|(_, total)| *total).unwrap().clone()
    }

    pub fn note<T>(&mut self, label: &'static str, value: T) where T: Display {
        self.notes.push((label, value.to_string()))
    }

    pub fn mark_dead_end(&mut self) {
        self.graph.connect(self.tail.unwrap(), 1, ());
        self.tail = Some(0);
    }

    pub fn part<F, T>(&mut self, label: &'static str, f: F) -> T where F: Fn() -> T, T: Display {
        let (res, dur) = self.run(f);

        let new_tail = self.graph.create_node(label, (res.to_string(), dur));
        if let Some(tail) = self.tail {
            self.graph.connect(tail, new_tail, ());
        }

        self.tail = Some(new_tail);

        res
    }

    pub fn prep<F, T>(&mut self, label: &'static str, f: F) -> T where F: Fn() -> T {
        let (res, dur) = self.run(f);

        let new_tail = self.graph.create_node(label, (String::from("$$run_parse$$"), dur));
        if let Some(tail) = self.tail {
            self.graph.connect(tail, new_tail, ());
        }

        self.tail = Some(new_tail);

        res
    }

    fn run<F, T>(&mut self, f: F) -> (T, i64) where F: Fn() -> T {
        let before = Instant::now();
        let res = f();
        let after = Instant::now();
        let dur = after - before;

        let runs: u32 = match dur.whole_milliseconds() {
            0 => 2500,
            1 => 1000,
            1..=9 => 100,
            10..=19 => 50,
            20..=49 => 20,
            50..=99 => 10,
            100..=299 => 4,
            300..=499 => 2,
            _ => 0
        };
        let mut dur = dur.whole_nanoseconds() as i64;

        if runs > 0 && !self.run_once {
            let before = Instant::now();
            for _ in 0..runs { f(); }
            let after = Instant::now();

            dur = ((after - before).whole_nanoseconds() as i64) / (runs as i64);
        }

        (res, dur)
    }
}
