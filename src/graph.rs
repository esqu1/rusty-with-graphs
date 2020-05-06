extern crate matrix;

use matrix::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Vertex<T: Default + Clone> {
    pub id: i32,
    pub weight: T, // for max flows, ignored if used for weighted graphs
}

#[derive(Clone, Copy, Debug)]
pub struct Edge<T: Default + Clone> {
    pub weight: T,
    pub v1: i32,
    pub v2: i32, // for undirected graphs, enforce that v1 < v2
}

pub struct GraphBase<U: Default + Clone, V: Default + Clone> {
    pub vertices: HashMap<i32, Vertex<U>>,
    pub edges: Vec<Edge<V>>,
    pub is_directed: bool,
}

pub struct AdjacencyListGraph<U: Default + Clone, V: Default + Clone> {
    pub graph: GraphBase<U, V>,
    pub adj_list: HashMap<i32, HashMap<i32, V>>, // (v_id, wt)
}

pub struct AdjacencyMatrixGraph<U: Default + Clone> {
    pub graph: GraphBase<U, f64>,
    pub adj_matrix: Compressed<f64>,
}

pub trait Graph<U: Default + Clone, V: Default + Clone> {
    // assumes simple graph structure 
    fn neighbors(&self, vertex_id: &i32) -> Option<HashMap<i32, V>>;
    fn degree(&self, vertex_id: &i32) -> Option<usize>;
    fn get_vertices(&self) -> &HashMap<i32, Vertex<U>>;
    fn vertex_set(&self) -> Vec<i32>;

    // mutation methods
    fn add_edge(&mut self, e: Edge<V>);
    fn remove_edge(&mut self, v1: &i32, v2: &i32) -> Option<V>;
    fn change_edge_weight(&mut self, v1: &i32, v2: &i32, weight: &V);
}

impl<U: Default + Clone, V: Default + Clone> GraphBase<U, V> {
    fn add_edge(&mut self, e: Edge<V>) {
        let v1 = e.v1;
        let v2 = e.v2;

        // make default vertices if they don't exist already
        if let None = self.vertices.get(&v1) {
            let new_v = Vertex {
                id: v1.to_owned(),
                weight: U::default(),
            };
            self.vertices.insert(v1, new_v);
        }
        if let None = self.vertices.get(&v2) {
            let new_v = Vertex {
                id: v2.to_owned(),
                weight: U::default(),
            };
            self.vertices.insert(v2, new_v);
        }

        self.edges.push(e);
    }

    fn remove_edge(&mut self, v1: &i32, v2: &i32) -> Option<V> {
        if let Some(index) = self.edges.iter().position(|e| e.v1 == *v1 && e.v2 == *v2) {
            Some(self.edges.remove(index).weight)
        } else {
            None
        }
    }

    fn change_edge_weight(&mut self, v1: &i32, v2: &i32, weight: &V) {
        if let Some(edge_change) = self.edges.iter_mut().find(|e| e.v1 == *v1 && e.v2 == *v2) {
            edge_change.weight = weight.clone();
        }
    }
}

impl<U: Default + Clone, V: Default + Clone> Graph<U, V> for AdjacencyListGraph<U, V> {
    fn get_vertices(self: &Self) -> &HashMap<i32, Vertex<U>> {
        &self.graph.vertices
    }
    fn neighbors(self: &AdjacencyListGraph<U, V>, vertex_id: &i32) -> Option<HashMap<i32, V>> {
        if let Some(x) = self.adj_list.get(vertex_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    fn degree(self: &AdjacencyListGraph<U, V>, vertex_id: &i32) -> Option<usize> {
        self.adj_list.get(vertex_id).map(|x| x.len())
    }

    fn vertex_set(&self) -> Vec<i32> {
        let mut ve = vec![];
        for v in self.graph.vertices.keys() {
            ve.push(v.clone());
        }
        ve
    }

    fn add_edge(self: &mut AdjacencyListGraph<U, V>, e: Edge<V>) {
        let v1 = e.v1;
        let v2 = e.v2;

        if let Some(set) = self.adj_list.get_mut(&v1) {
            set.insert(v2, e.weight.clone());
        } else {
            let mut new_map = HashMap::new();
            new_map.insert(v2, e.weight.clone());
            self.adj_list.insert(v1, new_map);
        }
        if !self.graph.is_directed {
            if let Some(set) = self.adj_list.get_mut(&v2) {
                set.insert(v1, e.weight.clone());
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(v1, e.weight.clone());
                self.adj_list.insert(v2, new_map);
            }
        }
        self.graph.add_edge(e);
    }

    fn remove_edge(&mut self, v1: &i32, v2: &i32) -> Option<V> {
        if let Some(neighbors) = self.adj_list.get_mut(&v1) {
            self.graph.remove_edge(v1, v2); 
            neighbors.remove(v2)
        } else {
            None
        }
    }

    fn change_edge_weight(&mut self, v1: &i32, v2: &i32, weight: &V) {
        if let Some(neighbors) = self.adj_list.get_mut(&v1) {
            neighbors.insert(*v2, weight.clone());
            self.graph.change_edge_weight(v1, v2, weight);
        }
    }
}

/**
 * Assumes number of vertices = maximum ID + 1
 */
impl<U: Default + Clone> Graph<U, f64> for AdjacencyMatrixGraph<U> {
    fn get_vertices(self: &Self) -> &HashMap<i32, Vertex<U>> {
        &self.graph.vertices
    }
    fn neighbors(&self, vertex_id: &i32) -> Option<HashMap<i32, f64>> {
        if *vertex_id as usize >= self.adj_matrix.rows() {
            None
        } else {
            let num_columns = self.adj_matrix.columns();
            let mut map = HashMap::new();
            for i in 0..num_columns {
                let weight = self.adj_matrix.get((*vertex_id as usize, i));
                if weight.is_finite() {
                    map.insert(i as i32, weight);
                }
            }
            Some(map)
        }
    }

    fn degree(&self, vertex_id: &i32) -> Option<usize> {
        // TODO: make this more efficient
        let num_columns = self.adj_matrix.columns();
        let mut deg = 0;
        for i in 0..num_columns {
            let weight = self.adj_matrix.get((*vertex_id as usize, i));
            if weight.is_finite() {
                deg += 1;
            }
        }
        Some(deg)
    }

    fn vertex_set(&self) -> Vec<i32> {
        let mut ve = vec![];
        for i in 0..(self.get_vertices().len()) {
            ve.push(i as i32);
        }
        ve
    }
    
    fn add_edge(&mut self, e: Edge<f64>) {
        let v1 = e.v1;
        let v2 = e.v2;

        self.adj_matrix.set((v1 as usize, v2 as usize), e.weight);
        if !self.graph.is_directed {
            self.adj_matrix.set((v2 as usize, v2 as usize), e.weight);
        }
        self.graph.add_edge(e);
    }

    fn remove_edge(&mut self, v1: &i32, v2: &i32) -> Option<f64> {
        let wt = self.adj_matrix.get((*v1 as usize, *v2 as usize));
        self.adj_matrix.set((*v1 as usize, *v2 as usize), f64::INFINITY);
        self.graph.remove_edge(&v1, &v2);
        Some(wt)
    }

    fn change_edge_weight(&mut self, v1: &i32, v2: &i32, weight: &f64) {
        self.adj_matrix.set((*v1 as usize, *v2 as usize), *weight);
        self.graph.change_edge_weight(v1, v2, weight);
    }
}