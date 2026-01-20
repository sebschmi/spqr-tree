use std::iter;

use num_traits::bounds::UpperBounded;
use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::decomposition::{
    Block, Component, SPQRDecompositionEdgeData, SPQRDecompositionNodeData, SPQRNode,
    graph::StaticGraph,
    indices::{BlockIndex, ComponentIndex, SPQRNodeIndex},
};

pub struct SPQRDecompositionBuilder<'graph, Graph: StaticGraph> {
    graph: &'graph Graph,
    components: TaggedVec<ComponentIndex<Graph>, Component<Graph>>,
    blocks: TaggedVec<BlockIndex<Graph>, Block<Graph>>,
    triconnected_components: TaggedVec<SPQRNodeIndex<Graph>, SPQRNode<Graph>>,
    node_data: Vec<SPQRDecompositionNodeData<Graph>>,
    edge_data: Vec<SPQRDecompositionEdgeData<Graph>>,
}

impl<'graph, Graph: StaticGraph> SPQRDecompositionBuilder<'graph, Graph> {
    pub fn new(graph: &'graph Graph) -> Self {
        Self {
            graph,
            components: TaggedVec::new(),
            blocks: TaggedVec::new(),
            triconnected_components: TaggedVec::new(),
            node_data: iter::repeat_with(|| SPQRDecompositionNodeData {
                component_index: ComponentIndex::max_value(),
                block_indices: SmallVec::new(),
                triconnected_component_indices: SmallVec::new(),
            })
            .take(graph.node_count())
            .collect(),
            edge_data: iter::repeat_with(|| SPQRDecompositionEdgeData {
                component_index: ComponentIndex::max_value(),
                block_index: BlockIndex::max_value(),
                triconnected_component_index: SPQRNodeIndex::max_value(),
            })
            .take(graph.edge_count())
            .collect(),
        }
    }
}
