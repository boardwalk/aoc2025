use anyhow::{Error, bail};
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Vertex {
    x: u64,
    y: u64,
    z: u64,
}

fn length_squared(a: &Vertex, b: &Vertex) -> u64 {
    let x_diff = a.x.abs_diff(b.x);
    let y_diff = a.y.abs_diff(b.y);
    let z_diff = a.z.abs_diff(b.z);

    x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
}

fn color_component(
    adj_lists: &Vec<Vec<usize>>,
    start_vertex: usize,
    vertex_colors: &mut Vec<Option<usize>>,
    next_color: &mut usize,
) {
    let mut seen = HashSet::new();
    let mut this_color: Option<usize> = None;

    let mut to_visit = vec![start_vertex];

    while let Some(vertex) = to_visit.pop() {
        if seen.contains(&vertex) {
            continue;
        }

        if vertex_colors[vertex].is_none() {
            let this_color = this_color.get_or_insert_with(|| {
                let c = *next_color;
                *next_color += 1;
                c
            });

            vertex_colors[vertex] = Some(*this_color);
        }

        seen.insert(vertex);
        to_visit.extend(&adj_lists[vertex]);
    }
}

fn component_size(adj_lists: &Vec<Vec<usize>>, vertex: usize) -> usize {
    let mut seen = HashSet::new();

    let mut to_visit = vec![vertex];

    while let Some(vertex) = to_visit.pop() {
        if seen.contains(&vertex) {
            continue;
        }

        seen.insert(vertex);

        to_visit.extend(&adj_lists[vertex]);
    }

    seen.len()
}

fn update_reachable(
    adj_lists: &Vec<Vec<usize>>,
    reachable_mat: &mut ndarray::Array2<bool>,
    start_vertex: usize,
) {
    let mut seen = HashSet::new();

    let mut to_visit = vec![start_vertex];

    while let Some(cur_vertex) = to_visit.pop() {
        if seen.contains(&cur_vertex) {
            continue;
        }

        seen.insert(cur_vertex);
        // update reachable_mat
        // our graph is undirected so both directions are true
        reachable_mat[(start_vertex, cur_vertex)] = true;
        reachable_mat[(cur_vertex, start_vertex)] = true;
        to_visit.extend(&adj_lists[cur_vertex]);
    }
}

fn main() -> Result<(), Error> {
    let mut vertices = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;

        let tokens = line.split(',').collect::<Vec<_>>();

        if tokens.len() != 3 {
            bail!("wrong number of tokens on line");
        }

        let x = u64::from_str_radix(tokens[0], 10)?;
        let y = u64::from_str_radix(tokens[1], 10)?;
        let z = u64::from_str_radix(tokens[2], 10)?;

        vertices.push(Vertex { x, y, z });
    }

    // println!("vertices = {vertices:#?}");

    // created a list of all possible edges
    let mut edges = Vec::new();

    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            edges.push((i, j));
        }
    }

    // println!("vertices.len() = {}", vertices.len());

    // sort possible edges by length
    edges.sort_by_cached_key(|(i, j)| length_squared(&vertices[*i], &vertices[*j]));

    // println!("edges = {edges:#?}");

    // "vertex [i] has an edge from it these vertices"
    let mut adj_lists: Vec<Vec<usize>> = Vec::new();

    adj_lists.resize_with(vertices.len(), Default::default);

    // create reachable_mat and set diagonal to true (a vertex can always reach itself)
    let mut reachable_mat: ndarray::Array2<bool> =
        Array2::default((vertices.len(), vertices.len()));

    for vertex in 0..vertices.len() {
        reachable_mat[(vertex, vertex)] = true;
    }

    // insert the ten shortest edges
    let mut edges_left = 10;

    for (i, j) in &edges {
        // don't connect anything that's already reachable
        if reachable_mat[(*i, *j)] {
            println!("not connecting {} {}", *i, *j);
            continue;
        }

        // println!("connecting {:?} and {:?}", vertices[*i], vertices[*j]);

        adj_lists[*i].push(*j);
        adj_lists[*j].push(*i);

        update_reachable(&adj_lists, &mut reachable_mat, *i); // i and j will both do

        edges_left -= 1;

        if edges_left == 0 {
            break;
        }
    }

    assert_eq!(edges_left, 0);

    let mut next_color = 1;
    let mut vertex_colors: Vec<Option<usize>> = Vec::new();

    // start with all the vertices uncolored
    vertex_colors.resize_with(vertices.len(), Default::default);

    for vertex in 0..vertices.len() {
        color_component(&adj_lists, vertex, &mut vertex_colors, &mut next_color);
    }
    println!("vertex_colors = {vertex_colors:?}");

    let mut color_to_size = HashMap::new();

    for vertex in 0..vertices.len() {
        color_to_size.insert(
            vertex_colors[vertex].unwrap(),
            component_size(&adj_lists, vertex),
        );
    }

    let mut color_to_size = color_to_size.into_iter().collect::<Vec<_>>();

    color_to_size.sort_by_cached_key(|(_color, size)| *size);

    color_to_size.reverse();

    let mut result = 1;

    for (_color, size) in color_to_size.iter().take(3) {
        result *= size;
    }

    // println!("color_to_size = {color_to_size:?}");
    println!("result = {result}");

    Ok(())
}
