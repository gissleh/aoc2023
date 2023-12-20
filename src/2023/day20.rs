use arrayvec::ArrayVec;
use common::aoc::Day;
use common::parse;
use common::parse::Parser;
use num::Integer;
use std::collections::VecDeque;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Machine::parse(input));

    day.note("Modules", input.modules.len());

    day.part("Part 1", || input.thousand_presses());
    day.part("Part 2", || input.rx_press_count());
}

#[derive(Clone)]
struct Machine {
    modules: Vec<Module>,
    rx: u8,
}

impl Machine {
    fn thousand_presses(&self) -> u32 {
        let mut rs = RunState::new(self);

        for _ in 0..1000 {
            rs.press(self);
        }

        rs.highs * rs.lows
    }

    fn rx_press_count(&self) -> u64 {
        let mut dependencies: Vec<usize> = Vec::with_capacity(16);
        let mut dep_stack: Vec<usize> = Vec::with_capacity(64);
        dep_stack.push(self.rx as usize);
        while let Some(index) = dep_stack.pop() {
            let current = &self.modules[index];
            let mut remainder_mask = 0;
            for i in 0..current.inputs.len() {
                let parent = &self.modules[current.inputs[i] as usize];

                if let ModuleKind::Conjunction = parent.kind {
                    dep_stack.push(current.inputs[i] as usize);
                } else {
                    remainder_mask |= 1 << i;
                }
            }

            if remainder_mask > 0 {
                dependencies.push(index);
            }
        }

        let mut lcm = 1;
        let mut rs = RunState::new(self);
        for n in 1.. {
            rs.press(self);

            while let Some(i) = dependencies
                .iter()
                .position(|index| rs.conj_signalled & 1 << index != 0)
            {
                lcm = lcm.lcm(&n);
                dependencies.swap_remove(i);
            }

            if dependencies.len() == 0 {
                break;
            }
        }

        lcm
    }

    fn parse(input: &[u8]) -> Self {
        Self::parser().parse(input).unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::take_while(|c| (c >= b'a' && c <= b'z') || c == b'&' || c == b'%')
            .and_discard(b" -> ")
            .and(
                parse::word()
                    .delimited_by(b", ")
                    .repeat::<ArrayVec<&[u8], 8>>(),
            )
            .delimited_by(b'\n')
            .repeat_fold(
                || (Vec::<Module>::with_capacity(128), [usize::MAX; 26 * 26 + 2]),
                |(mut modules, mut indices), (name, nexts)| {
                    if modules.len() == 0 {
                        modules.push(Module::default());
                        modules.push(Module::default());
                        indices[26 * 26] = 0;
                        indices[26 * 26 + 1] = 0;
                    }

                    let name_index = name_to_index(&name[1..]);
                    if indices[name_index] == usize::MAX {
                        indices[name_index] = modules.len();
                        modules.push(Module::default());
                    }
                    let module_index = indices[name_index];

                    modules[module_index].kind = match name[0] {
                        b'b' => ModuleKind::Broadcaster,
                        b'o' => ModuleKind::Output,
                        b'&' => ModuleKind::Conjunction,
                        b'%' => ModuleKind::FlipFlop,
                        _ => panic!("Unknown type"),
                    };

                    for next_name in nexts.iter() {
                        let next_name_index = name_to_index(next_name);
                        if indices[next_name_index] == usize::MAX {
                            indices[next_name_index] = modules.len();
                            modules.push(Module::default());
                        }
                        let next_module_index = indices[next_name_index];
                        let next_output_index = modules[next_module_index].inputs.len() as u8;

                        modules[module_index]
                            .outputs
                            .push((next_module_index as u8, next_output_index));
                        modules[next_module_index].inputs.push(module_index as u8);
                    }

                    (modules, indices)
                },
            )
            .map(|(modules, indices)| Self {
                modules,
                rx: indices[name_to_index(b"rx")] as u8,
            })
    }
}

struct RunState {
    states: Vec<u16>,
    conj_signalled: u64,
    queue: VecDeque<(u8, u8, bool)>,
    highs: u32,
    lows: u32,
}

impl RunState {
    fn press(&mut self, machine: &Machine) {
        self.conj_signalled = 0;
        self.queue.push_back((0u8, 0u8, false));

        while let Some((index, input, high)) = self.queue.pop_front() {
            let state = &mut self.states[index as usize];
            let module = &machine.modules[index as usize];

            match module.kind {
                ModuleKind::Broadcaster => {
                    for (index, output) in module.outputs.iter() {
                        self.queue.push_back((*index, *output, false));
                    }
                }
                ModuleKind::Output => {}
                ModuleKind::FlipFlop => {
                    if !high {
                        *state = !*state;
                        for (index, output) in module.outputs.iter() {
                            self.queue.push_back((*index, *output, *state != 0));
                        }
                    }
                }
                ModuleKind::Conjunction => {
                    if high {
                        *state |= 1 << input;
                    } else {
                        *state &= !(1 << input);
                    }

                    let next_low = *state != 65535;
                    if !next_low {
                        self.conj_signalled |= 1 << index;
                    }

                    for (index, output) in module.outputs.iter() {
                        self.queue.push_back((*index, *output, next_low));
                    }
                }
            }

            if high {
                self.highs += 1;
            } else {
                self.lows += 1;
            }
        }
    }

    fn new(machine: &Machine) -> Self {
        let mut states = vec![0u16; machine.modules.len()];
        for i in 0..states.len() {
            if let ModuleKind::Conjunction = machine.modules[i].kind {
                states[i] = !((1u16 << machine.modules[i].inputs.len() as u16) - 1);
            }
        }

        Self {
            states,
            conj_signalled: 0,
            queue: VecDeque::with_capacity(128),
            lows: 0,
            highs: 0,
        }
    }
}

#[derive(Default, Clone)]
struct Module {
    kind: ModuleKind,
    inputs: ArrayVec<u8, 16>,
    outputs: ArrayVec<(u8, u8), 8>,
}

#[derive(Default, Copy, Clone)]
enum ModuleKind {
    Broadcaster,
    #[default]
    Output,
    FlipFlop,
    Conjunction,
}

fn name_to_index(name: &[u8]) -> usize {
    if name.starts_with(b"roa") {
        26 * 26
    } else if name.starts_with(b"utp") || name.starts_with(b"out") {
        26 * 26 + 1
    } else {
        name.iter().fold(0, |c, v| c * 26 + (*v - b'a') as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> in
&in -> a
";

    #[test]
    fn p1_works_on_example() {
        assert_eq!(Machine::parse(P1_EXAMPLE).thousand_presses(), 32000000);
    }
}
