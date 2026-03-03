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
            if u == v {
                endpoints.0 == v && endpoints.1 == v
            } else {
                endpoints.0 == v || endpoints.1 == v
            }
        })
    }
}

impl<T: GfaNodeData> NamedNodeData for T {
    fn name(&'_ self) -> std::borrow::Cow<'_, str> {
        GfaNodeData::name(self)
    }
}

#[cfg(test)]
mod tests {
    use bidirected_adjacency_array::graph::{BidirectedAdjacencyArray, BidirectedEdge};
    use itertools::Itertools;
    use tagged_vec::TaggedVec;

    use crate::graph::{NamedNodeData, StaticGraph};

    #[test]
    fn test_edges_between() {
        struct NodeData(u8);
        impl NamedNodeData for NodeData {
            fn name(&'_ self) -> std::borrow::Cow<'_, str> {
                std::borrow::Cow::Owned(format!("n{}", self.0))
            }
        }

        let nodes = [1, 2, 3].into_iter().map(NodeData).collect();
        let edges = TaggedVec::from(vec![
            BidirectedEdge::new(3.into(), 0.into(), 1),
            BidirectedEdge::new(0.into(), 1.into(), 2),
            BidirectedEdge::new(4.into(), 4.into(), 3),
            BidirectedEdge::new(0.into(), 4.into(), 4),
            BidirectedEdge::new(0.into(), 3.into(), 5),
        ]);

        let graph = BidirectedAdjacencyArray::<u8, NodeData, u8>::new(nodes, edges);

        for n1 in graph.node_indices() {
            for n2 in graph.node_indices() {
                let edges_between = graph.edges_between(n1, n2).sorted().collect::<Vec<_>>();
                let edges_between_reversed =
                    graph.edges_between(n2, n1).sorted().collect::<Vec<_>>();
                assert_eq!(edges_between, edges_between_reversed);
            }
        }

        assert_eq!(
            graph
                .edges_between(0.into(), 0.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![1.into()]
        );
        assert_eq!(
            graph
                .edges_between(0.into(), 1.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![0.into(), 4.into()]
        );
        assert_eq!(
            graph
                .edges_between(0.into(), 2.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![3.into()]
        );
        assert_eq!(
            graph
                .edges_between(1.into(), 1.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            graph
                .edges_between(1.into(), 2.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            graph
                .edges_between(2.into(), 2.into())
                .sorted()
                .collect::<Vec<_>>(),
            vec![2.into()]
        );
    }
}
