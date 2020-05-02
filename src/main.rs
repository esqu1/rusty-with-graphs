mod lib;
use std::collections::HashMap;
use lib::graph::GraphMutate;
use lib::graph::GraphBase;

macro_rules! add_edge {
    ($g: expr, $weight: expr, $v1 : expr, $v2 : expr) => {
        let edge = lib::graph::Edge {
            weight: $weight, v1: $v1, v2: $v2,
        };
        $g.add_edge(edge);
    }
}

fn main() {
    let g : GraphBase<i32> = lib::graph::GraphBase {
        vertices: HashMap::new(),
        is_directed: false,
    };

    let mut adj_g = lib::graph::AdjacencyListGraph {
        graph: g,
        adj_list: HashMap::new(),
    };
    add_edge!(&mut adj_g, 2.0, 4, 1);
    println!("Hello, world!");
}
