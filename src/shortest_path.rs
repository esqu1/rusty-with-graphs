use crate::datastructures;
use crate::graph::Graph;
use datastructures::heap::*;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::collections::HashSet;

pub trait Measure : Ord {
    fn zero() -> Self;
    fn infinity() -> Self;
}

struct PriorityVertex<E: Measure + Add<Output = E>> {
    pub dist: E,
    pub id: i32,
}

impl<E: Measure + Add<Output = E>> PartialEq for PriorityVertex<E> {
    fn eq(&self, other: &Self) -> bool {
        E::eq(&self.dist, &other.dist)
    }
}

impl<E: Measure + Add<Output = E>> Eq for PriorityVertex<E> {}

impl<E: Measure + Add<Output = E>> PartialOrd for PriorityVertex<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(E::cmp(&self.dist, &other.dist))
    }
}

impl<E: Measure + Add<Output = E>> Ord for PriorityVertex<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        E::cmp(&self.dist, &other.dist)
    }
}

pub fn dijkstra<V: Default + Copy, E: Default + Copy + Measure + Add<Output = E>>(
    graph: &dyn Graph<V, E>,
    vertex_id: &i32,
) -> HashMap<i32, E> {
    let queue = &mut MinHeap::new();
    let dist = &mut HashMap::new();
    let visited = &mut HashSet::new();
    {
        let vertices = graph.vertex_set();
        for v in vertices.into_iter() {
            if v == *vertex_id {
                dist.insert(v, E::zero());
                queue.insert(PriorityVertex {
                    id: v,
                    dist: E::zero(),
                });
            }
            dist.insert(v, E::infinity());
            queue.insert(PriorityVertex {
                id: v,
                dist: E::infinity(),
            })
        }
    }

    while queue.size() > 0 {
        let v = queue.pop().unwrap();
        if visited.contains(&v.id) {
            continue;
        }
        visited.insert(v.id.clone());
        let og_dist = *dist.get(&v.id).unwrap();
        for (neighbor_id, wt) in graph.neighbors(&v.id).unwrap().iter() {
            let new_dist = og_dist + *wt;
            if new_dist < *dist.get(&neighbor_id).unwrap() {
                dist.insert(*neighbor_id, new_dist);
                queue.insert(PriorityVertex {
                    id: *neighbor_id,
                    dist: new_dist,
                })
            }
        }
    }
    dist.clone()
}
