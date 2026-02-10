use std::iter;

use smallvec::SmallVec;
use tagged_vec::TaggedVec;

use crate::{
    decomposition::{
        Block, Component, CutNode, SPQRDecomposition, SPQRDecompositionEdgeData,
        SPQRDecompositionNodeData, SPQREdge, SPQRNode, SPQRNodeType,
        indices::{
            BlockIndex, ComponentIndex, CutNodeIndex, OptionalBlockIndex, OptionalComponentIndex,
            OptionalCutNodeIndex, OptionalSPQRNodeIndex, SPQREdgeIndex, SPQRNodeIndex,
        },
    },
    graph::StaticGraph,
};

pub struct SPQRDecompositionBuilder<'graph, Graph: StaticGraph> {
    graph: &'graph Graph,
    components: TaggedVec<ComponentIndex<Graph::IndexType>, Component<Graph>>,
    blocks: TaggedVec<BlockIndex<Graph::IndexType>, Block<Graph>>,
    cut_nodes: TaggedVec<CutNodeIndex<Graph::IndexType>, CutNode<Graph>>,
    spqr_nodes: TaggedVec<SPQRNodeIndex<Graph::IndexType>, SPQRNode<Graph>>,
    spqr_edges: TaggedVec<SPQREdgeIndex<Graph::IndexType>, SPQREdge<Graph>>,
    node_data: TaggedVec<Graph::NodeIndex, SPQRDecompositionNodeDataBuilder<Graph>>,
    edge_data: TaggedVec<Graph::EdgeIndex, SPQRDecompositionEdgeDataBuilder<Graph>>,
}

struct SPQRDecompositionNodeDataBuilder<Graph: StaticGraph> {
    component_index: OptionalComponentIndex<Graph::IndexType>,
    block_indices: SmallVec<[BlockIndex<Graph::IndexType>; 1]>,
    cut_node_index: OptionalCutNodeIndex<Graph::IndexType>,
    spqr_node_indices: SmallVec<[SPQRNodeIndex<Graph::IndexType>; 1]>,
    extra_data: String,
}

struct SPQRDecompositionEdgeDataBuilder<Graph: StaticGraph> {
    component_index: OptionalComponentIndex<Graph::IndexType>,
    block_index: OptionalBlockIndex<Graph::IndexType>,
    spqr_node_index: OptionalSPQRNodeIndex<Graph::IndexType>,
    extra_data: String,
}

impl<'graph, Graph: StaticGraph> SPQRDecompositionBuilder<'graph, Graph> {
    pub fn new(graph: &'graph Graph) -> Self {
        Self {
            graph,
            components: TaggedVec::new(),
            blocks: TaggedVec::new(),
            cut_nodes: TaggedVec::new(),
            spqr_nodes: TaggedVec::new(),
            spqr_edges: TaggedVec::new(),
            node_data: iter::repeat_with(|| SPQRDecompositionNodeDataBuilder {
                component_index: OptionalComponentIndex::new_none(),
                block_indices: SmallVec::new(),
                cut_node_index: OptionalCutNodeIndex::new_none(),
                spqr_node_indices: SmallVec::new(),
                extra_data: String::new(),
            })
            .take(graph.node_count())
            .collect(),
            edge_data: iter::repeat_with(|| SPQRDecompositionEdgeDataBuilder {
                component_index: OptionalComponentIndex::new_none(),
                block_index: OptionalBlockIndex::new_none(),
                spqr_node_index: OptionalSPQRNodeIndex::new_none(),
                extra_data: String::new(),
            })
            .take(graph.edge_count())
            .collect(),
        }
    }

    /// Adds a component to the decomposition.
    ///
    /// Edges are automatically assigned to the component based on their endpoints.
    pub fn add_component(
        &mut self,
        nodes: Vec<Graph::NodeIndex>,
    ) -> ComponentIndex<Graph::IndexType> {
        assert!(!nodes.is_empty());

        self.components.push_in_place(|index| {
            for node in nodes.iter().copied() {
                assert!(self.node_data[node].component_index.is_none());
                self.node_data[node].component_index = index.into();

                for edge in self.graph.incident_edges(node) {
                    if self.edge_data[edge].component_index != Some(index).into() {
                        assert!(self.edge_data[edge].component_index.is_none());
                        self.edge_data[edge].component_index = index.into();
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

    /// Adds extra data to the given node.
    pub fn add_extra_data_to_node(&mut self, node: Graph::NodeIndex, extra_data: String) {
        assert!(self.node_data[node].extra_data.is_empty());
        self.node_data[node].extra_data = extra_data;
    }

    /// Adds a block into a component.
    ///
    /// Edges are automatically assigned to the block if both of their endpoints are in the block.
    /// Cut nodes are identified once [`build`](Self::build) is called.
    pub fn add_block(
        &mut self,
        component: ComponentIndex<Graph::IndexType>,
        nodes: Vec<Graph::NodeIndex>,
    ) -> BlockIndex<Graph::IndexType> {
        assert!(!nodes.is_empty());

        self.blocks.push_in_place(|index| {
            self.components[component].blocks.push(index);

            for node in nodes.iter().copied() {
                assert_eq!(self.node_data[node].component_index, Some(component).into());
                assert!(!self.node_data[node].block_indices.contains(&index));
                self.node_data[node].block_indices.push(index);
            }

            for node in nodes.iter().copied() {
                for edge in self.graph.incident_edges(node) {
                    if self.edge_data[edge].block_index != Some(index).into() {
                        let (a, b) = self.graph.edge_endpoints(edge);

                        if self.node_data[a].block_indices.contains(&index)
                            && self.node_data[b].block_indices.contains(&index)
                        {
                            // Both endpoints are in the block, so the edge must be in the block.
                            assert!(self.edge_data[edge].block_index.is_none());
                            self.edge_data[edge].block_index = index.into();
                        }
                    }
                }
            }

            Block {
                component,
                nodes,
                cut_nodes: Vec::new(),
                spqr_nodes: Vec::new(),
                spqr_edges: Vec::new(),
            }
        })
    }

    /// Manually add a cut node to the decomposition.
    pub fn add_cut_node(
        &mut self,
        cut_node: Graph::NodeIndex,
        blocks: Vec<BlockIndex<Graph::IndexType>>,
    ) -> CutNodeIndex<Graph::IndexType> {
        assert!(!blocks.is_empty());

        self.cut_nodes.push_in_place(|cut_node_index| {
            assert!(self.node_data[cut_node].cut_node_index.is_none());
            self.node_data[cut_node].cut_node_index = Some(cut_node_index).into();

            let component_index = self.node_data[cut_node].component_index.unwrap();

            self.components[component_index]
                .cut_nodes
                .push(cut_node_index);

            for block_index in blocks.iter().copied() {
                debug_assert!(!self.blocks[block_index].cut_nodes.contains(&cut_node_index));
                self.blocks[block_index].cut_nodes.push(cut_node_index);
            }

            CutNode {
                component: component_index,
                node: cut_node,
                adjacent_blocks: blocks.into(),
            }
        })
    }

    /// Adds an SPQR node into a block.
    ///
    /// Edges are not added to the component and must be added separately.
    pub fn add_spqr_node(
        &mut self,
        block: BlockIndex<Graph::IndexType>,
        nodes: Vec<Graph::NodeIndex>,
        spqr_node_type: SPQRNodeType,
    ) -> SPQRNodeIndex<Graph::IndexType> {
        assert!(nodes.len() >= 2);

        self.spqr_nodes.push_in_place(|index| {
            self.blocks[block].spqr_nodes.push(index);

            for node in nodes.iter().copied() {
                assert_eq!(
                    self.node_data[node].component_index,
                    self.blocks[block].component.into()
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
                spqr_edges: SmallVec::new(),
            }
        })
    }

    /// Adds an edge into an SPQR node.
    ///
    /// These edges are Q-nodes in some interpretations of the SPQR tree.
    pub fn add_edge_to_spqr_node(
        &mut self,
        edge: Graph::EdgeIndex,
        spqr_node: SPQRNodeIndex<Graph::IndexType>,
    ) {
        assert!(self.edge_data[edge].spqr_node_index.is_none());

        let (a, b) = self.graph.edge_endpoints(edge);
        assert!(self.node_data[a].spqr_node_indices.contains(&spqr_node));
        assert!(self.node_data[b].spqr_node_indices.contains(&spqr_node));

        self.edge_data[edge].spqr_node_index = spqr_node.into();
        self.spqr_nodes[spqr_node].edges.push(edge);
    }

    /// Adds an edge to the SPQR tree.
    ///
    /// These edges connect two SPQR nodes `endpoints` and correspond to the virtual edge `virtual_edge` in the two SPQR nodes.
    ///
    /// If the block index is `None`, it is inferred from the SPQR nodes.
    pub fn add_spqr_edge(
        &mut self,
        block: OptionalBlockIndex<Graph::IndexType>,
        endpoints: (
            SPQRNodeIndex<Graph::IndexType>,
            SPQRNodeIndex<Graph::IndexType>,
        ),
        virtual_edge: (Graph::NodeIndex, Graph::NodeIndex),
    ) -> SPQREdgeIndex<Graph::IndexType> {
        let block = block.unwrap_or_else(|| {
            let block_u = self.spqr_nodes[endpoints.0].block;
            let block_v = self.spqr_nodes[endpoints.1].block;
            assert_eq!(block_u, block_v);
            block_u
        });

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

            assert!(!self.spqr_nodes[endpoints.0].spqr_edges.contains(&index));
            assert!(!self.spqr_nodes[endpoints.1].spqr_edges.contains(&index));
            self.spqr_nodes[endpoints.0].spqr_edges.push(index);
            self.spqr_nodes[endpoints.1].spqr_edges.push(index);

            SPQREdge {
                endpoints,
                virtual_edge,
            }
        })
    }

    /// Finalize the SPQR decomposition.
    ///
    /// This method performs some sanity checks and identifies remaining cut nodes.
    pub fn build(mut self) -> SPQRDecomposition<'graph, Graph> {
        // Ensure that all nodes have actually been assigned to components, blocks, and SPQR nodes.
        for node_index in self.graph.node_indices() {
            let SPQRDecompositionNodeDataBuilder {
                component_index,
                block_indices,
                spqr_node_indices,
                ..
            } = &self.node_data[node_index];

            debug_assert!(component_index.is_some());
            debug_assert!(!block_indices.is_empty());
            debug_assert!(!spqr_node_indices.is_empty());
        }

        // Ensure that all edges have actually been assigned to components, blocks, and SPQR nodes.
        for edge_index in self.graph.edge_indices() {
            let SPQRDecompositionEdgeDataBuilder {
                component_index,
                block_index,
                spqr_node_index,
                ..
            } = &self.edge_data[edge_index];

            debug_assert!(component_index.is_some());
            debug_assert!(block_index.is_some());
            debug_assert!(spqr_node_index.is_some());
        }

        // Identify cut nodes.
        for node_index in self.graph.node_indices() {
            if self.node_data[node_index].cut_node_index.is_some() {
                // Skip nodes that are already declared as cut nodes.
                continue;
            }

            let block_indices = &self.node_data[node_index].block_indices;

            // Nodes in multiple blocks are cut nodes.
            if block_indices.len() >= 2 {
                let block_indices: SmallVec<_> = block_indices.iter().copied().collect();
                let component_index = self.node_data[node_index].component_index.unwrap();

                self.cut_nodes.push_in_place(|cut_node_index| {
                    self.components[component_index]
                        .cut_nodes
                        .push(cut_node_index);

                    for block_index in block_indices.iter().copied() {
                        debug_assert!(
                            !self.blocks[block_index].cut_nodes.contains(&cut_node_index),
                        );
                        self.blocks[block_index].cut_nodes.push(cut_node_index);
                    }

                    assert!(self.node_data[node_index].cut_node_index.is_none());
                    self.node_data[node_index].cut_node_index = Some(cut_node_index).into();

                    CutNode {
                        component: component_index,
                        node: node_index,
                        adjacent_blocks: block_indices,
                    }
                });
            }
        }

        SPQRDecomposition {
            graph: self.graph,
            components: self.components,
            blocks: self.blocks,
            cut_nodes: self.cut_nodes,
            spqr_nodes: self.spqr_nodes,
            spqr_edges: self.spqr_edges,
            node_data: self
                .node_data
                .into_values_iter()
                .map(SPQRDecompositionNodeDataBuilder::build)
                .collect(),
            edge_data: self
                .edge_data
                .into_values_iter()
                .map(SPQRDecompositionEdgeDataBuilder::build)
                .collect(),
        }
    }

    /// Returns the block index of the given SPQR node.
    pub fn spqr_node_block_index(
        &self,
        spqr_node_index: SPQRNodeIndex<Graph::IndexType>,
    ) -> BlockIndex<Graph::IndexType> {
        self.spqr_nodes[spqr_node_index].block
    }
}

impl<Graph: StaticGraph> SPQRDecompositionNodeDataBuilder<Graph> {
    fn build(self) -> SPQRDecompositionNodeData<Graph> {
        SPQRDecompositionNodeData {
            component_index: self.component_index.unwrap(),
            block_indices: self.block_indices,
            cut_node_index: self.cut_node_index,
            spqr_node_indices: self.spqr_node_indices,
            extra_data: self.extra_data,
        }
    }
}

impl<Graph: StaticGraph> SPQRDecompositionEdgeDataBuilder<Graph> {
    fn build(self) -> SPQRDecompositionEdgeData<Graph> {
        SPQRDecompositionEdgeData {
            component_index: self.component_index.unwrap(),
            block_index: self.block_index.unwrap(),
            spqr_node_index: self.spqr_node_index.unwrap(),
            extra_data: self.extra_data,
        }
    }
}
