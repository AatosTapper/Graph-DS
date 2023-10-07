
mod graphs;

fn main() {
    let mut data: graphs::Graph = graphs::Graph::new();

    let vertex1: usize = data.add_vertex(graphs::Vertex::new(0.0));
    let vertex2: usize = data.add_vertex(graphs::Vertex::new(0.0));
    let vertex3: usize = data.add_vertex(graphs::Vertex::new(0.0));
    let vertex4: usize = data.add_vertex(graphs::Vertex::new(0.0));

    // edges from vertex 1
    data.add_edge(vertex1, vertex2);
    data.add_edge(vertex1, vertex3);
    
    // edges from vertex 2
    data.add_edge(vertex2, vertex1);
    data.add_edge(vertex2, vertex3);
    data.add_edge(vertex2, vertex4);

    // edges from vertex 3
    data.add_edge(vertex3, vertex1);
    data.add_edge(vertex3, vertex4);

    // edges from vertex 4
    data.add_edge(vertex4, vertex1);
    data.add_edge(vertex4, vertex2);
    data.add_edge(vertex4, vertex3);
    data.add_edge(vertex4, vertex4);

    let connected_vertices1: Vec<usize> = data.neighbors(vertex1);
    let connected_vertices2: Vec<usize> = data.neighbors(vertex2);
    let connected_vertices3: Vec<usize> = data.neighbors(vertex3);
    let connected_vertices4: Vec<usize> = data.neighbors(vertex4);

    for n in connected_vertices1 {
        if let Some(v) = data.get_vertex(n) {
            v.data += 10.0;
        }
    }

    for n in connected_vertices2 {
        if let Some(v) = data.get_vertex(n) {
            v.data -= 9.81;
        }
    }

    for n in connected_vertices3 {
        if let Some(v) = data.get_vertex(n) {
            v.data -= 3.14;
        }
    }

    for n in connected_vertices4 {
        if let Some(v) = data.get_vertex(n) {
            v.data -= 2.17;
        }
    }

    data.print_vertices();
}
