use crate::graph::Graph;
use crate::graph::Vertex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn bfs_fold<T: Default + Clone, V: Default + Clone, U, G: Graph<T, V>>(
    graph: &G,
    vertex_id: &i32,
    fold_func: &dyn Fn((Option<i32>, &Vertex<T>), U) -> U,
    base: U,
) -> U {
    let mut queue: VecDeque<(Option<i32>, i32)> = VecDeque::new();
    let mut visited_set: HashSet<i32> = HashSet::new();
    let mut acc = base;
    queue.push_back((None, vertex_id.clone()));
    while queue.len() > 0 {
        if let Some((pred, new_vertex)) = queue.pop_front() {
            if visited_set.contains(&new_vertex) {
                continue;
            }
            let vertex = graph.get_vertices().get(&new_vertex).unwrap();

            // do something to visited vertex
            acc = fold_func((pred, &vertex), acc);

            if let Some(neighbors) = graph.neighbors(&new_vertex) {
                for (v, _) in neighbors.iter() {
                    queue.push_back((Some(new_vertex), v.to_owned()));
                }
            }
            visited_set.insert(new_vertex.to_owned());
        }
    }
    acc
}

fn print_vertex<T: Default + Clone>((_, vertex): (Option<i32>, &Vertex<T>), _: ()) {
    println!("Visiting vertex {}", vertex.id);
}

pub fn bfs_print<T: Default + Clone, V: Default + Clone, G: Graph<T, V>>(
    graph: &G,
    vertex_id: &i32,
) {
    bfs_fold(graph, vertex_id, &print_vertex, ());
}

pub fn bfs_path_exists<T: Default + Clone, V: Default + Clone, G: Graph<T, V>>(
    graph: &G,
    start: &i32,
    finish: &i32,
) {
    bfs_fold(
        graph,
        start,
        &|(_, v): (Option<i32>, &Vertex<T>), acc| acc || v.id == *finish,
        false,
    );
}

fn add_pred<'a, T: Default + Clone>(
    (pred, v): (Option<i32>, &Vertex<T>),
    acc: &'a mut HashMap<i32, Option<i32>>,
) -> &'a mut HashMap<i32, Option<i32>> {
    acc.insert(v.id, pred);
    acc
}

pub fn build_bfs_preds<T: Default + Clone, V: Default + Clone, G: Graph<T, V>>(
    graph: &G,
    vertex_id: &i32,
) -> HashMap<i32, Option<i32>> {
    let map = HashMap::new();
    bfs_fold(graph, vertex_id, &add_pred, &mut HashMap::new());
    map
}
