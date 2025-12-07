use crate::graph::{DenseMatrixGraph, Edge, GraphTrait, Id};

#[derive(Debug)]
pub struct MST<T: GraphTrait> {
    pub g: T,
    pub mst: Vec<Option<Edge>>,
    pub total_weight: f64,
}

pub trait PrimMST<T: GraphTrait> {
    fn calc_mst(graph: T) -> Self;
}

impl PrimMST<DenseMatrixGraph> for MST<DenseMatrixGraph> {
    fn calc_mst(graph: DenseMatrixGraph) -> Self {
        let v = graph.v();
        let mut in_mst = vec![false; v];
        let mut min_edge = vec![f64::MAX; v];
        let mut parent = vec![None; v];
        let mut mst_edges = vec![None; v];
        let mut cnt = 0;

        // Начинаем с вершины 0
        min_edge[0] = 0.0;

        for _ in 0..v {
            // Находим вершину с минимальным весом, ещё не входящую в MST
            let mut u = None;
            for i in 0..v {
                if !in_mst[i] && (u.is_none() || min_edge[i] < min_edge[u.unwrap()]) {
                    u = Some(i);
                }
                cnt += 1;
            }

            let u = u.unwrap();
            in_mst[u] = true;

            // Если есть родитель, добавляем ребро в MST
            if let Some(p) = parent[u] {
                mst_edges[u] = Some(Edge::new(p, Id(u), min_edge[u]));
            }

            // Обновляем веса соседних вершин
            if let Some(map) = graph.edges(u.into()) {
                map.iter().for_each(|(_, edge)| {
                    let w = edge.to.0;
                    if !in_mst[w] && edge.weight < min_edge[w] {
                        min_edge[w] = edge.weight;
                        parent[w] = Some(u.into());
                    }
                    cnt += 1;
                })
            }
        }
        println!("TOTAL OPERATION: {}", cnt);

        // Вычисляем общий вес
        let total_weight = mst_edges
            .iter()
            .filter_map(|e| e.as_ref())
            .map(|e| e.weight)
            .sum::<f64>();

        Self {
            g: graph,
            mst: mst_edges,
            total_weight,
        }
    }
}
