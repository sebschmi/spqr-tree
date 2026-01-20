use num_traits::bounds::UpperBounded;

use crate::decomposition::graph::StaticGraph;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct BlockIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct SPQRNodeIndex<Graph: StaticGraph>(Graph::NodeIndex);

impl<Graph: StaticGraph> From<usize> for ComponentIndex<Graph> {
    fn from(value: usize) -> Self {
        ComponentIndex(Graph::NodeIndex::from(value))
    }
}

impl<Graph: StaticGraph> From<ComponentIndex<Graph>> for usize {
    fn from(value: ComponentIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> From<&'_ ComponentIndex<Graph>> for usize {
    fn from(value: &ComponentIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> UpperBounded for ComponentIndex<Graph> {
    fn max_value() -> Self {
        ComponentIndex(Graph::NodeIndex::max_value())
    }
}

impl<Graph: StaticGraph> From<usize> for BlockIndex<Graph> {
    fn from(value: usize) -> Self {
        BlockIndex(Graph::NodeIndex::from(value))
    }
}

impl<Graph: StaticGraph> From<BlockIndex<Graph>> for usize {
    fn from(value: BlockIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> From<&'_ BlockIndex<Graph>> for usize {
    fn from(value: &BlockIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> UpperBounded for BlockIndex<Graph> {
    fn max_value() -> Self {
        BlockIndex(Graph::NodeIndex::max_value())
    }
}

impl<Graph: StaticGraph> From<usize> for SPQRNodeIndex<Graph> {
    fn from(value: usize) -> Self {
        SPQRNodeIndex(Graph::NodeIndex::from(value))
    }
}

impl<Graph: StaticGraph> From<SPQRNodeIndex<Graph>> for usize {
    fn from(value: SPQRNodeIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> From<&'_ SPQRNodeIndex<Graph>> for usize {
    fn from(value: &SPQRNodeIndex<Graph>) -> Self {
        value.0.into()
    }
}

impl<Graph: StaticGraph> UpperBounded for SPQRNodeIndex<Graph> {
    fn max_value() -> Self {
        SPQRNodeIndex(Graph::NodeIndex::max_value())
    }
}
