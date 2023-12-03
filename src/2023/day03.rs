use common::aoc::Day;
use common::grid::Grid;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let schematic = day.prep("Parse", || Schematic::parse(input));

    day.note("Width", schematic.map.width());
    day.note("Height", schematic.map.height());
    day.note("Numbers", schematic.next_index);

    day.part("Part 1", || schematic.part_number_sum());
    day.part("Part 2", || schematic.gear_ratios_sum());
}

struct Schematic {
    map: Grid<Cell, Vec<Cell>>,
    next_index: u16,
}

impl Schematic {
    fn part_number_sum(&self) -> u32 {
        let mut last_index = self.next_index;
        let mut sum = 0;

        for y in 0..self.map.height() {
            let above_range = if y == 0 { 0 } else { 1 };
            let below_range = if y == self.map.height() - 1 { 0 } else { 1 };

            for x in 0..self.map.width() {
                let left_range = if x == 0 { 0 } else { 1 };
                let right_range = if x == self.map.width() - 1 { 0 } else { 1 };

                if let Cell::Digit(i, v) = self.map[(x, y)] {
                    if i == last_index {
                        continue;
                    }

                    'check_loop: for y2 in y - above_range..=y + below_range {
                        for x2 in x - left_range..=x + right_range {
                            if x == x2 && y == y2 {
                                continue;
                            }

                            if let Cell::Symbol(_) = self.map[(x2, y2)] {
                                last_index = i;
                                sum += v as u32;
                                break 'check_loop;
                            }
                        }
                    }
                }
            }
        }

        sum
    }

    fn gear_ratios_sum(&self) -> u32 {
        let mut sum = 0;

        for y in 0..self.map.height() {
            let above_range = if y == 0 { 0 } else { 1 };
            let below_range = if y == self.map.height() - 1 { 0 } else { 1 };

            for x in 0..self.map.width() {
                let left_range = if x == 0 { 0 } else { 1 };
                let right_range = if x == self.map.width() - 1 { 0 } else { 1 };

                if let Cell::Symbol(s) = self.map[(x, y)] {
                    if s != b'*' {
                        continue;
                    }

                    let mut last_index = self.next_index;
                    let mut count = 0;
                    let mut ratio = 1;

                    'check_loop: for y2 in y - above_range..=y + below_range {
                        for x2 in x - left_range..=x + right_range {
                            if x == x2 && y == y2 {
                                continue;
                            }

                            if let Cell::Digit(i, v) = self.map[(x2, y2)] {
                                if last_index == i {
                                    continue;
                                }
                                count += 1;
                                if count == 3 {
                                    break 'check_loop;
                                }

                                last_index = i;
                                ratio *= v as u32;
                            }
                        }
                    }

                    if count == 2 {
                        sum += ratio;
                    }
                }
            }
        }

        sum
    }

    fn collapse_digit(&mut self, v: u16, y: usize, x1: usize, x2: usize) {
        for x in x1..x2 {
            self.map[(x, y)] = Cell::Digit(self.next_index, v);
        }

        self.next_index += 1;
    }

    fn collapse_digits(&mut self) {
        let mut d_start = 0;
        let mut d_value = 0;

        let grid_width = self.map.width();
        let grid_height = self.map.height();

        for y in 0..grid_height {
            for x in 0..grid_width {
                let found_num = if let Cell::Symbol(s) = self.map[(x, y)] {
                    if s >= b'0' && s <= b'9' {
                        if d_value == 0 {
                            d_start = x;
                            d_value = (s - b'0') as u16
                        } else {
                            d_value *= 10;
                            d_value += (s - b'0') as u16
                        }

                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !found_num && d_value > 0 {
                    self.collapse_digit(d_value, y, d_start, x);
                    d_value = 0;
                }
            }

            if d_value > 0 {
                self.collapse_digit(d_value, y, d_start, grid_width);
                d_value = 0;
            }
        }
    }

    fn parse(input: &[u8]) -> Schematic {
        let mut schematic = Schematic {
            map: Grid::parser(parse::any_byte().map(|b| match b {
                b'.' => Cell::Blank,
                _ => Cell::Symbol(b),
            }))
            .parse(input)
            .unwrap(),
            next_index: 0,
        };

        schematic.collapse_digits();

        schematic
    }
}

#[derive(Default, Copy, Clone, Debug)]
enum Cell {
    #[default]
    Blank,
    Symbol(u8),
    Digit(u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;
}
