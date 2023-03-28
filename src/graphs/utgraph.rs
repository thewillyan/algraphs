/// A symmetric matrix.
pub struct SymMat<T> {
    size: usize,
    values: Vec<T>,
}

impl<T> SymMat<T> {
    /// Sum of integers from 1 to `end` (inclusive).
    fn int_sum(end: usize) -> usize {
        (end * (end + 1)) / 2
    }

    /// Get the index where the element of line `i` column `j` is stored in
    /// the matrix vector.
    fn index(&self, i: usize, j: usize) -> usize {
        // The line must aways be <= col.
        let (line, col) = if i > j { (j, i) } else { (i, j) };

        let skip = Self::int_sum(line);
        (self.size * line + col) - skip
    }

    /// Returns a reference to the element of the line `i` column `j` in the
    /// matrix or `None` if out of bounds.
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        let conn_index = self.index(i, j);
        self.values.get(conn_index)
    }

    /// Returns a mutable reference to the element of the line `i` column `j`
    /// in the matrix or `None` if out of bounds.
    pub fn get_mut(&mut self, a: usize, b: usize) -> Option<&mut T> {
        let conn_index = self.index(a, b);
        self.values.get_mut(conn_index)
    }

    /// Set the element of line `i` column `j` to the given `value`.
    pub fn set(&mut self, i: usize, j: usize, value: T) {
        let index = self.index(i, j);
        match self.values.get_mut(index) {
            Some(r) => *r = value,
            None => panic!("({}, {}) out of range.", i, j),
        }
    }

    /// Get the matrix items as a slice of values.
    pub fn values(&self) -> &[T] {
        &self.values
    }
}

impl<T: Clone> SymMat<T> {
    /// Creates a new symmetric matrix with `size` lines and columns filled with
    /// the given `value`.
    pub fn fill(size: usize, value: T) -> Self {
        Self {
            size,
            values: vec![value; Self::int_sum(size)],
        }
    }
}

impl<T: Default + Clone> SymMat<T> {
    /// Creates a new symmetric matrix with `size` lines and columns filled with
    /// the default value of `T`.
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
    /// Creates new graph with `verts` vertices.
    pub fn new(verts: usize) -> Self {
        Self {
            edges: 0,
            adj_mat: SymMat::fill_default(verts),
        }
    }

    /// Connect the vertex 'a' to the vertex 'b'.
    fn connect(&mut self, a: usize, b: usize) {
        if a == b {
            panic!("Cannot connect a vertice to itself.");
        } else if self.connected(a, b) {
            panic!("The vertice ({a}, {b}) already exists!");
        }
        self.adj_mat.set(a, b, 1);
        *self.adj_mat.get_mut(a, a).unwrap() += 1;
        *self.adj_mat.get_mut(b, b).unwrap() += 1;
    }

    /// Add the given edges to the graph. An edge is represented by a tuple
    /// of the form (a, b), which represents an edge from 'a' to 'b'.
    pub fn with_edges(mut self, edges: &[(usize, usize)]) -> Self {
        for &(from, to) in edges {
            self.connect(from, to);
        }
        self.edges = edges.len();
        self
    }

    /// Get the number of vertices of the graph.
    pub fn verts(&self) -> usize {
        self.adj_mat.size
    }

    /// Get the number of edges of the graph.
    pub fn edges(&self) -> usize {
        self.edges
    }

    /// Returns `true` if the vertex 'a' is connected to the vertex 'b' and
    /// `false` otherwise.
    pub fn connected(&self, a: usize, b: usize) -> bool {
        a != b && *self.adj_mat.get(a, b).expect("Invalid edge.") == 1
    }

    /// Returns the degree of the vertex 'a' or `None` if the vertex is out of
    /// bounds.
    pub fn degree(&self, a: usize) -> Option<usize> {
        self.adj_mat.get(a, a).copied()
    }

    /// Returns the maximum degree of the graph.
    pub fn max_deg(&self) -> usize {
        (0..self.verts())
            .map(|val| self.degree(val).unwrap())
            .max()
            .unwrap()
    }

    /// Returns `true` if the graph is a star and `false` otherwise.
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

    /// Returns the neighborhood of 'a'.
    pub fn nbhd(&self, a: usize) -> Vec<usize> {
        let mut nbhd = Vec::new();
        for b in 0..self.verts() {
            if self.connected(a, b) {
                nbhd.push(b);
            }
        }
        nbhd
    }

    /// Returns a path from 'a' to 'b' or `None` if it is not possible.
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
                }
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
    use super::UTGraph;
    use crate::models::GRAPHS;

    #[test]
    fn test_deg() {
        let model = &GRAPHS[2];
        let graph = UTGraph::new(model.verts).with_edges(model.edges);
        for v in 0..graph.verts() {
            assert!(graph.degree(v).unwrap() == 0)
        }

        let model = &GRAPHS[4];
        let graph = UTGraph::new(model.verts).with_edges(model.edges);
        let degrees = [2, 2, 3, 1];
        for (v, deg) in degrees.iter().enumerate() {
            assert!(graph.degree(v).unwrap() == *deg);
        }
    }

    #[test]
    fn test_max_deg() {
        let max_degrees = [2, 3, 0, 3, 3, 4];
        for (model, max_deg) in GRAPHS.iter().zip(max_degrees) {
            let graph = UTGraph::new(model.verts).with_edges(model.edges);
            assert_eq!(max_deg, graph.max_deg())
        }
    }

    #[test]
    fn test_is_star() {
        for (i, model) in GRAPHS.iter().enumerate() {
            let graph = UTGraph::new(model.verts).with_edges(model.edges);
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
        let graph = UTGraph::new(model.verts).with_edges(model.edges);

        assert!(graph.path(0, 11).is_none());

        let path = graph.path(0, 6).expect("No path received.");
        assert!(*path.first().unwrap() == 0 && *path.last().unwrap() == 6);
    }
}
