use bidirected_adjacency_array::{
    graph::BidirectedAdjacencyArray,
    index::{EdgeIndex, GraphIndexInteger, NodeIndex},
    io::gfa1::GfaNodeData,
};

use crate::graph::{NamedNodeData, StaticGraph};

impl<IndexType: GraphIndexInteger, NodeData: NamedNodeData, EdgeData> StaticGraph
    for BidirectedAdjacencyArray<IndexType, NodeData, EdgeData>
{
    type IndexType = IndexType;

    type NodeIndex = NodeIndex<IndexType>;

    type EdgeIndex = EdgeIndex<IndexType>;

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIndex> {
        self.iter_nodes()
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIndex> {
        self.iter_edges()
    }

    fn node_count(&self) -> usize {
        BidirectedAdjacencyArray::node_count(self)
    }

    fn edge_count(&self) -> usize {
        BidirectedAdjacencyArray::edge_count(self)
    }

    fn node_name(&self, node_index: Self::NodeIndex) -> std::borrow::Cow<'_, str> {
        self.node_data(node_index).name()
    }

    fn incident_edges(&self, node: Self::NodeIndex) -> impl Iterator<Item = Self::EdgeIndex> {
        self.iter_incident_edges(node)
    }

    fn edge_endpoints(&self, edge: Self::EdgeIndex) -> (Self::NodeIndex, Self::NodeIndex) {
        let edge_data = self.edge(edge);
        (
            edge_data.from().into_bidirected(),
            edge_data.to().into_bidirected(),
        )
    }

    fn edges_between(
        &self,
        u: Self::NodeIndex,
        v: Self::NodeIndex,
    ) -> impl Iterator<Item = Self::EdgeIndex> {
        self.iter_incident_edges(u).filter(move |edge| {
            let endpoints = self.edge_endpoints(*edge);
            endpoints.0 == v || endpoints.1 == v
        })
    }
}

impl<T: GfaNodeData> NamedNodeData for T {
    fn name(&'_ self) -> std::borrow::Cow<'_, str> {
        GfaNodeData::name(self)
    }
}
