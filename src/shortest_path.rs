use crate::lib::graph::Graph;
use crate::lib::graph::Vertex;
use std::ops::Add;

pub fn dijkstra<V: Default + Clone, E: Default + Clone + PartialOrd + Add<Output = V>>(
    graph: &dyn Graph<V, E>,
    vertex_id: &i32,
) {
}
