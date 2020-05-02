extern crate matrix;

pub mod graph {
    use std::collections::HashMap;
    use matrix::prelude::*;

    #[derive(PartialEq, Debug)]
    pub enum NotFoundError {
        NotFound,
    }

    pub struct Vertex<T : Default> {
        pub id: i32,
        pub info: T,
        pub weight: f64, // for max flows, ignored if used for weighted graphs
    }

    pub struct Edge {
        pub weight: f64,
        pub v1: i32, 
        pub v2: i32, // for undirected graphs, enforce that v1 < v2
    }

    pub struct GraphBase<U : Default> {
        pub vertices: HashMap<i32, Vertex<U>>,
        pub is_directed: bool,
    }

    pub struct AdjacencyListGraph<U : Default> {
        pub graph: GraphBase<U>,
        pub adj_list: HashMap<i32, HashMap<i32, f64>>, // (v_id, wt)
    }

    pub struct AdjacencyMatrixGraph<U : Default> {
        pub graph: GraphBase<U>,
        pub adj_matrix: Compressed<f64>,
    }

    pub trait Neighbor {
        fn neighbors(&self, vertex_id: &i32) -> Result<HashMap<i32, f64>, NotFoundError>;
    }
    
    impl<U : Default> Neighbor for AdjacencyListGraph<U> {
        fn neighbors(self : &AdjacencyListGraph<U>, vertex_id: &i32) -> Result<HashMap<i32, f64>, NotFoundError> {
            if let Some(x) = self.adj_list.get(vertex_id) {
                Ok(x.clone())
            } else {
                Err(NotFoundError::NotFound) 
            }
        }
    }

    impl<U : Default> Neighbor for AdjacencyMatrixGraph<U> {
        fn neighbors(self : &AdjacencyMatrixGraph<U>, vertex_id: &i32) -> Result<HashMap<i32, f64>, NotFoundError> {
            let num_columns = self.adj_matrix.columns();
            let mut map = HashMap::new();
            for i in 0..num_columns {
                let weight = self.adj_matrix.get((*vertex_id as usize, i));
                if weight != 0.0 {
                    map.insert(i as i32, weight);
                }
            }
            Ok(map)
        }
    }

    pub trait GraphMutate {
        fn add_edge(&mut self, e: Edge);
    }

    impl<U : Default> GraphMutate for AdjacencyListGraph<U> {
        fn add_edge(self: &mut AdjacencyListGraph<U>, e: Edge) {
            let mut v1 = e.v1.to_owned();
            let mut v2 = e.v2.to_owned();
            if !self.graph.is_directed {
                v1 = std::cmp::min(e.v1.to_owned(), e.v2.to_owned());
                v2 = std::cmp::max(e.v1.to_owned(), e.v2.to_owned());
            }

            // make default vertices if they don't exist already
            if let None = self.graph.vertices.get(&v1) {
                let new_v = Vertex {
                    id: v1.to_owned(),
                    info: U::default(),
                    weight: 0.0,
                };
                self.graph.vertices.insert(v1, new_v);
            }
            if let None = self.graph.vertices.get(&v2) {
                let new_v = Vertex {
                    id: v2.to_owned(),
                    info: U::default(),
                    weight: 0.0,
                };
                self.graph.vertices.insert(v2, new_v);
            }
            if let Some(set) = self.adj_list.get_mut(&v1) {
                set.insert(v2, e.weight.to_owned());
            }
            if !self.graph.is_directed {
                if let Some(set) = self.adj_list.get_mut(&v2) {
                    set.insert(v2, e.weight.to_owned());
                }
            }
        }
    }
}