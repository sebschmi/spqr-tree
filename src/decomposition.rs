use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::decomposition::{
    graph::StaticGraph,
    indices::{
        BlockIndex, ComponentIndex, CutNodeIndex, OptionalCutNodeIndex, SPQREdgeIndex,
        SPQRNodeIndex,
    },
};

pub mod builder;
pub mod graph;
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
    graph: &'graph Graph,
    components: TaggedVec<ComponentIndex<Graph>, Component<Graph>>,
    blocks: TaggedVec<BlockIndex<Graph>, Block<Graph>>,
    cut_nodes: TaggedVec<CutNodeIndex<Graph>, CutNode<Graph>>,
    spqr_nodes: TaggedVec<SPQRNodeIndex<Graph>, SPQRNode<Graph>>,
    spqr_edges: TaggedVec<SPQREdgeIndex<Graph>, SPQREdge<Graph>>,
    node_data: TaggedVec<Graph::NodeIndex, SPQRDecompositionNodeData<Graph>>,
    edge_data: TaggedVec<Graph::EdgeIndex, SPQRDecompositionEdgeData<Graph>>,
}

pub struct Component<Graph: StaticGraph> {
    nodes: Vec<Graph::NodeIndex>,
    blocks: Vec<BlockIndex<Graph>>,
    cut_nodes: Vec<CutNodeIndex<Graph>>,
}

pub struct Block<Graph: StaticGraph> {
    component: ComponentIndex<Graph>,
    nodes: Vec<Graph::NodeIndex>,
    spqr_nodes: Vec<SPQRNodeIndex<Graph>>,
    spqr_edges: Vec<SPQREdgeIndex<Graph>>,
}

pub struct CutNode<Graph: StaticGraph> {
    component: ComponentIndex<Graph>,
    node: Graph::NodeIndex,
    adjacent_blocks: SmallVec<[BlockIndex<Graph>; 2]>,
}

pub struct SPQRNode<Graph: StaticGraph> {
    block: BlockIndex<Graph>,
    nodes: Vec<Graph::NodeIndex>,
    edges: Vec<Graph::EdgeIndex>,
    spqr_node_type: SPQRNodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SPQRNodeType {
    SNode,
    PNode,
    RNode,
}

/// An edge in the SPQR tree connecting two SPQR nodes.
pub struct SPQREdge<Graph: StaticGraph> {
    endpoints: (SPQRNodeIndex<Graph>, SPQRNodeIndex<Graph>),
    virtual_edge: (Graph::NodeIndex, Graph::NodeIndex),
}

pub struct SPQRDecompositionNodeData<Graph: StaticGraph> {
    component_index: ComponentIndex<Graph>,
    block_indices: SmallVec<[BlockIndex<Graph>; 1]>,
    cut_node_index: OptionalCutNodeIndex<Graph>,
    spqr_node_indices: SmallVec<[SPQRNodeIndex<Graph>; 1]>,
    extra_data: String,
}

pub struct SPQRDecompositionEdgeData<Graph: StaticGraph> {
    component_index: ComponentIndex<Graph>,
    block_index: BlockIndex<Graph>,
    spqr_node_index: SPQRNodeIndex<Graph>,
    extra_data: String,
}

impl<'graph, Graph: StaticGraph> SPQRDecomposition<'graph, Graph> {
    pub fn graph(&self) -> &'graph Graph {
        self.graph
    }

    pub fn iter_component_indices(&self) -> impl Iterator<Item = ComponentIndex<Graph>> {
        self.components.iter_indices()
    }

    pub fn iter_components(
        &self,
    ) -> impl Iterator<Item = (ComponentIndex<Graph>, &Component<Graph>)> {
        self.components.iter()
    }

    pub fn iter_blocks_in_component(
        &self,
        component_index: ComponentIndex<Graph>,
    ) -> impl Iterator<Item = (BlockIndex<Graph>, &Block<Graph>)> {
        self.components[component_index]
            .blocks
            .iter()
            .copied()
            .map(move |block_index| (block_index, &self.blocks[block_index]))
    }

    pub fn iter_spqr_nodes_in_block(
        &self,
        block_index: BlockIndex<Graph>,
    ) -> impl Iterator<Item = (SPQRNodeIndex<Graph>, &SPQRNode<Graph>)> {
        self.blocks[block_index]
            .spqr_nodes
            .iter()
            .copied()
            .map(move |spqr_node_index| (spqr_node_index, &self.spqr_nodes[spqr_node_index]))
    }

    pub fn iter_spqr_edges_in_block(
        &self,
        block_index: BlockIndex<Graph>,
    ) -> impl Iterator<Item = (SPQREdgeIndex<Graph>, &SPQREdge<Graph>)> {
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

    pub fn spqr_node_name(&self, spqr_node_index: SPQRNodeIndex<Graph>) -> String {
        match self.spqr_nodes[spqr_node_index].spqr_node_type() {
            SPQRNodeType::SNode => format!("S{spqr_node_index}"),
            SPQRNodeType::PNode => format!("P{spqr_node_index}"),
            SPQRNodeType::RNode => format!("R{spqr_node_index}"),
        }
    }

    pub fn cut_node_index_to_node_index(
        &self,
        cut_node_index: CutNodeIndex<Graph>,
    ) -> Graph::NodeIndex {
        self.cut_nodes[cut_node_index].node
    }

    pub fn cut_node(&self, cut_node_index: CutNodeIndex<Graph>) -> &CutNode<Graph> {
        &self.cut_nodes[cut_node_index]
    }
}

impl<Graph: StaticGraph> Component<Graph> {
    pub fn iter_nodes(&self) -> impl Iterator<Item = Graph::NodeIndex> {
        self.nodes.iter().copied()
    }

    pub fn iter_cut_nodes(&self) -> impl Iterator<Item = CutNodeIndex<Graph>> {
        self.cut_nodes.iter().copied()
    }
}

impl<Graph: StaticGraph> Block<Graph> {
    pub fn iter_nodes(&self) -> impl Iterator<Item = Graph::NodeIndex> {
        self.nodes.iter().copied()
    }
}

impl<Graph: StaticGraph> CutNode<Graph> {
    pub fn component(&self) -> ComponentIndex<Graph> {
        self.component
    }

    pub fn node(&self) -> Graph::NodeIndex {
        self.node
    }

    pub fn iter_adjacent_blocks(&self) -> impl Iterator<Item = BlockIndex<Graph>> {
        self.adjacent_blocks.iter().copied()
    }
}

impl<Graph: StaticGraph> SPQRNode<Graph> {
    pub fn block(&self) -> BlockIndex<Graph> {
        self.block
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = Graph::NodeIndex> {
        self.nodes.iter().copied()
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = Graph::EdgeIndex> {
        self.edges.iter().copied()
    }

    pub fn spqr_node_type(&self) -> SPQRNodeType {
        self.spqr_node_type
    }
}

impl<Graph: StaticGraph> SPQREdge<Graph> {
    pub fn endpoints(&self) -> (SPQRNodeIndex<Graph>, SPQRNodeIndex<Graph>) {
        self.endpoints
    }

    pub fn virtual_edge(&self) -> (Graph::NodeIndex, Graph::NodeIndex) {
        self.virtual_edge
    }
}
