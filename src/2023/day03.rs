use common::aoc::Day;
use common::grid::Grid;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let schematic = day.prep("Parse", || Schematic::parse(input));

    day.note("Width", schematic.map.width());
    day.note("Height", schematic.map.height());
    day.note("Numbers", schematic.next_index);
    day.note(
        "Parts",
        schematic
            .map
            .iter()
            .filter(|(_, c)| if let Cell::Symbol(_) = c { true } else { false })
            .count(),
    );

    day.part("Part 1", || schematic.part_number_sum());
    day.part("Part 2", || schematic.gear_ratios_sum());

    day.branch_from_root();

    let schematic2 = day.prep("Parse (NG)", || Schematic2::parse(input));

    day.note("Numbers (NG)", schematic2.numbers.len());
    day.note("Parts (NG)", schematic2.parts.len());
    day.note("Height (NG)", schematic2.height);

    day.part("Part 1 (NG)", || schematic2.part_number_sum());
    day.part("Part 2 (NG)", || schematic2.gear_ratio_sum());
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

struct Schematic2 {
    parts: Vec<PartName>,
    numbers: Vec<PartNumber>,
    lens: Vec<(usize, usize)>,
    height: i16,
}

impl Schematic2 {
    fn part_number_sum(&self) -> u32 {
        let mut sum = 0;

        for y in 1..=self.height as usize {
            let lp1 = if y > 1 { self.lens[y - 2].0 } else { 0 };
            let (_, ln1) = self.lens[y - 1];
            let (_, ln2) = self.lens[y];
            let (lp2, _) = self.lens[y + 1];

            let parts = &self.parts[lp1..lp2];
            let numbers = &self.numbers[ln1..ln2];

            'part_loop: for PartNumber(nx, nw, nv) in numbers.iter() {
                for PartName(px, _) in parts.iter() {
                    if *px >= *nx - 1 && *px <= *nx + *nw {
                        sum += nv;
                        continue 'part_loop;
                    }
                }
            }
        }

        sum
    }

    fn gear_ratio_sum(&self) -> u32 {
        let mut sum = 0;

        for y in 1..=self.height as usize {
            let ln1 = if y > 1 { self.lens[y - 2].1 } else { 0 };
            let (lp1, _) = self.lens[y - 1];
            let (lp2, _) = self.lens[y];
            let (_, ln2) = self.lens[y + 1];

            let parts = &self.parts[lp1..lp2];
            let numbers = &self.numbers[ln1..ln2];

            'part_loop: for PartName(px, pn) in parts.iter() {
                if *pn != b'*' {
                    continue;
                }

                let mut left = 0u32;
                let mut right = 0u32;
                for PartNumber(nx, nw, nv) in numbers.iter() {
                    if *px >= *nx - 1 && *px <= *nx + *nw {
                        if left == 0 {
                            left = *nv;
                        } else if right == 0 {
                            right = *nv;
                        } else {
                            continue 'part_loop;
                        }
                    }
                }

                if right != 0 {
                    sum += left * right;
                }
            }
        }

        sum
    }

    fn parse(input: &[u8]) -> Self {
        let mut parts = Vec::with_capacity(160);
        let mut numbers = Vec::with_capacity(160);
        let mut lens = Vec::with_capacity(160);
        let mut dv = 0;
        let mut dx = 0;
        let mut y = 0;
        let mut x = 0;

        lens.push((0, 0));

        for ch in input.iter() {
            match *ch {
                b'\n' => {
                    if dv > 0 {
                        numbers.push(PartNumber(dx, x - dx, dv));
                        dv = 0;
                    }

                    lens.push((parts.len(), numbers.len()));

                    if x == 0 {
                        break;
                    }

                    y += 1;
                    x = 0;
                }

                b'.' => {
                    if dv > 0 {
                        numbers.push(PartNumber(dx, x - dx, dv));
                        dv = 0;
                    }

                    x += 1;
                }

                b'0'..=b'9' => {
                    if dv > 0 {
                        dv *= 10;
                        dv += (*ch - b'0') as u32;
                    } else {
                        dx = x;
                        dv = (*ch - b'0') as u32;
                    }

                    x += 1;
                }

                _ => {
                    if dv > 0 {
                        numbers.push(PartNumber(dx, x - dx, dv));
                        dv = 0;
                    }

                    parts.push(PartName(x, *ch));

                    x += 1;
                }
            }
        }

        lens.push((parts.len(), numbers.len()));
        Schematic2 {
            parts,
            numbers,
            lens,
            height: y,
        }
    }
}

#[derive(Debug)]
struct PartName(i16, u8);
#[derive(Debug)]
struct PartNumber(i16, i16, u32);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn p1ng_works_on_example() {
        let schematic = Schematic2::parse(EXAMPLE);

        assert_eq!(schematic.part_number_sum(), 4361);
    }

    #[test]
    fn p2ng_works_on_example() {
        let schematic = Schematic2::parse(EXAMPLE);

        assert_eq!(schematic.gear_ratio_sum(), 467835);
    }
}
