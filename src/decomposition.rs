#![expect(clippy::type_complexity)]

use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::{
    decomposition::indices::{
        BlockIndex, ComponentIndex, CutNodeIndex, GraphIndex, GraphIndexInteger,
        OptionalCutNodeIndex, SPQREdgeIndex, SPQRNodeIndex,
    },
    graph::StaticGraph,
};

pub mod builder;
pub mod indices;

/// Represents the SPQR decomposition as an augmentation over a graph.
///
/// The SPQR decomposition is a hierarchical decomposition of the graph first into its connected components,
/// then into the biconnected components of each connected component,
/// and finally into the triconnected components of each biconnected component.
///
/// The decomposition of a connected component into its biconnected components is called the [block cut tree](https://en.wikipedia.org/wiki/Biconnected_component#Block-cut_tree).
/// The decomposition of a biconnected component into its triconnected components is called the [SPQR tree](https://en.wikipedia.org/wiki/SPQR_tree).
pub struct SPQRDecomposition<'graph, Graph: StaticGraph> {
    pub(crate) graph: &'graph Graph,
    pub(crate) components:
        TaggedVec<ComponentIndex<Graph::IndexType>, Component<Graph::NodeIndex, Graph::IndexType>>,
    pub(crate) blocks:
        TaggedVec<BlockIndex<Graph::IndexType>, Block<Graph::NodeIndex, Graph::IndexType>>,
    pub(crate) cut_nodes:
        TaggedVec<CutNodeIndex<Graph::IndexType>, CutNode<Graph::NodeIndex, Graph::IndexType>>,
    pub(crate) spqr_nodes: TaggedVec<
        SPQRNodeIndex<Graph::IndexType>,
        SPQRNode<Graph::NodeIndex, Graph::EdgeIndex, Graph::IndexType>,
    >,
    pub(crate) spqr_edges:
        TaggedVec<SPQREdgeIndex<Graph::IndexType>, SPQREdge<Graph::NodeIndex, Graph::IndexType>>,
    pub(crate) node_data: TaggedVec<Graph::NodeIndex, SPQRDecompositionNodeData<Graph::IndexType>>,
    pub(crate) edge_data: TaggedVec<Graph::EdgeIndex, SPQRDecompositionEdgeData<Graph::IndexType>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<NodeIndex, IndexType> {
    pub(crate) nodes: Vec<NodeIndex>,
    pub(crate) blocks: Vec<BlockIndex<IndexType>>,
    pub(crate) cut_nodes: Vec<CutNodeIndex<IndexType>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<NodeIndex, IndexType> {
    pub(crate) component: ComponentIndex<IndexType>,
    pub(crate) nodes: Vec<NodeIndex>,
    pub(crate) cut_nodes: Vec<CutNodeIndex<IndexType>>,
    pub(crate) spqr_nodes: Vec<SPQRNodeIndex<IndexType>>,
    pub(crate) spqr_edges: Vec<SPQREdgeIndex<IndexType>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutNode<NodeIndex, IndexType> {
    pub(crate) component: ComponentIndex<IndexType>,
    pub(crate) node: NodeIndex,
    pub(crate) adjacent_blocks: SmallVec<[BlockIndex<IndexType>; 2]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SPQRNode<NodeIndex, EdgeIndex, IndexType> {
    pub(crate) block: BlockIndex<IndexType>,
    pub(crate) nodes: Vec<NodeIndex>,
    pub(crate) edges: Vec<EdgeIndex>,
    pub(crate) spqr_node_type: SPQRNodeType,
    pub(crate) spqr_edges: SmallVec<[SPQREdgeIndex<IndexType>; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SPQRNodeType {
    SNode,
    PNode,
    RNode,
}

/// An edge in the SPQR tree connecting two SPQR nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SPQREdge<NodeIndex, IndexType> {
    pub(crate) endpoints: (SPQRNodeIndex<IndexType>, SPQRNodeIndex<IndexType>),
    pub(crate) virtual_edge: (NodeIndex, NodeIndex),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SPQRDecompositionNodeData<IndexType: GraphIndexInteger> {
    pub(crate) component_index: ComponentIndex<IndexType>,
    pub(crate) block_indices: SmallVec<[BlockIndex<IndexType>; 1]>,
    pub(crate) cut_node_index: OptionalCutNodeIndex<IndexType>,
    pub(crate) spqr_node_indices: SmallVec<[SPQRNodeIndex<IndexType>; 1]>,
    pub(crate) extra_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SPQRDecompositionEdgeData<IndexType> {
    pub(crate) component_index: ComponentIndex<IndexType>,
    pub(crate) block_index: BlockIndex<IndexType>,
    pub(crate) spqr_node_index: SPQRNodeIndex<IndexType>,
    pub(crate) extra_data: String,
}

impl<'graph, Graph: StaticGraph> SPQRDecomposition<'graph, Graph> {
    pub fn graph(&self) -> &'graph Graph {
        self.graph
    }

    pub fn iter_component_indices(&self) -> impl Iterator<Item = ComponentIndex<Graph::IndexType>> {
        self.components.iter_indices()
    }

    pub fn iter_components(
        &self,
    ) -> impl Iterator<
        Item = (
            ComponentIndex<Graph::IndexType>,
            &Component<Graph::NodeIndex, Graph::IndexType>,
        ),
    > {
        self.components.iter()
    }

    pub fn iter_blocks_in_component(
        &self,
        component_index: ComponentIndex<Graph::IndexType>,
    ) -> impl Iterator<
        Item = (
            BlockIndex<Graph::IndexType>,
            &Block<Graph::NodeIndex, Graph::IndexType>,
        ),
    > {
        self.components[component_index]
            .blocks
            .iter()
            .copied()
            .map(move |block_index| (block_index, &self.blocks[block_index]))
    }

    pub fn iter_spqr_nodes_in_block(
        &self,
        block_index: BlockIndex<Graph::IndexType>,
    ) -> impl Iterator<
        Item = (
            SPQRNodeIndex<Graph::IndexType>,
            &SPQRNode<Graph::NodeIndex, Graph::EdgeIndex, Graph::IndexType>,
        ),
    > {
        self.blocks[block_index]
            .spqr_nodes
            .iter()
            .copied()
            .map(move |spqr_node_index| (spqr_node_index, &self.spqr_nodes[spqr_node_index]))
    }

    pub fn iter_spqr_edges_in_block(
        &self,
        block_index: BlockIndex<Graph::IndexType>,
    ) -> impl Iterator<
        Item = (
            SPQREdgeIndex<Graph::IndexType>,
            &SPQREdge<Graph::NodeIndex, Graph::IndexType>,
        ),
    > {
        self.blocks[block_index]
            .spqr_edges
            .iter()
            .copied()
            .map(move |spqr_edge_index| (spqr_edge_index, &self.spqr_edges[spqr_edge_index]))
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = Graph::NodeIndex> {
        self.graph.node_indices()
    }

    pub fn node_extra_data(&self, node_index: Graph::NodeIndex) -> &str {
        &self.node_data[node_index].extra_data
    }

    pub fn edge_extra_data(&self, edge_index: Graph::EdgeIndex) -> &str {
        &self.edge_data[edge_index].extra_data
    }

    pub fn spqr_node_name(&self, spqr_node_index: SPQRNodeIndex<Graph::IndexType>) -> String {
        match self.spqr_nodes[spqr_node_index].spqr_node_type() {
            SPQRNodeType::SNode => format!("S{spqr_node_index}"),
            SPQRNodeType::PNode => format!("P{spqr_node_index}"),
            SPQRNodeType::RNode => format!("R{spqr_node_index}"),
        }
    }

    pub fn node_component_index(
        &self,
        node_index: Graph::NodeIndex,
    ) -> ComponentIndex<Graph::IndexType> {
        self.node_data[node_index].component_index
    }

    pub fn node_block_indices(
        &self,
        node_index: Graph::NodeIndex,
    ) -> impl Iterator<Item = BlockIndex<Graph::IndexType>> {
        self.node_data[node_index].block_indices.iter().copied()
    }

    pub fn node_spqr_node_indices(
        &self,
        node_index: Graph::NodeIndex,
    ) -> impl Iterator<Item = SPQRNodeIndex<Graph::IndexType>> {
        self.node_data[node_index].spqr_node_indices.iter().copied()
    }

    pub fn cut_node_index_to_node_index(
        &self,
        cut_node_index: CutNodeIndex<Graph::IndexType>,
    ) -> Graph::NodeIndex {
        self.cut_nodes[cut_node_index].node
    }

    pub fn cut_node(
        &self,
        cut_node_index: CutNodeIndex<Graph::IndexType>,
    ) -> &CutNode<Graph::NodeIndex, Graph::IndexType> {
        &self.cut_nodes[cut_node_index]
    }

    /// Returns true if the given node is a cut node.
    pub fn is_cut_node(&self, node_index: Graph::NodeIndex) -> bool {
        self.node_data[node_index].cut_node_index.is_some()
    }

    pub fn spqr_edge(
        &self,
        spqr_edge_index: SPQREdgeIndex<Graph::IndexType>,
    ) -> &SPQREdge<Graph::NodeIndex, Graph::IndexType> {
        &self.spqr_edges[spqr_edge_index]
    }

    /// Returns true if the given node has an incident virtual edge.
    ///
    /// This also means that the node is part of at least two SPQR nodes.
    pub fn has_incident_virtual_edge(&self, node_index: Graph::NodeIndex) -> bool {
        self.node_data[node_index].spqr_node_indices.len() >= 2
    }
}

impl<NodeIndex: Copy, IndexType: Copy> Component<NodeIndex, IndexType> {
    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeIndex> {
        self.nodes.iter().copied()
    }

    pub fn iter_cut_nodes(&self) -> impl Iterator<Item = CutNodeIndex<IndexType>> {
        self.cut_nodes.iter().copied()
    }
}

impl<NodeIndex: Copy, IndexType: Copy> Block<NodeIndex, IndexType> {
    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeIndex> {
        self.nodes.iter().copied()
    }

    pub fn iter_cut_nodes(&self) -> impl Iterator<Item = CutNodeIndex<IndexType>> {
        self.cut_nodes.iter().copied()
    }
}

impl<NodeIndex: Copy, IndexType: Copy> CutNode<NodeIndex, IndexType> {
    pub fn component(&self) -> ComponentIndex<IndexType> {
        self.component
    }

    pub fn node(&self) -> NodeIndex {
        self.node
    }

    pub fn iter_adjacent_blocks(&self) -> impl Iterator<Item = BlockIndex<IndexType>> {
        self.adjacent_blocks.iter().copied()
    }
}

impl<NodeIndex: Copy, EdgeIndex: Copy, IndexType: Copy> SPQRNode<NodeIndex, EdgeIndex, IndexType> {
    pub fn block(&self) -> BlockIndex<IndexType> {
        self.block
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeIndex> {
        self.nodes.iter().copied()
    }

    pub fn p_node_poles(&self) -> Option<(NodeIndex, NodeIndex)> {
        if self.spqr_node_type == SPQRNodeType::PNode {
            Some((self.nodes[0], self.nodes[1]))
        } else {
            None
        }
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = EdgeIndex> {
        self.edges.iter().copied()
    }

    pub fn spqr_node_type(&self) -> SPQRNodeType {
        self.spqr_node_type
    }

    pub fn iter_incident_spqr_edges(&self) -> impl Iterator<Item = SPQREdgeIndex<IndexType>> {
        self.spqr_edges.iter().copied()
    }
}

impl<NodeIndex: GraphIndex, IndexType: GraphIndexInteger> SPQREdge<NodeIndex, IndexType> {
    pub fn endpoints(&self) -> (SPQRNodeIndex<IndexType>, SPQRNodeIndex<IndexType>) {
        self.endpoints
    }

    /// Returns the endpoint with the smaller index.
    pub fn min_endpoint(&self) -> SPQRNodeIndex<IndexType> {
        self.endpoints.0.min(self.endpoints.1)
    }

    /// Returns the endpoint with the larger index.
    pub fn max_endpoint(&self) -> SPQRNodeIndex<IndexType> {
        self.endpoints.0.max(self.endpoints.1)
    }

    pub fn virtual_edge(&self) -> (NodeIndex, NodeIndex) {
        self.virtual_edge
    }

    pub fn ordered_virtual_edge(&self) -> (NodeIndex, NodeIndex) {
        let (u, v) = self.virtual_edge();
        if u < v { (u, v) } else { (v, u) }
    }
}
