use std::collections::BTreeMap;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

#[derive(Debug, Clone)]
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

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
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

#[derive(Debug)]
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
            }
        }
        DenseMatrixGraph {
            v_cnt: vertices.len(),
            e_cnt: if directed {
                matrix.len()
            } else {
                matrix.len() / 2
            },
            directed,
            matrix,
        }
    }

    pub fn dist_between_vertices(v: &Vertex, w: &Vertex) -> f64 {
        let x = f64::abs(v.x - w.x);
        let y = f64::abs(v.y - w.y);
        f64::sqrt(x.powi(2) + y.powi(2))
    }
}
