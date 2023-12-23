use crate::utils::gather_target::GatherTarget;
use arrayvec::ArrayVec;

#[derive(Debug, Eq, PartialEq)]
pub struct Graph<K, V, E, const CAP: usize = 16> {
    nodes: Vec<Node<K, V, E, CAP>>,
}

impl<K, V, E, const CAP: usize> Graph<K, V, E, CAP> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline(always)]
    pub fn node(&self, index: usize) -> Option<(&K, &V)> {
        self.nodes.get(index).map(|n| (&n.key, &n.value))
    }

    #[inline(always)]
    pub fn node_mut(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
        self.nodes
            .get_mut(index)
            .map(|n| (&mut n.key, &mut n.value))
    }

    #[inline(always)]
    pub fn nodes(&self) -> impl Iterator<Item = (usize, &K, &V)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (i, &n.key, &n.value))
    }

    #[inline(always)]
    pub fn find_by<F>(&self, pred: F) -> Option<usize>
    where
        F: Fn(&K) -> bool,
    {
        self.nodes.iter().position(|n| pred(&n.key))
    }

    /// Create a node with the key and value. This does not check if the key already exists; you
    /// should use ensure_node or check first with find_by if that's important.
    #[inline]
    pub fn create_node(&mut self, key: K, value: V) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            key,
            value,
            edges: ArrayVec::new(),
        });

        index
    }

    /// Iterate over all edges on a node, returning
    #[inline]
    pub fn edges_from(&self, index: usize) -> impl Iterator<Item = (&E, &usize, &K)> {
        self.nodes[index]
            .edges
            .iter()
            .map(|(target_idx, edge)| (edge, target_idx, &self.nodes[*target_idx].key))
    }

    #[inline]
    pub fn connect(&mut self, src: usize, dst: usize, edge: E) {
        self.nodes[src].edges.push((dst, edge));
    }

    pub fn replace_connection(&mut self, src: usize, dst: usize, edge: E) {
        if let Some(index) = self.nodes[src].edges.iter().position(|(edge_dst, _)| *edge_dst == dst) {
            self.nodes[src].edges[index] = (dst, edge)
        } else {
            self.nodes[src].edges.push((dst, edge));
        }
    }

    pub fn fallback_connection(&mut self, src: usize, dst: usize, edge: E) {
        if let None = self.nodes[src].edges.iter().find(|(edge_dst, _)| *edge_dst == dst) {
            self.nodes[src].edges.push((dst, edge));
        }
    }

    pub fn is_connected(&self, src: usize, dst: usize) -> bool {
        self.nodes[src].edges.iter().find(|(e, _)| *e == dst).is_some()
    }
}

impl<K, V, E, const CAP: usize> Graph<K, V, E, CAP>
where
    E: Clone,
{
    #[inline]
    pub fn connect_mutual(&mut self, src: usize, dst: usize, edge: E) {
        self.nodes[src].edges.push((dst, edge.clone()));
        self.nodes[dst].edges.push((src, edge));
    }
}

impl<K, V, E, const CAP: usize> Graph<K, V, E, CAP>
where
    K: Eq,
{
    /// Ensures that a node exists. This is different from find_or_create
    /// in that it sets the value if it finds it, and does not need a default
    /// value on the type.
    #[inline]
    pub fn ensure_node(&mut self, key: K, value: V) -> usize {
        match self.find(&key) {
            Some(index) => {
                self.nodes[index].value = value;
                index
            }
            None => self.create_node(key, value),
        }
    }

    #[inline(always)]
    pub fn find(&self, key: &K) -> Option<usize> {
        self.nodes.iter().position(|n| n.key.eq(key))
    }
}

impl<K, V, E, const CAP: usize> Graph<K, V, E, CAP>
where
    K: Eq,
    V: Default,
{
    #[inline]
    pub fn find_or_create(&mut self, key: K) -> usize {
        match self.find(&key) {
            Some(index) => index,
            None => self.create_node(key, Default::default()),
        }
    }
}

impl<K, V, E, const CAP: usize> GatherTarget<GraphInstruction<K, V, E, CAP>> for Graph<K, V, E, CAP>
where
    K: Eq,
    V: Default,
    E: Clone,
{
    #[inline]
    fn start_gathering(size_hint: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(size_hint),
        }
    }

    #[inline]
    fn gather_into(&mut self, _index: usize, desc: GraphInstruction<K, V, E, CAP>) -> bool {
        match desc {
            GraphInstruction::Node(key, value) => {
                self.ensure_node(key, value);
            }
            GraphInstruction::Edge(src, edge, dst) => {
                let src = self.find_or_create(src);
                let dst = self.find_or_create(dst);
                self.connect(src, dst, edge);
            }
            GraphInstruction::MutualEdge(src, edge, dst) => {
                let src = self.find_or_create(src);
                let dst = self.find_or_create(dst);
                self.connect_mutual(src, dst, edge);
            }
            GraphInstruction::NodeAndEdges(key, value, edges) => {
                let src = self.ensure_node(key, value);
                for (key, edge) in edges.into_iter() {
                    let dst = self.find_or_create(key);
                    self.connect(src, dst, edge);
                }
            }
        }

        false
    }
}

pub enum GraphInstruction<K, V, E, const CAP: usize = 4> {
    Node(K, V),
    Edge(K, E, K),
    MutualEdge(K, E, K),
    NodeAndEdges(K, V, ArrayVec<(K, E), CAP>),
}

#[derive(Debug, Eq, PartialEq)]
struct Node<K, V, E, const CAP: usize> {
    key: K,
    value: V,
    edges: ArrayVec<(usize, E), CAP>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{any_byte, bytes_until, n_bytes, signed_int, unsigned_int, Parser};
    use test::Bencher;

    const EXAMPLE_01: &[u8] = include_bytes!("./test_fixtures/graph_01.txt");
    const EXAMPLE_2022_DAY16: &[u8] = include_bytes!("./test_fixtures/2022_d16_example.txt");
    const EXAMPLE_2015_DAY09: &[u8] = include_bytes!("./test_fixtures/2015_d09_example.txt");

    fn parse_2022_d16_example() -> Graph<[u8; 2], i32, u8, 4> {
        b"Valve "
            .and_instead(n_bytes::<2>())
            .and_discard(b" has flow rate=")
            .and(signed_int::<i32>())
            .and_discard(b"; tunnel".and(b" leads to valve ".or(b"s lead to valves ")))
            .and(
                n_bytes::<2>()
                    .map(|name| (name, 1))
                    .delimited_by(b", ")
                    .repeat::<ArrayVec<_, 4>>(),
            )
            .map(|((name, flow_rate), tunnels_to)| {
                GraphInstruction::NodeAndEdges(name, flow_rate, tunnels_to)
            })
            .delimited_by(b'\n')
            .repeat()
            .parse(EXAMPLE_2022_DAY16)
            .unwrap()
    }

    #[test]
    fn gather_works_on_2022_d16_example() {
        let graph = parse_2022_d16_example();

        assert_eq!(graph.nodes.len(), 10);
        assert_eq!(
            graph,
            Graph {
                nodes: vec![
                    Node {
                        key: *b"AA",
                        value: 0,
                        edges: ArrayVec::try_from([(1, 1), (2, 1), (3, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"DD",
                        value: 20,
                        edges: ArrayVec::try_from([(4, 1), (0, 1), (5, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"II",
                        value: 0,
                        edges: ArrayVec::try_from([(0, 1), (9, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"BB",
                        value: 13,
                        edges: ArrayVec::try_from([(4, 1), (0, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"CC",
                        value: 2,
                        edges: ArrayVec::try_from([(1, 1), (3, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"EE",
                        value: 3,
                        edges: ArrayVec::try_from([(6, 1), (1, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"FF",
                        value: 0,
                        edges: ArrayVec::try_from([(5, 1), (7, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"GG",
                        value: 0,
                        edges: ArrayVec::try_from([(6, 1), (8, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"HH",
                        value: 22,
                        edges: ArrayVec::try_from([(7, 1)].as_slice()).unwrap()
                    },
                    Node {
                        key: *b"JJ",
                        value: 21,
                        edges: ArrayVec::try_from([(2, 1)].as_slice()).unwrap()
                    },
                ]
            }
        );
    }

    #[test]
    fn gather_works_on_2015_d09_example() {
        let graph: Graph<&[u8], (), u32> = bytes_until(b' ', true)
            .and_discard(b"to ")
            .and(bytes_until(b' ', true))
            .and_discard(b"= ")
            .and(unsigned_int::<u32>())
            .and_discard(b'\n')
            .map(|((src, dst), dist)| GraphInstruction::MutualEdge(src, dist, dst))
            .repeat()
            .parse(EXAMPLE_2015_DAY09)
            .unwrap();

        assert_eq!(
            graph,
            Graph {
                nodes: vec![
                    Node {
                        key: b"London".as_slice(),
                        value: (),
                        edges: ArrayVec::try_from([(1, 464), (2, 518)].as_slice()).unwrap()
                    },
                    Node {
                        key: b"Dublin".as_slice(),
                        value: (),
                        edges: ArrayVec::try_from([(0, 464), (2, 141)].as_slice()).unwrap()
                    },
                    Node {
                        key: b"Belfast".as_slice(),
                        value: (),
                        edges: ArrayVec::try_from([(0, 518), (1, 141)].as_slice()).unwrap()
                    },
                ]
            }
        )
    }

    #[bench]
    fn bench_gather_works_on_2022_d16_example(b: &mut Bencher) {
        b.iter(|| parse_2022_d16_example());
    }

    #[test]
    fn gather_target_can_parse() {
        let graph: Graph<char, u32, u32, 4> = b"node "
            .and_instead(
                any_byte()
                    .and_discard(b' ')
                    .and(unsigned_int::<u32>())
                    .map(|(k, v)| GraphInstruction::Node(k as char, v)),
            )
            .or(b"edge "
                .and_instead(any_byte())
                .and(b'-'.or(b'='))
                .and(any_byte())
                .and_discard(b' ')
                .and(unsigned_int::<u32>())
                .map(|(((src, c), dst), cost)| {
                    if c == b'=' {
                        GraphInstruction::MutualEdge(src as char, cost, dst as char)
                    } else {
                        GraphInstruction::Edge(src as char, cost, dst as char)
                    }
                }))
            .delimited_by(b'\n')
            .repeat()
            .parse(EXAMPLE_01)
            .unwrap();

        assert_eq!(
            graph,
            Graph {
                nodes: vec![
                    Node {
                        key: 'A',
                        value: 5000,
                        edges: ArrayVec::try_from([(1, 32), (2, 7), (5, 119)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'Z',
                        value: 7600,
                        edges: ArrayVec::try_from([(5, 2)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'B',
                        value: 0,
                        edges: ArrayVec::try_from([(0, 7), (1, 6)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'C',
                        value: 0,
                        edges: ArrayVec::try_from([(3, 2), (1, 170)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'D',
                        value: 9000,
                        edges: ArrayVec::try_from([(1, 9), (5, 11)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'E',
                        value: 0,
                        edges: ArrayVec::try_from([(4, 11), (0, 119), (1, 2)].as_slice()).unwrap()
                    },
                    Node {
                        key: 'F',
                        value: 4000,
                        edges: ArrayVec::try_from([].as_slice()).unwrap()
                    },
                    Node {
                        key: 'G',
                        value: 12000,
                        edges: ArrayVec::try_from([].as_slice()).unwrap()
                    },
                ],
            }
        );
    }

    #[test]
    fn graph_works_as_it_should() {
        let mut g: Graph<char, u32, u32, 4> = Graph::new();

        let a = g.create_node('a', 1);
        let b = g.create_node('b', 2);
        let c = g.create_node('c', 3);
        let d = g.create_node('d', 4);
        let e = g.create_node('e', 5);
        let f = g.create_node('f', 6);

        assert_eq!(g.find(&'c'), Some(2));
        assert_eq!(g.find(&'f'), Some(5));
        assert_eq!(g.find(&'g'), None);

        assert_eq!(g.node(0), Some((&'a', &1)));
        assert_eq!(g.node(2), Some((&'c', &3)));
        assert_eq!(g.node(3), Some((&'d', &4)));
        assert_eq!(g.node(5), Some((&'f', &6)));
        assert_eq!(g.node_mut(5), Some((&mut 'f', &mut 6)));
        assert_eq!(g.node(6), None);
        assert_eq!(g.node_mut(6), None);

        g.connect_mutual(a, b, 17);
        g.connect(b, d, 7);
        g.connect(b, c, 3);
        g.connect(d, e, 9);
        g.connect(e, f, 3);
        g.connect(a, f, 10);
        g.connect(f, a, 6);

        assert_eq!(
            g.edges_from(a).collect::<Vec<_>>().as_slice(),
            &[(&17, &b, &'b'), (&10, &f, &'f')]
        );
        assert_eq!(
            g.edges_from(b).collect::<Vec<_>>().as_slice(),
            &[(&17, &a, &'a'), (&7, &d, &'d'), (&3, &c, &'c')]
        );
        assert_eq!(
            g.edges_from(f).collect::<Vec<_>>().as_slice(),
            &[(&6, &a, &'a')]
        );
    }
}
