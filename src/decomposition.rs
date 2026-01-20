use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::decomposition::{
    graph::StaticGraph,
    indices::{BlockIndex, ComponentIndex, SPQRNodeIndex},
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
    triconnected_components: TaggedVec<SPQRNodeIndex<Graph>, SPQRNode<Graph>>,
    node_data: Vec<SPQRDecompositionNodeData<Graph>>,
    edge_data: Vec<SPQRDecompositionEdgeData<Graph>>,
}

pub struct Component<Graph: StaticGraph> {
    nodes: Vec<Graph::NodeIndex>,
    blocks: Vec<BlockIndex<Graph>>,
}

pub struct Block<Graph: StaticGraph> {
    component: ComponentIndex<Graph>,
    nodes: Vec<Graph::NodeIndex>,
    triconnected_components: Vec<SPQRNodeIndex<Graph>>,
}

pub struct SPQRNode<Graph: StaticGraph> {
    block: BlockIndex<Graph>,
    nodes: Vec<Graph::NodeIndex>,
    spqr_node_type: SPQRNodeType,
}

pub enum SPQRNodeType {
    SNode,
    PNode,
    RNode,
}

pub struct SPQRDecompositionNodeData<Graph: StaticGraph> {
    component_index: ComponentIndex<Graph>,
    block_indices: SmallVec<[BlockIndex<Graph>; 1]>,
    triconnected_component_indices: SmallVec<[SPQRNodeIndex<Graph>; 1]>,
}

pub struct SPQRDecompositionEdgeData<Graph: StaticGraph> {
    component_index: ComponentIndex<Graph>,
    block_index: BlockIndex<Graph>,
    triconnected_component_index: SPQRNodeIndex<Graph>,
}

impl<'graph, Graph: StaticGraph> SPQRDecomposition<'graph, Graph> {}
