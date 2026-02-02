use bidirected_adjacency_array::{
    graph::BidirectedAdjacencyArray,
    index::{EdgeIndex, GraphIndexInteger, NodeIndex},
};

use crate::graph::StaticGraph;

impl<IndexType: GraphIndexInteger, NodeData, EdgeData> StaticGraph
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

    fn node_index_from_name(&self, name: &str) -> Option<Self::NodeIndex> {
        let name = if name.starts_with('N') {
            &name[name.char_indices().nth(1).unwrap().0..]
        } else {
            name
        };
        let index = name.parse::<usize>().ok()?;

        if index < self.node_count() {
            Some(Self::NodeIndex::from_usize(index))
        } else {
            None
        }
    }

    fn edge_index_from_name(&self, name: &str) -> Option<Self::EdgeIndex> {
        let name = if name.starts_with('E') {
            &name[name.char_indices().nth(1).unwrap().0..]
        } else {
            name
        };
        let index = name.parse::<usize>().ok()?;

        if index < self.node_count() {
            Some(Self::EdgeIndex::from_usize(index))
        } else {
            None
        }
    }

    fn node_name(&self, node_index: Self::NodeIndex) -> std::borrow::Cow<'_, str> {
        format!("N{node_index}").into()
    }

    fn edge_name(&self, edge_index: Self::EdgeIndex) -> std::borrow::Cow<'_, str> {
        format!("E{edge_index}").into()
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
