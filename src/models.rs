pub struct GraphData {
    verts: usize,
    edges: Vec<(usize, usize)>
}

impl GraphData {
    pub fn verts(&self) -> usize {
        self.verts
    }
    
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}

pub fn graphs() -> [GraphData; 6] {
    //    [2]
    //    / \
    // [0]   [1]
    let graph1 = GraphData {
        verts: 3,
        edges: vec![(0, 2), (1, 2)]
    };

    //     [0]       [5]
    //    / | \
    // [1] [2] [3]
    //       \ /
    //       [4]
    let graph2 = GraphData {
        verts: 6,
        edges: vec![(0, 1), (0, 2), (0, 3), (2, 4), (3, 4)]
    };
    
    // [0] [1] [2] [3]
    let graph3 = GraphData {
        verts: 4,
        edges: vec![],
    };

    // [0]   [1]
    //   \   /
    //    [2]
    //     |
    //    [3]
    let graph4 = GraphData {
        verts: 4,
        edges: vec![(0, 2), (1, 2), (3, 2)]
    };

    // [0]---[1]
    //   \   /
    //    [2]
    //     |
    //    [3]
    let graph5 = GraphData {
        verts: 4,
        edges: vec![(0, 2), (1, 2), (3, 2), (0, 1)]
    };

    //     [0]     _[5]--[6]--[7]   [11]
    //    / | \   /      / \
    // [1] [2] [3]     [8] [9]
    //       \ /         \ /
    //       [4]--------[10]
    //
    let verts = 12;
    let edges = vec![
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
    ];
    let graph6 = GraphData { verts, edges };

    [graph1, graph2, graph3, graph4, graph5, graph6]
}
