use crate::lib::graph::Graph;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn bfs(graph : &Graph, vertex_id: &i32) {
    let mut queue : VecDeque<i32> = VecDeque::new();
    let mut visited_set : HashSet<i32> = HashSet::new();
    queue.push_back(vertex_id.clone()); 
    while queue.len() > 0 {
        if let Some(new_vertex) = queue.pop_front() {
            if visited_set.contains(&new_vertex) {
                continue;
            }

            // do something to visited vertex

            if let Some(neighbors) = graph.neighbors(&new_vertex) {
                for (v, _) in neighbors.iter() {
                    queue.push_back(v.to_owned());
                }
            }
            visited_set.insert(new_vertex.to_owned());
        }
    }
}