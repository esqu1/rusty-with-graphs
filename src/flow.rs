use crate::lib::graph::Graph;
use crate::search;
use std::collections::HashMap;

pub fn ford_fulkerson<T: Default>(graph: &dyn Graph<T>, s: i32, t: i32) -> HashMap<(i32, i32), f64> {
    // augment vertices
    // assert that all vertex capacities are infinity
    // clone to get residual graph
    HashMap::new()
}