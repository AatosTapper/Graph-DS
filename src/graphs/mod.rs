
/*
GRAPH - Aatos Tapper 7.10.23

Example usecase:

fn main() {
    let mut my_graph = Graph::new();

    // add vertices to the graph
    let vertex1: usize = my_graph.add_vertex(Vertex::new(10.0));
    let vertex2: usize = my_graph.add_vertex(Vertex::new(10.0));

    // access vertices by their indices
    if let Some(v1) = my_graph.get_vertex(vertex1) {
        println!("Vertex 1 data: {}", v1.data);
    }

    if let Some(v2) = my_graph.get_vertex(vertex2) {
        println!("Vertex 2 data: {}", v2.data);
    }

    // print all neighbors
    let connected_vertices: Vec<usize> = my_graph.neighbors(vertex1);
    for n in connected_vertices {
        if let Some(v) = my_graph.get_vertex(n) {
            println!("Vertex data: {}", v.data);
        }
    }
}

*/

#[warn(dead_code)]

#[warn(dead_code)]

#[derive(Debug)]
pub struct Vertex {
    pub data: f32
}

impl Vertex {
    pub fn new(x: f32) -> Self {
        Vertex { data: x }
    }
}

#[derive(Debug)]
pub enum GraphNode {
    On(Vertex),
    Off,
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<GraphNode>,
    adjacency_matrix: Vec<Vec<i8>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            adjacency_matrix: Vec::new(),
        }
    }

    pub fn get_vertex(&mut self, index: usize) -> Option<&mut Vertex> {
        if let Some(GraphNode::On(vertex)) = self.vertices.get_mut(index) {
            Some(vertex)
        } else {
            None
        }
    }

    pub fn add_vertex(&mut self, x: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(GraphNode::On(x));

        // expand the adjacency matrix
        for row in self.adjacency_matrix.iter_mut() {
            row.push(0);
        }
        self.adjacency_matrix.push(vec![0; index + 1]);

        index
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.vertices.len() && to < self.vertices.len() {
            if let (GraphNode::On(_), GraphNode::On(_)) = (&self.vertices[from], &self.vertices[to]) {
                self.adjacency_matrix[from][to] = 1;
            }
        }
    }

    pub fn remove_vertex(&mut self, index: usize) {
        if index < self.vertices.len() {
            self.vertices[index] = GraphNode::Off;
            for it in self.adjacency_matrix.iter_mut() {
                it[index] = -1;
            }
        }
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from < self.vertices.len() && to < self.vertices.len() {
            self.adjacency_matrix[from][to] = 0;
        }
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        if from < self.vertices.len() && to < self.vertices.len() {
            if let (GraphNode::On(_), GraphNode::On(_)) = (&self.vertices[from], &self.vertices[to]) {
                return self.adjacency_matrix[from][to] == 1;
            }
        }
        false
    }

    pub fn neighbors(&self, index: usize) -> Vec<usize> {
        if index < self.vertices.len() {
            if let GraphNode::On(_) = &self.vertices[index] {
                return self.adjacency_matrix[index]
                    .iter()
                    .enumerate()
                    .filter(|(_, &weight)| weight == 1)
                    .map(|(i, _)| i)
                    .collect();
            }
        }
        Vec::new()
    }

    pub fn print_vertices(&self) {
        for (v, vertex) in self.vertices.iter().enumerate() {
            match vertex {
                GraphNode::On(vertex) => {
                    println!("{} {:?}", v, vertex);
                    print!("Edges: ");
                    for i in 0..self.adjacency_matrix[v].len() {
                        if self.adjacency_matrix[v][i] > 0 {
                            print!("{}, ", i);
                        }
                    }
                    print!("\n\n");
                }
                GraphNode::Off => continue,
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
