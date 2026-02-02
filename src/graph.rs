use std::borrow::Cow;

use crate::decomposition::indices::{GraphIndex, GraphIndexInteger};

pub mod implementations;

/// An undirected graph without multiedges or self-loops.
pub trait StaticGraph {
    /// The integer type used for indices in this graph.
    type IndexType: GraphIndexInteger;

    type NodeIndex: GraphIndex;

    type EdgeIndex: GraphIndex;

    /// Returns an iterator over all node indices in the graph.
    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIndex>;

    /// Returns an iterator over all edge indices in the graph.
    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIndex>;

    /// Returns the number of nodes in the graph.
    fn node_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;

    /// Returns the index of the node with the given name.
    fn node_index_from_name(&self, name: &str) -> Option<Self::NodeIndex>;

    /// Returns the index of the edge with the given name.
    fn edge_index_from_name(&self, name: &str) -> Option<Self::EdgeIndex>;

    /// Returns the name of the given node.
    fn node_name(&self, node_index: Self::NodeIndex) -> Cow<'_, str>;

    /// Returns the name of the given edge.
    fn edge_name(&self, edge_index: Self::EdgeIndex) -> Cow<'_, str>;

    /// Returns an iterator over the incident edges of the given node.
    ///
    /// Each edge is returned exactly once.
    fn incident_edges(&self, node: Self::NodeIndex) -> impl Iterator<Item = Self::EdgeIndex>;

    /// Returns the endpoints of the given edge as a tuple of node indices.
    fn edge_endpoints(&self, edge: Self::EdgeIndex) -> (Self::NodeIndex, Self::NodeIndex);

    /// Returns the edges between the two given nodes.
    fn edges_between(
        &self,
        u: Self::NodeIndex,
        v: Self::NodeIndex,
    ) -> impl Iterator<Item = Self::EdgeIndex>;
}

pub trait NamedNodeData {
    fn name(&'_ self) -> Cow<'_, str>;
}

pub trait NamedEdgeData {
    fn name(&'_ self) -> Cow<'_, str>;
}
