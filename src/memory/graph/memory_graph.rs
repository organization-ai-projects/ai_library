use super::{node::MemoryNode, edge::MemoryEdge};

/// Structure représentant le graph mémoire.
pub struct MemoryGraph {
    pub nodes: Vec<MemoryNode>,
    pub edges: Vec<MemoryEdge>,
}

impl MemoryGraph {
    /// Ajoute un noeud au graph.
    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.push(node);
    }
    /// Ajoute une arête au graph.
    pub fn add_edge(&mut self, edge: MemoryEdge) {
        self.edges.push(edge);
    }
}
