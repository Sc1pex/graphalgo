use crate::graphstyle::GraphStyle;

#[derive(Debug, Clone)]
pub struct Graph {
    pub node_count: usize,
    pub edges: Vec<Vec<usize>>,

    pub style: GraphStyle,
}

impl Graph {
    pub fn new(node_count: usize) -> Self {
        Self {
            node_count,
            edges: vec![vec![]; node_count],

            style: GraphStyle::new(node_count),
        }
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize) {
        self.edges[node1].push(node2);
        self.edges[node2].push(node1);

        self.style.add_edge(node1, node2);
    }
}

impl From<String> for Graph {
    fn from(value: String) -> Self {
        let mut lines = value.trim().lines();
        let mut line = lines.next().unwrap().split_whitespace();
        let node_count = line.next().unwrap().parse::<usize>().unwrap();
        let mut graph = Graph::new(node_count);

        while let Some(line) = lines.next() {
            let mut line = line.split_whitespace();
            let node1 = line.next().unwrap().parse::<usize>().unwrap();
            let node2 = line.next().unwrap().parse::<usize>().unwrap();

            graph.add_edge(node1, node2);
        }

        graph
    }
}
