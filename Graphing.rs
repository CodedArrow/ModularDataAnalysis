pub struct BasicApp {
    g: egui_graphs::Graph,
}

impl BasicApp {
    fn new(_: &CreationContext<'_>) -> Self {
        Self { g: generate_graph() }
    }
}

fn generate_graph() -> egui_graphs::Graph {
    let mut g = petgraph::stable_graph::StableGraph::new();

let i=0;
    for file in files {
    i=i+1;
    let node = g.add_node(());
    
    Graph::from(&g)
}
}