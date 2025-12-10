use std::collections::{HashMap, VecDeque};
use std::f64;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use graph::{graph::*, mst::*};

fn main() {
    let (mst, _) = calc_prim_mst();
    for edge in mst.mst.into_iter().flatten() {
        println!("[{}] -> [{}]", edge.from.0, edge.to.0);
    }
    println!("Total weight of Prim MST: {}", mst.total_weight);
    let (mst, _) = calc_prim_mst_with_const_edges();
    for edge in mst.mst.into_iter().flatten() {
        println!("[{}] -> [{}]", edge.from.0, edge.to.0);
    }
    println!("Total weight of MST with const edges: {}", mst.total_weight);
}

fn prepare_mst_to_plot<T: GraphTrait>(
    mst: MST<T>,
    vertices_map: HashMap<Id, Vertex>,
) -> (Vec<graph::graph::Vertex>, Vec<(f32, f32)>) {
    let mut mst_vec: VecDeque<(Id, Id)> = mst
        .mst
        .clone()
        .into_iter()
        .flatten()
        .map(|e| (e.from, e.to))
        .collect();
    let zero_edge = mst_vec.iter().find(|e| e.0 == Id(0)).cloned();
    if let Some(e) = zero_edge {
        mst_vec.push_front((e.1, e.0));
    }

    let mut mst_vertices: Vec<Vertex> = vec![];
    mst_vertices.insert(0, vertices_map.get(&mst_vec[1].1).unwrap().clone());
    mst_vertices.insert(1, vertices_map.get(&mst_vec[1].0).unwrap().clone());
    let mut idx = mst_vertices[1].id;

    while mst_vertices.len() <= mst_vec.len() - 1 {
        let from_tail_id = mst_vec.iter().find(|e| e.1 == idx);
        println!("from_tail_id: {:?}", idx.0);
        if from_tail_id.is_some() {
            let from_vertex = vertices_map.get(&from_tail_id.map(|id| id.0).unwrap());
            if let Some(v) = from_vertex {
                println!("to_vertex by tail: {:?}", v);
                mst_vertices.push(v.clone());
                idx = from_tail_id.map(|id| id.0).unwrap();
                println!("IDX: {}", idx.0);
            }
        }
    }
    let coords_vec: Vec<(f32, f32)> = mst_vertices
        .iter()
        .map(|v| (v.x as f32, v.y as f32))
        .collect();
    (mst_vertices, coords_vec)
}

fn calc_prim_mst_with_const_edges() -> (MST<DenseMatrixGraph>, HashMap<Id, Vertex>) {
    let mut vertices = calc_median();
    vertices.sort_by(|a, b| calc_avg(a.x, a.y).partial_cmp(&calc_avg(b.x, b.y)).unwrap());
    let graph = DenseMatrixGraph::from_points_with_neighbors(vertices.clone(), false);
    (
        MST::calc_mst(graph),
        vertices.iter().map(|v| (v.id, v.clone())).collect(),
    )
}

fn calc_median() -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = vec![];
    let mut id_cnt = 0_usize;
    let default_coord = f64::MIN;
    if let Ok(lines) = read_lines("./data/lines.txt") {
        for line in lines.map_while(Result::ok) {
            let (x, y) = line.split_once(" ").unwrap_or(("", ""));
            let (x, y) = (
                x.parse::<f64>().unwrap_or(default_coord),
                y.parse::<f64>().unwrap_or(default_coord),
            );
            let v = Vertex::new(Id(id_cnt), x, y);
            vertices.push(v);
            id_cnt += 1;
        }
    }
    vertices
}

fn calc_avg(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn calc_prim_mst() -> (MST<DenseMatrixGraph>, HashMap<Id, Vertex>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut vertices_map: HashMap<Id, Vertex> = HashMap::new();
    let mut id_cnt = 0_usize;
    let default_coord = f64::MIN;
    if let Ok(lines) = read_lines("./data/lines.txt") {
        for line in lines.map_while(Result::ok) {
            let (x, y) = line.split_once(" ").unwrap_or(("", ""));
            let (x, y) = (
                x.parse::<f64>().unwrap_or(default_coord),
                y.parse::<f64>().unwrap_or(default_coord),
            );
            let v = Vertex::new(Id(id_cnt), x, y);
            vertices.push(v.clone());
            vertices_map.insert(v.id, v);
            id_cnt += 1;
        }
    }
    let graph = DenseMatrixGraph::from_points(vertices, false);
    // let ser_graph = serde_json::to_string(&graph);
    // println!("Graph: {:?}", ser_graph.unwrap());
    // println!("Graph: {:?}", graph);
    (MST::calc_mst(graph), vertices_map)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[test]
fn prim_mst_test() {
    for step in 1..=6 {
        let n = 5_usize.pow(step);
        test_mst(n);
    }
}

fn test_mst(n: usize) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut vertices: Vec<Vertex> = vec![];
    for id_cnt in 0..n {
        let range = 0.0..3e+5;
        let x = rand::random_range(range.clone());
        let y = rand::random_range(range);
        let v = Vertex::new(Id(id_cnt), x, y);
        vertices.push(v);
    }
    let graph = DenseMatrixGraph::from_points(vertices, false);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let prepare_time = end.as_millis() - start.as_millis();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let _ = MST::calc_mst(graph);
    // println!("MST: {:?}", mst.mst);
    // for edge in mst.mst.into_iter().flatten() {
    //     println!("[{}] -> [{}]", edge.from.0, edge.to.0);
    // }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mst_time = end.as_millis() - start.as_millis();

    println!(
        "Prepare data and build graph: {} milliseconds;\nCalculate mst with N = {} vertices: {} milliseconds.",
        prepare_time, n, mst_time
    );
}

#[test]
fn prim_mst_with_const_edges_test() {
    for step in 1..=6 {
        let n = 5_usize.pow(step);
        test_mst_with_const_edges(n);
    }
}

fn test_mst_with_const_edges(n: usize) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut vertices: Vec<Vertex> = vec![];
    for id_cnt in 0..n {
        let range = 0.0..3e+5;
        let x = rand::random_range(range.clone());
        let y = rand::random_range(range);
        let v = Vertex::new(Id(id_cnt), x, y);
        vertices.push(v);
    }
    vertices.sort_by(|a, b| calc_avg(a.x, a.y).partial_cmp(&calc_avg(b.x, b.y)).unwrap());
    let graph = DenseMatrixGraph::from_points_with_neighbors(vertices, false);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let prepare_time = end.as_millis() - start.as_millis();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let _ = MST::calc_mst(graph);
    // println!("MST: {:?}", mst.mst);
    // for edge in mst.mst.into_iter().flatten() {
    //     println!("[{}] -> [{}]", edge.from.0, edge.to.0);
    // }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mst_time = end.as_millis() - start.as_millis();

    println!(
        "Prepare data and build graph: {} milliseconds;\nCalculate mst with N = {} vertices: {} milliseconds.",
        prepare_time, n, mst_time
    );
}
