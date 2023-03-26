pub struct GraphData {
    verts: usize,
    edges: &'static [(usize, usize)]
}

impl GraphData {
    pub fn verts(&self) -> usize {
        self.verts
    }
    
    pub fn edges(&self) -> &[(usize, usize)] {
        self.edges
    }
}

//    [2]
//    / \
// [0]   [1]
const GRAPH1: GraphData = GraphData {
    verts: 3,
    edges: &[(0, 2), (1, 2)]
};

//     [0]       [5]
//    / | \
// [1] [2] [3]
//       \ /
//       [4]
const GRAPH2: GraphData = GraphData {
    verts: 6,
    edges: &[(0, 1), (0, 2), (0, 3), (2, 4), (3, 4)]
};

// [0] [1] [2] [3]
const GRAPH3: GraphData = GraphData {
    verts: 4,
    edges: &[],
};

// [0]   [1]
//   \   /
//    [2]
//     |
//    [3]
const GRAPH4: GraphData = GraphData {
    verts: 4,
    edges: &[(0, 2), (1, 2), (3, 2)]
};

// [0]---[1]
//   \   /
//    [2]
//     |
//    [3]
const GRAPH5: GraphData = GraphData {
    verts: 4,
    edges: &[(0, 2), (1, 2), (3, 2), (0, 1)]
};

//     [0]     _[5]--[6]--[7]   [11]
//    / | \   /      / \
// [1] [2] [3]     [8] [9]
//       \ /         \ /
//       [4]--------[10]
//
const GRAPH6: GraphData = GraphData {
    verts: 12,
    edges: &[
        (0, 1),
        (0, 2),
        (0, 3),
        (2, 3),
        (3, 4),
        (3, 5),
        (5, 6),
        (6, 7),
        (6, 8),
        (6, 9),
        (8, 10),
        (9, 10),
        (10, 4),
    ]
};

pub const GRAPHS: [GraphData; 6] = [GRAPH1, GRAPH2, GRAPH3, GRAPH4, GRAPH5, GRAPH6];
