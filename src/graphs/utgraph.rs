/// A symetric matrix.
pub struct SymMat<T> {
    size: usize,
    values: Vec<T>,
}

impl<T> SymMat<T> {
    /// Sum of integers from 1 to `end` (inclusive).
    fn int_sum(end: usize) -> usize {
        (end * (end + 1))/2
    }

    fn index(&self, a: usize, b: usize) -> usize {
        // The line must aways be <= col.
        let (line, col) = if a > b {
            (b, a)
        } else {
            (a, b)
        };

        let skip = Self::int_sum(line);
        (self.size * line + col) - skip
    }

    pub fn get(&self, a: usize, b: usize) -> Option<&T> {
        let conn_index = self.index(a, b);
        self.values.get(conn_index)
    }

    pub fn get_mut(&mut self, a: usize, b: usize) -> Option<&mut T> {
        let conn_index = self.index(a, b);
        self.values.get_mut(conn_index)
    }

    pub fn set(&mut self, line: usize, col: usize, value: T) {
        let conn_index = self.index(line, col);
        match self.values.get_mut(conn_index) {
            Some(r) => *r = value,
            None => panic!("({}, {}) out of range.", line, col),
        }
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }
}

impl<T: Clone> SymMat<T> {
    pub fn fill(size: usize, value: T) -> Self {
        Self {
            size,
            values: vec![value; Self::int_sum(size)],
        }
    }
}

impl<T: Default + Clone> SymMat<T> {
    pub fn fill_default(size: usize) -> Self {
        Self {
            size,
            values: vec![T::default(); Self::int_sum(size)],
        }
    }
}


/// Upper triangular graph.
pub struct UTGraph {
    edges: usize,
    adj_mat: SymMat<usize>,
}

impl UTGraph {
    pub fn new(verts: usize) -> Self {
        Self {
            edges: 0,
            adj_mat: SymMat::fill_default(verts),
        }
    }

    pub fn edges(mut self, edges: &[(usize, usize)]) -> Self {
        for &(from, to) in edges {
            self.connect(from, to);
        }
        self.edges = edges.len();
        self
    }

    pub fn verts(&self) -> usize {
        self.adj_mat.size
    }

    pub fn connect(&mut self, a: usize, b: usize) {
        if a == b {
            panic!("Cannot connect a vertice to itself.");
        } else if self.connected(a, b) {
            panic!("The vertice ({a}, {b}) already exists!");
        }
        self.adj_mat.set(a, b, 1);
        *self.adj_mat.get_mut(a, a).unwrap() += 1;
        *self.adj_mat.get_mut(b, b).unwrap() += 1;
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        a != b && *self.adj_mat.get(a, b).expect("Invalid edge.") == 1
    }

    pub fn degree(&self, a: usize) -> Option<usize> {
        self.adj_mat.get(a, a).copied()
    }

    pub fn max_deg(&self) -> usize {
        (0..self.verts())
            .map(|val| self.degree(val).unwrap())
            .max()
            .unwrap()
    }

    pub fn is_star(&self) -> bool {
        let max_deg = self.verts() - 1;
        if self.edges != max_deg {
            return false;
        }

        for v in 0..self.verts() {
            let deg = self.degree(v).unwrap();
            if deg == max_deg {
                return true;
            } else if deg != 1 {
                return false;
            }
        }
        unreachable!()
    }

    // Neighborood
    pub fn nbhd(&self, a: usize) -> Vec<usize> {
        let mut nbhd = Vec::new();
        for b in 0..self.verts() {
            if self.connected(a, b) {
                nbhd.push(b);
            }
        }
        nbhd
    }

    pub fn path(&self, a: usize, b: usize) -> Option<Vec<usize>> {
        let mut path_stack = vec![a];
        let mut blist = Vec::new();

        while let Some(&curr_vert) = path_stack.last() {
            if self.connected(curr_vert, b) {
                path_stack.push(b);
                return Some(path_stack);
            }

            let neigh = self.nbhd(curr_vert);
            let new_vert = neigh.iter().find(|&v| blist.binary_search(v).is_err());

            let bitem = match new_vert {
                Some(&v) => {
                    path_stack.push(v);
                    curr_vert
                },
                None => path_stack.pop().unwrap(),
            };

            if let Err(i) = blist.binary_search(&bitem) {
                blist.insert(i, bitem);
            }
        }
        None
    }
}

#[cfg(test)]
pub mod test {
    use crate::models::GRAPHS;
    use super::UTGraph;

    #[test]
    fn test_deg() {
        let model = &GRAPHS[2];
        let graph = UTGraph::new(model.verts).edges(model.edges);
        for v in 0..graph.verts() {
            assert!(graph.degree(v).unwrap() == 0)
        }

        let model = &GRAPHS[4];
        let graph = UTGraph::new(model.verts).edges(model.edges);
        let degrees = [2, 2, 3, 1];
        for (v, deg) in degrees.iter().enumerate() {
            assert!(graph.degree(v).unwrap() == *deg);
        }
    }

    #[test]
    fn test_max_deg() {
        let max_degrees = [2, 3, 0, 3, 3, 4];
        for (model, max_deg) in GRAPHS.iter().zip(max_degrees) {
            let graph = UTGraph::new(model.verts).edges(model.edges);
            assert_eq!(max_deg, graph.max_deg())
        }
    }

    #[test]
    fn test_is_star() {
        for (i, model) in GRAPHS.iter().enumerate() {
            let graph = UTGraph::new(model.verts).edges(model.edges);
            if i == 0 || i == 3 {
                assert!(graph.is_star());
            } else {
                assert!(!graph.is_star());
            }
        }
    }

    #[test]
    fn test_path() {
        let model = &GRAPHS[5];
        let graph = UTGraph::new(model.verts).edges(model.edges);

        assert!(graph.path(0, 11).is_none());

        let path = graph.path(0, 6).expect("No path received.");
        assert!(*path.first().unwrap() == 0 && *path.last().unwrap() == 6);
    }
}
