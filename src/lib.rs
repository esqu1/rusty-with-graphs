extern crate matrix;

pub mod graph {
    use matrix::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Vertex<T: Default> {
        pub id: i32,
        pub info: T,
        pub weight: f64, // for max flows, ignored if used for weighted graphs
    }

    pub struct Edge {
        pub weight: f64,
        pub v1: i32,
        pub v2: i32, // for undirected graphs, enforce that v1 < v2
    }

    pub struct GraphBase<U: Default> {
        pub vertices: HashMap<i32, Vertex<U>>,
        pub is_directed: bool,
    }

    pub struct AdjacencyListGraph<U: Default> {
        pub graph: GraphBase<U>,
        pub adj_list: HashMap<i32, HashMap<i32, f64>>, // (v_id, wt)
    }

    pub struct AdjacencyMatrixGraph<U: Default> {
        pub graph: GraphBase<U>,
        pub adj_matrix: Compressed<f64>,
    }

    pub trait Graph<U: Default> {
        fn neighbors(&self, vertex_id: &i32) -> Option<HashMap<i32, f64>>;
        fn degree(&self, vertex_id: &i32) -> Option<usize>;
        fn get_vertices(&self) -> &HashMap<i32, Vertex<U>>;
    }

    impl<U: Default> Graph<U> for AdjacencyListGraph<U> {
        fn get_vertices(self: &Self) -> &HashMap<i32, Vertex<U>> {
            &self.graph.vertices
        }
        fn neighbors(self: &AdjacencyListGraph<U>, vertex_id: &i32) -> Option<HashMap<i32, f64>> {
            if let Some(x) = self.adj_list.get(vertex_id) {
                Some(x.clone())
            } else {
                None
            }
        }

        fn degree(self: &AdjacencyListGraph<U>, vertex_id: &i32) -> Option<usize> {
            self.adj_list.get(vertex_id).map(|x| x.len())
        }
    }

    /**
     * Assumes number of vertices = maximum ID + 1
     */
    impl<U: Default> Graph<U> for AdjacencyMatrixGraph<U> {
        fn get_vertices(self: &Self) -> &HashMap<i32, Vertex<U>> {
            &self.graph.vertices
        }
        fn neighbors(self: &AdjacencyMatrixGraph<U>, vertex_id: &i32) -> Option<HashMap<i32, f64>> {
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

        fn degree(self: &AdjacencyMatrixGraph<U>, vertex_id: &i32) -> Option<usize> {
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
    }

    pub trait GraphMutate {
        fn add_edge(&mut self, e: Edge);
    }

    impl<U: Default> GraphMutate for GraphBase<U> {
        fn add_edge(self: &mut GraphBase<U>, e: Edge) {
            let mut v1 = e.v1.to_owned();
            let mut v2 = e.v2.to_owned();
            if !self.is_directed {
                v1 = std::cmp::min(e.v1.to_owned(), e.v2.to_owned());
                v2 = std::cmp::max(e.v1.to_owned(), e.v2.to_owned());
            }

            // make default vertices if they don't exist already
            if let None = self.vertices.get(&v1) {
                let new_v = Vertex {
                    id: v1.to_owned(),
                    info: U::default(),
                    weight: 0.0,
                };
                self.vertices.insert(v1, new_v);
            }
            if let None = self.vertices.get(&v2) {
                let new_v = Vertex {
                    id: v2.to_owned(),
                    info: U::default(),
                    weight: 0.0,
                };
                self.vertices.insert(v2, new_v);
            }
        }
    }

    impl<U: Default> GraphMutate for AdjacencyListGraph<U> {
        fn add_edge(self: &mut AdjacencyListGraph<U>, e: Edge) {
            let v1 = e.v1;
            let v2 = e.v2;

            if let Some(set) = self.adj_list.get_mut(&v1) {
                set.insert(v2, e.weight.to_owned());
            } else {
                let mut new_map = HashMap::new();
                new_map.insert(v2, e.weight.to_owned());
                self.adj_list.insert(v1, new_map);
            }
            if !self.graph.is_directed {
                if let Some(set) = self.adj_list.get_mut(&v2) {
                    set.insert(v1, e.weight.to_owned());
                } else {
                    let mut new_map = HashMap::new();
                    new_map.insert(v1, e.weight.to_owned());
                    self.adj_list.insert(v2, new_map);
                }
            }
            self.graph.add_edge(e);
        }
    }

    impl<U: Default> GraphMutate for AdjacencyMatrixGraph<U> {
        fn add_edge(self: &mut AdjacencyMatrixGraph<U>, e: Edge) {
            let mut v1 = e.v1.to_owned();
            let mut v2 = e.v2.to_owned();
            if !self.graph.is_directed {
                v1 = std::cmp::min(e.v1.to_owned(), e.v2.to_owned());
                v2 = std::cmp::max(e.v1.to_owned(), e.v2.to_owned());
            }

            self.adj_matrix.set((v1 as usize, v2 as usize), e.weight);
            if !self.graph.is_directed {
                self.adj_matrix.set((v2 as usize, v2 as usize), e.weight);
            }
            self.graph.add_edge(e);
        }
    }
}
