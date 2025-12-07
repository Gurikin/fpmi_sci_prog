use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Id(pub usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Id(value)
    }
}

impl From<Id> for usize {
    fn from(val: Id) -> Self {
        val.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vertex {
    pub id: Id,
    pub x: f64,
    pub y: f64,
}

impl Vertex {
    pub fn new(id: Id, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Edge {
    pub from: Id,
    pub to: Id,
    pub weight: f64,
}
impl Edge {
    pub fn new(v: Id, w: Id, weight: f64) -> Self {
        Self {
            from: v,
            to: w,
            weight,
        }
    }
}

pub trait GraphTrait {
    fn v(&self) -> usize;
    fn e(&self) -> usize;
    fn directed(&self) -> bool;
    fn insert(&mut self, edge: &Edge);
    fn remove(&mut self, edge: &Edge);
    fn edge(&self, v_id: Id, w_id: Id) -> Option<Edge>;
    fn edges(&self, v_id: Id) -> Option<BTreeMap<Id, Edge>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DenseMatrixGraph {
    v_cnt: usize,
    e_cnt: usize,
    directed: bool,
    matrix: BTreeMap<Id, BTreeMap<Id, Edge>>,
}

impl GraphTrait for DenseMatrixGraph {
    fn v(&self) -> usize {
        self.v_cnt
    }

    fn e(&self) -> usize {
        self.e_cnt
    }

    fn directed(&self) -> bool {
        self.directed
    }

    fn insert(&mut self, edge: &Edge) {
        let v = edge.from;
        let w = edge.to;
        if !self
            .matrix
            .iter()
            .any(|(from, from_edges)| from == &v && from_edges.contains_key(&w))
        {
            self.e_cnt += 1;
        }
        match self.matrix.get_mut(&v) {
            Some(v_edges) => {
                v_edges.insert(w, *edge);
            }
            None => {
                let v_edges = BTreeMap::new();
                self.matrix.insert(v, v_edges);
            }
        }

        match self.matrix.get_mut(&w) {
            Some(w_edges) => {
                w_edges.insert(w, *edge);
            }
            None => {
                let w_edges = BTreeMap::new();
                self.matrix.insert(w, w_edges);
            }
        }
    }

    fn remove(&mut self, edge: &Edge) {
        let v = edge.from;
        let w = edge.to;
        if self
            .matrix
            .get(&v)
            .is_some_and(|v_edges| v_edges.contains_key(&w))
        {
            self.e_cnt -= 1;
        }
        self.matrix
            .get_mut(&v)
            .and_then(|v_edges| v_edges.remove(&w));
        if !self.directed {
            self.matrix
                .get_mut(&w)
                .and_then(|w_edges| w_edges.remove(&v));
        }
    }

    fn edge(&self, v_id: Id, w_id: Id) -> Option<Edge> {
        self.matrix
            .get(&v_id)
            .and_then(|v_edges| v_edges.get(&w_id))
            .copied()
    }

    fn edges(&self, v_id: Id) -> Option<BTreeMap<Id, Edge>> {
        self.matrix.get(&v_id).cloned()
    }
}

impl DenseMatrixGraph {
    pub fn from_points(vertices: Vec<Vertex>, directed: bool) -> Self {
        let mut matrix: BTreeMap<Id, BTreeMap<Id, Edge>> = BTreeMap::new();
        for curr_v in vertices.iter() {
            let v = curr_v;
            matrix.insert(v.id, BTreeMap::new());
            for curr_w in vertices.iter() {
                let w = curr_w;
                let weight = DenseMatrixGraph::dist_between_vertices(v, w);
                matrix.get_mut(&v.id).and_then(|v_edges| {
                    v_edges.insert(
                        w.id,
                        Edge {
                            from: v.id,
                            to: w.id,
                            weight,
                        },
                    )
                });

                matrix.get_mut(&w.id).and_then(|w_edges| {
                    w_edges.insert(
                        v.id,
                        Edge {
                            from: w.id,
                            to: v.id,
                            weight,
                        },
                    )
                });
            }
        }
        DenseMatrixGraph {
            v_cnt: vertices.len(),
            e_cnt: matrix.len(),
            directed,
            matrix,
        }
    }

    pub fn from_points_with_neighbors(vertices: Vec<Vertex>, directed: bool) -> Self {
        let mut matrix: BTreeMap<Id, BTreeMap<Id, Edge>> = BTreeMap::new();
        for (curr_idx, curr_v) in vertices.iter().enumerate() {
            let v = curr_v;
            matrix.insert(v.id, BTreeMap::new());
            for curr_w in DenseMatrixGraph::get_neighbors(curr_idx, 8, &vertices).iter() {
                let w = curr_w;
                let length = DenseMatrixGraph::dist_between_vertices(v, w);
                matrix.get_mut(&v.id).and_then(|v_edges| {
                    v_edges.insert(
                        w.id,
                        Edge {
                            from: v.id,
                            to: w.id,
                            weight: length,
                        },
                    )
                });
                if !directed {
                    matrix.get_mut(&v.id).and_then(|v_edges| {
                        v_edges.insert(
                            w.id,
                            Edge {
                                from: v.id,
                                to: w.id,
                                weight: length,
                            },
                        )
                    });
                }
            }
        }
        DenseMatrixGraph {
            v_cnt: vertices.len(),
            e_cnt: matrix.len(),
            directed,
            matrix,
        }
    }

    fn get_neighbors(n: usize, neighbor_cnt: usize, vertices: &[Vertex]) -> Vec<Vertex> {
        let n = n as i32;
        let neighbor_cnt = neighbor_cnt as i32;

        let mut result = vec![];

        for i in 0..=(neighbor_cnt / 2) {
            let left_n: i32 = if (n - i) >= 0 {
                n - i
            } else {
                vertices.len() as i32 - i - 1
            };
            let right_n = if (n + i) < (vertices.len() as i32 - 1) {
                n + i
            } else {
                i
            };
            let left_n = left_n as usize;
            let right_n = right_n as usize;
            result.push(vertices[left_n].clone());
            result.push(vertices[right_n].clone());
        }
        result
    }

    fn dist_between_vertices(v: &Vertex, w: &Vertex) -> f64 {
        let x = f64::abs(v.x - w.x);
        let y = f64::abs(v.y - w.y);
        f64::sqrt(x.powi(2) + y.powi(2))
    }
}
