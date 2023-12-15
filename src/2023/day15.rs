use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    day.note("Input size", input.len());

    day.part("Part 1", || p1(input));
    let input = day.prep("Parse", || parse(input));
    day.note("Hash ops", input.len());
    day.part("Part 2", || p2(&input));
}

fn parse(input: &[u8]) -> Vec<HashOp> {
    parse::word()
        .and(parse::any_byte())
        .and(parse::unsigned_int().or_return(0u8))
        .map(|((key, op), value)| match op {
            b'=' => HashOp::Set(key, value),
            b'-' => HashOp::Remove(key),
            _ => panic!("Illegal operation {}", op),
        })
        .delimited_by(b',')
        .repeat()
        .parse(input)
        .unwrap()
}

fn p1(input: &[u8]) -> u32 {
    let mut curr = 0u8;
    let mut sum = 0u32;

    for v in input.iter() {
        if *v == b',' || *v == b'\n' {
            sum += curr as u32;
            curr = 0;
            continue;
        }

        curr = curr.wrapping_add(*v).wrapping_mul(17);
    }

    sum
}

fn p2<'i>(input: &'i [HashOp]) -> usize {
    let mut boxes: Vec<Vec<BoxItem<'i>>> = vec![Vec::with_capacity(16); 256];

    for op in input.iter() {
        match op {
            HashOp::Set(key, value) => {
                let box_index = hash(*key) as usize;

                if let Some(item) = boxes[box_index].iter_mut().find(|i| i.key == *key) {
                    item.value = *value;
                } else {
                    boxes[box_index].push(BoxItem {
                        key: *key,
                        value: *value,
                    })
                }
            }
            HashOp::Remove(key) => {
                let box_index = hash(*key) as usize;

                if let Some(index) = boxes[box_index].iter().position(|i| i.key == *key) {
                    boxes[box_index].remove(index);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(p, b)| {
            (p + 1)
                * b.iter()
                    .enumerate()
                    .map(|(p, i)| (p + 1) * i.value as usize)
                    .sum::<usize>()
        })
        .sum()
}

fn hash(input: &[u8]) -> u8 {
    let mut curr = 0u8;
    for v in input.iter() {
        curr = curr.wrapping_add(*v).wrapping_mul(17);
    }

    curr
}

#[derive(Copy, Clone)]
struct BoxItem<'i> {
    key: &'i [u8],
    value: u8,
}

enum HashOp<'i> {
    Set(&'i [u8], u8),
    Remove(&'i [u8]),
}

#[cfg(test)]
mod tests {
    use super::*;
}
