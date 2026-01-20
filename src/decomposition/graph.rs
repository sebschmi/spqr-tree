use num_traits::bounds::UpperBounded;

pub trait StaticGraph {
    type NodeIndex: Copy
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + From<usize>
        + Into<usize>
        + UpperBounded;

    /// Returns the number of nodes in the graph.
    fn node_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;
}
