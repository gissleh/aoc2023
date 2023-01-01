use arrayvec::ArrayVec;

pub struct Graph<N, E, const CAP: usize = 16> {
    nodes: Vec<Node<N, E, CAP>>,
}

impl<N, E, const CAP: usize> Graph<N, E, CAP> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity)
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline(always)]
    pub fn node(&self, index: usize) -> Option<&N> {
        self.nodes.get(index).map(|n| &n.body)
    }

    #[inline(always)]
    pub fn node_mut(&mut self, index: usize) -> Option<&mut N> {
        self.nodes.get_mut(index).map(|n| &mut n.body)
    }

    #[inline(always)]
    pub fn find_by<F>(&self, pred: F) -> Option<usize> where F: Fn(&N) -> bool {
        self.nodes.iter().position(|n| pred(&n.body))
    }

    #[inline]
    pub fn create_node(&mut self, body: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            body,
            edges: ArrayVec::new(),
        });

        index
    }

    /// Iterate over all edges on a node, returning
    #[inline]
    pub fn edges_from(&self, index: usize) -> impl Iterator<Item=(&E, &usize, &N)> {
        self.nodes[index].edges.iter()
            .map(|(target_idx, edge)| (
                edge, target_idx, &self.nodes[*target_idx].body
            ))
    }

    #[inline]
    pub fn connect(&mut self, src: usize, dst: usize, edge: E) {
        self.nodes[src].edges.push((dst, edge));
    }
}

impl<N, E, const CAP: usize> Graph<N, E, CAP> where E: Clone {
    #[inline]
    pub fn connect_mutual(&mut self, src: usize, dst: usize, edge: E) {
        self.nodes[src].edges.push((dst, edge.clone()));
        self.nodes[dst].edges.push((src, edge));
    }
}

impl<N, E, const CAP: usize> Graph<N, E, CAP> where N: Eq {
    #[inline]
    pub fn find_or_insert(&mut self, body: N) -> usize {
        match self.find(&body) {
            Some(index) => index,
            None => self.create_node(body),
        }
    }

    #[inline(always)]
    pub fn find(&self, needle: &N) -> Option<usize> {
        self.nodes.iter().position(|n| n.body.eq(needle))
    }
}

struct Node<N, E, const CAP: usize> {
    body: N,
    edges: ArrayVec<(usize, E), CAP>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_works_as_it_should() {
        let mut g: Graph<char, u32, 4> = Graph::new();

        let a = g.create_node('a');
        let b = g.create_node('b');
        let c = g.create_node('c');
        let d = g.create_node('d');
        let e = g.create_node('e');
        let f = g.create_node('f');

        assert_eq!(g.find(&'c'), Some(2));
        assert_eq!(g.find(&'f'), Some(5));
        assert_eq!(g.find(&'g'), None);

        assert_eq!(g.node(0), Some(&'a'));
        assert_eq!(g.node(2), Some(&'c'));
        assert_eq!(g.node(3), Some(&'d'));
        assert_eq!(g.node(5), Some(&'f'));
        assert_eq!(g.node_mut(5), Some(&mut 'f'));
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