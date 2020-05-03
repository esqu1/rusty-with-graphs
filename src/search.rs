use crate::lib::graph::Graph;
use crate::lib::graph::Vertex;
use std::collections::VecDeque;
use std::collections::HashSet;


pub fn bfs_fold<T: Default, U>(graph : &dyn Graph<T>, vertex_id: &i32, fold_func: &dyn Fn(&Vertex<T>, U) -> U, base: U) {
    let mut queue : VecDeque<i32> = VecDeque::new();
    let mut visited_set : HashSet<i32> = HashSet::new();
    let mut acc = base;
    queue.push_back(vertex_id.clone()); 
    while queue.len() > 0 {
        if let Some(new_vertex) = queue.pop_front() {
            if visited_set.contains(&new_vertex) {
                continue;
            }
            let vertex = graph.get_vertices().get(&new_vertex).unwrap();

            // do something to visited vertex
            acc = fold_func(&vertex, acc);

            if let Some(neighbors) = graph.neighbors(&new_vertex) {
                for (v, _) in neighbors.iter() {
                    queue.push_back(v.to_owned());
                }
            }
            visited_set.insert(new_vertex.to_owned());
        }
    }
}

pub fn print_vertex<T: Default>(vertex: &Vertex<T>, _: ()) {
    println!("Visiting vertex {}", vertex.id);
}

pub fn bfs_print<T: Default>(graph: &dyn Graph<T>, vertex_id: &i32) {
    bfs_fold(graph, vertex_id, &print_vertex, ());
}