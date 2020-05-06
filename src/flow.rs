use crate::graph::Graph;
use crate::graph::Edge;
use crate::measure::Measure;
use crate::search;
use std::collections::HashMap;

pub fn ford_fulkerson<
    T: Default + Clone + Measure,
    G: Graph<T, T> + Clone,
>(
    graph: &G,
    s: i32,
    t: i32,
) -> HashMap<(i32, i32), f64> {
    // augment vertices
    // assert that all vertex capacities are infinity
    // clone to get residual graph

    // Maintain a map of new vertices to old vertices, in order to
    // determine vertex capacities later
    let mut old_vertex_mapping = HashMap::new();

    let mut new_graph = graph.to_owned();
    let mut edges_to_remove = vec![];
    let mut edges_to_add = vec![];

    let extended_vertex_graph = &new_graph;
    // for vertices with capacity < infinity
    let vertices = extended_vertex_graph.get_vertices();
    let mut v_ctr = vertices.keys().max().unwrap() + 1;
    for (v_id, vertex) in vertices.iter() {
        if vertex.weight != T::infinity() {
            old_vertex_mapping.insert(v_ctr, v_id);
            for (v2_id, wt) in extended_vertex_graph.neighbors(v_id).unwrap().iter() {
                edges_to_remove.push((v_id.clone(), v2_id.clone()));
                // let wt = extended_vertex_graph.remove_edge(v_id, v2_id).unwrap();
                edges_to_add.push(Edge {
                    v1: v_ctr,
                    v2: *v2_id,
                    weight: wt.clone(),
                });
            }
            edges_to_add.push(Edge {
                v1: *v_id,
                v2: v_ctr,
                weight: vertex.weight.clone(),
            });
            v_ctr += 1;
        }
    }

    for edge in edges_to_remove.iter() {
        new_graph.remove_edge(&edge.0, &edge.1);
    }

    for edge in edges_to_add.iter() {
        new_graph.add_edge(edge.clone());
    }

    let mut residual_graph = new_graph.clone();
    search::bfs_path_exists(&residual_graph, &s, &t);

    // now, find augmenting paths and augment flow
    // TODO: implement this

    HashMap::new()
}
