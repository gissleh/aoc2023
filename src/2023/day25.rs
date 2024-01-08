use arrayvec::ArrayVec;
use common::aoc::Day;
use common::ds::Graph;
use common::parse;
use common::parse::Parser;
use common::search::{bfs, Search, WithHint};

type PartGraph = Graph<[u8; 3], (), (), 16>;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Graph nodes", input.len());

    day.part("Part 1", || p1(&input));
}

fn parse(input: &[u8]) -> PartGraph {
    parse::n_bytes::<3>()
        .and_discard(b": ")
        .and(
            parse::n_bytes::<3>()
                .delimited_by(b' ')
                .repeat::<ArrayVec<_, 8>>(),
        )
        .delimited_by(b'\n')
        .repeat_fold(
            || PartGraph::new(),
            |mut graph, (name, children)| {
                let src = graph.ensure_node(name, ());

                for child in children {
                    let dst = graph.ensure_node(child, ());
                    graph.connect_mutual(src, dst, ());
                }

                graph
            },
        )
        .parse(input)
        .unwrap()
}

fn p1(input: &PartGraph) -> usize {
    let mut disabled_edges = Vec::<(usize, usize)>::with_capacity(128);
    let mut found_start = 0usize;

    'start_loop: for start in 0..input.len() {
        'end_loop: for end in (start + 1)..input.len() {
            disabled_edges.clear();
            for i in 0..4 {
                let res = bfs()
                    .with_initial_state(WithHint(start, ArrayVec::<(usize, usize), 16>::new()))
                    .find_mut(|s, WithHint(pos, mut path)| {
                        if pos == end {
                            return Some(path);
                        }

                        for (_, next, _) in input.edges_from(pos) {
                            if disabled_edges.contains(&(pos, *next)) {
                                continue;
                            }

                            path.push((pos, *next));
                            s.add_state(WithHint(*next, path.clone()));
                            path.pop();
                        }

                        None
                    });

                if i == 3 {
                    if res.is_some() {
                        continue 'end_loop;
                    } else {
                        break;
                    }
                }

                for (a, b) in res.unwrap().iter() {
                    disabled_edges.push((*a, *b));
                    disabled_edges.push((*b, *a));
                }
            }

            found_start = start;
            break 'start_loop;
        }
    }

    let mut disabled_edges2 = Vec::with_capacity(6);
    for (i, ei) in disabled_edges.iter().copied().enumerate() {
        let ei2 = (ei.1, ei.0);
        disabled_edges2.push(ei);
        disabled_edges2.push(ei2);

        for (j, ej) in disabled_edges.iter().copied().enumerate().skip(i + 1) {
            let ej2 = (ej.1, ej.0);
            disabled_edges2.push(ej);
            disabled_edges2.push(ej2);

            for ek in disabled_edges.iter().copied().skip(j + 1) {
                let ek2 = (ek.1, ek.0);
                disabled_edges2.push(ek);
                disabled_edges2.push(ek2);

                let count: usize = bfs().with_initial_state(found_start).gather(|s, pos| {
                    for (_, next, _) in input.edges_from(pos) {
                        if disabled_edges.contains(&(pos, *next)) {
                            continue;
                        }

                        s.add_state(*next);
                    }

                    Some(())
                });

                if count < input.len() - 10 {
                    return count * (input.len() - count);
                }

                disabled_edges2.pop();
                disabled_edges2.pop();
            }

            disabled_edges2.pop();
            disabled_edges2.pop();
        }

        disabled_edges2.pop();
        disabled_edges2.pop();
    }

    panic!("solution not found")
}

#[cfg(test)]
mod tests {
    use super::*;
}
