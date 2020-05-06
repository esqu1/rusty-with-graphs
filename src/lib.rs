mod graph;
mod datastructures;
mod shortest_path;
mod search;
mod flow;
mod measure;

// use datastructures::*;
// use shortest_path::*;
// use search::*;
#[allow(unused_imports)]
use graph::Graph;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(unused_macros)]
macro_rules! add_edge {
    ($g: expr, $weight: expr, $v1 : expr, $v2 : expr) => {
        let edge = graph::Edge {
            weight: $weight,
            v1: $v1,
            v2: $v2,
        };
        $g.add_edge(edge);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph_prints() {
        let g: graph::GraphBase<i32, f64> = graph::GraphBase {
            vertices: HashMap::new(),
            edges: vec![], 
            is_directed: false,
        };

        let mut adj_g = graph::AdjacencyListGraph {
            graph: g,
            adj_list: HashMap::new(),
        };
        add_edge!(&mut adj_g, 1.0, 1, 2);
        add_edge!(&mut adj_g, 1.0, 2, 3);
        add_edge!(&mut adj_g, 1.0, 1, 4);
        add_edge!(&mut adj_g, 1.0, 4, 5);
        search::bfs_print(&adj_g, &1);
        assert_eq!(2 + 2, 4);
    }
}