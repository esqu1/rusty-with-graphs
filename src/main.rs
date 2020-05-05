mod lib;
mod search;
mod flow;
mod shortest_path;
mod datastructures;
use lib::graph::GraphBase;
use lib::graph::GraphMutate;
use std::collections::HashMap;


macro_rules! add_edge {
    ($g: expr, $weight: expr, $v1 : expr, $v2 : expr) => {
        let edge = lib::graph::Edge {
            weight: $weight,
            v1: $v1,
            v2: $v2,
        };
        $g.add_edge(edge);
    };
}

fn main() {
    let g: GraphBase<i32> = lib::graph::GraphBase {
        vertices: HashMap::new(),
        is_directed: false,
    };

    let mut adj_g = lib::graph::AdjacencyListGraph {
        graph: g,
        adj_list: HashMap::new(),
    };
    add_edge!(&mut adj_g, 1.0, 1, 2);
    add_edge!(&mut adj_g, 1.0, 2, 3);
    add_edge!(&mut adj_g, 1.0, 1, 4);
    add_edge!(&mut adj_g, 1.0, 4, 5);
    search::bfs_print(&adj_g, &1);
}
