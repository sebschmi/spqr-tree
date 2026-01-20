use std::iter;

use num_traits::bounds::UpperBounded;
use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::decomposition::{
    Block, Component, SPQRDecompositionEdgeData, SPQRDecompositionNodeData, SPQREdge, SPQRNode,
    SPQRNodeType,
    graph::StaticGraph,
    indices::{BlockIndex, ComponentIndex, SPQREdgeIndex, SPQRNodeIndex},
};

pub struct SPQRDecompositionBuilder<'graph, Graph: StaticGraph> {
    graph: &'graph Graph,
    components: TaggedVec<ComponentIndex<Graph>, Component<Graph>>,
    blocks: TaggedVec<BlockIndex<Graph>, Block<Graph>>,
    spqr_nodes: TaggedVec<SPQRNodeIndex<Graph>, SPQRNode<Graph>>,
    spqr_edges: TaggedVec<SPQREdgeIndex<Graph>, SPQREdge<Graph>>,
    node_data: TaggedVec<Graph::NodeIndex, SPQRDecompositionNodeData<Graph>>,
    edge_data: TaggedVec<Graph::EdgeIndex, SPQRDecompositionEdgeData<Graph>>,
}

impl<'graph, Graph: StaticGraph> SPQRDecompositionBuilder<'graph, Graph> {
    pub fn new(graph: &'graph Graph) -> Self {
        Self {
            graph,
            components: TaggedVec::new(),
            blocks: TaggedVec::new(),
            spqr_nodes: TaggedVec::new(),
            spqr_edges: TaggedVec::new(),
            node_data: iter::repeat_with(|| SPQRDecompositionNodeData {
                component_index: ComponentIndex::max_value(),
                block_indices: SmallVec::new(),
                spqr_node_indices: SmallVec::new(),
            })
            .take(graph.node_count())
            .collect(),
            edge_data: iter::repeat_with(|| SPQRDecompositionEdgeData {
                component_index: ComponentIndex::max_value(),
                block_index: BlockIndex::max_value(),
                spqr_node_index: SPQRNodeIndex::max_value(),
            })
            .take(graph.edge_count())
            .collect(),
        }
    }

    /// Adds a component to the decomposition.
    ///
    /// Edges are automatically assigned to the component based on their endpoints.
    pub fn add_component(&mut self, nodes: Vec<Graph::NodeIndex>) -> ComponentIndex<Graph> {
        self.components.push_in_place(|index| {
            for node in nodes.iter().copied() {
                assert_eq!(
                    self.node_data[node].component_index,
                    ComponentIndex::max_value(),
                );
                self.node_data[node].component_index = index;

                for edge in self.graph.incident_edges(node) {
                    if self.edge_data[edge].component_index != index {
                        assert_eq!(
                            self.edge_data[edge].component_index,
                            ComponentIndex::max_value(),
                        );
                        self.edge_data[edge].component_index = index;
                    }
                }
            }

            Component {
                nodes,
                blocks: Vec::new(),
                cut_nodes: Vec::new(),
            }
        })
    }

    /// Adds a block into a component.
    ///
    /// Edges are automatically assigned to the block if both of their endpoints are in the block.
    pub fn add_block(
        &mut self,
        component: ComponentIndex<Graph>,
        nodes: Vec<Graph::NodeIndex>,
    ) -> BlockIndex<Graph> {
        self.blocks.push_in_place(|index| {
            self.components[component].blocks.push(index);

            for node in nodes.iter().copied() {
                assert_eq!(self.node_data[node].component_index, component);
                assert!(!self.node_data[node].block_indices.contains(&index));
                self.node_data[node].block_indices.push(index);
            }

            for node in nodes.iter().copied() {
                for edge in self.graph.incident_edges(node) {
                    if self.edge_data[edge].block_index != index {
                        let (a, b) = self.graph.edge_endpoints(edge);

                        if self.node_data[a].block_indices.contains(&index)
                            && self.node_data[b].block_indices.contains(&index)
                        {
                            // Both endpoints are in the block, so the edge must be in the block.
                            assert_eq!(self.edge_data[edge].block_index, BlockIndex::max_value());
                            self.edge_data[edge].block_index = index;
                        }
                    }
                }
            }

            Block {
                component,
                nodes,
                spqr_nodes: Vec::new(),
                spqr_edges: Vec::new(),
            }
        })
    }

    /// Adds an SPQR node into a block.
    ///
    /// Edges are not added to the component and must be added separately.
    pub fn add_spqr_node(
        &mut self,
        block: BlockIndex<Graph>,
        nodes: Vec<Graph::NodeIndex>,
        spqr_node_type: SPQRNodeType,
    ) -> SPQRNodeIndex<Graph> {
        self.spqr_nodes.push_in_place(|index| {
            self.blocks[block].spqr_nodes.push(index);

            for node in nodes.iter().copied() {
                assert_eq!(
                    self.node_data[node].component_index,
                    self.blocks[block].component
                );
                assert!(self.node_data[node].block_indices.contains(&block));
                assert!(!self.node_data[node].spqr_node_indices.contains(&index));
                self.node_data[node].spqr_node_indices.push(index);
            }

            SPQRNode {
                block,
                nodes,
                edges: Vec::new(),
                spqr_node_type,
            }
        })
    }

    pub fn add_edge_to_spqr_node(
        &mut self,
        edge: Graph::EdgeIndex,
        spqr_node: SPQRNodeIndex<Graph>,
    ) {
        assert_eq!(
            self.edge_data[edge].spqr_node_index,
            SPQRNodeIndex::max_value()
        );

        let (a, b) = self.graph.edge_endpoints(edge);
        assert!(self.node_data[a].spqr_node_indices.contains(&spqr_node));
        assert!(self.node_data[b].spqr_node_indices.contains(&spqr_node));

        self.edge_data[edge].spqr_node_index = spqr_node;
        self.spqr_nodes[spqr_node].edges.push(edge);
    }

    pub fn add_spqr_edge(
        &mut self,
        block: BlockIndex<Graph>,
        endpoints: (SPQRNodeIndex<Graph>, SPQRNodeIndex<Graph>),
        virtual_edge: (Graph::NodeIndex, Graph::NodeIndex),
    ) -> SPQREdgeIndex<Graph> {
        self.spqr_edges.push_in_place(|index| {
            assert_eq!(self.spqr_nodes[endpoints.0].block, block);
            assert_eq!(self.spqr_nodes[endpoints.1].block, block);
            self.blocks[block].spqr_edges.push(index);

            assert!(
                self.node_data[virtual_edge.0]
                    .spqr_node_indices
                    .contains(&endpoints.0)
            );
            assert!(
                self.node_data[virtual_edge.0]
                    .spqr_node_indices
                    .contains(&endpoints.1)
            );
            assert!(
                self.node_data[virtual_edge.1]
                    .spqr_node_indices
                    .contains(&endpoints.0)
            );
            assert!(
                self.node_data[virtual_edge.1]
                    .spqr_node_indices
                    .contains(&endpoints.1)
            );

            SPQREdge {
                endpoints,
                virtual_edge,
            }
        })
    }
}
