use num_traits::bounds::UpperBounded;

use crate::decomposition::graph::StaticGraph;

pub struct ComponentIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct BlockIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct CutNodeIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct OptionalCutNodeIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct SPQRNodeIndex<Graph: StaticGraph>(Graph::NodeIndex);

pub struct SPQREdgeIndex<Graph: StaticGraph>(Graph::NodeIndex);

macro_rules! impl_index_traits {
    ($name:ident) => {
        impl<Graph: StaticGraph> std::fmt::Debug for $name<Graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl<Graph: StaticGraph> std::fmt::Display for $name<Graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<Graph: StaticGraph> Clone for $name<Graph> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Graph: StaticGraph> Copy for $name<Graph> {}

        impl<Graph: StaticGraph> PartialEq for $name<Graph> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<Graph: StaticGraph> Eq for $name<Graph> {}

        impl<Graph: StaticGraph> std::hash::Hash for $name<Graph> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl<Graph: StaticGraph> PartialOrd for $name<Graph> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<Graph: StaticGraph> Ord for $name<Graph> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl<Graph: StaticGraph> From<usize> for $name<Graph> {
            fn from(value: usize) -> Self {
                $name(Graph::NodeIndex::from(value))
            }
        }

        impl<Graph: StaticGraph> From<$name<Graph>> for usize {
            fn from(value: $name<Graph>) -> Self {
                value.0.into()
            }
        }

        impl<Graph: StaticGraph> From<&'_ $name<Graph>> for usize {
            fn from(value: &$name<Graph>) -> Self {
                value.0.into()
            }
        }

        impl<Graph: StaticGraph> UpperBounded for $name<Graph> {
            fn max_value() -> Self {
                $name(Graph::NodeIndex::max_value())
            }
        }
    };
}

macro_rules! impl_optional_index_traits {
    ($name:ident, $some_name:ident) => {
        impl<Graph: StaticGraph> std::fmt::Debug for $name<Graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Option::<$some_name<Graph>>::from(*self).fmt(f)
            }
        }

        impl<Graph: StaticGraph> std::fmt::Display for $name<Graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0 == Graph::NodeIndex::max_value() {
                    write!(f, "None")
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }

        impl<Graph: StaticGraph> Clone for $name<Graph> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Graph: StaticGraph> Copy for $name<Graph> {}

        impl<Graph: StaticGraph> PartialEq for $name<Graph> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<Graph: StaticGraph> Eq for $name<Graph> {}

        impl<Graph: StaticGraph> std::hash::Hash for $name<Graph> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl<Graph: StaticGraph> PartialOrd for $name<Graph> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<Graph: StaticGraph> Ord for $name<Graph> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl<Graph: StaticGraph> From<Option<$some_name<Graph>>> for $name<Graph> {
            fn from(value: Option<$some_name<Graph>>) -> Self {
                if let Some(some_value) = value {
                    $name(some_value.0)
                } else {
                    $name(Graph::NodeIndex::max_value())
                }
            }
        }

        impl<Graph: StaticGraph> From<$name<Graph>> for Option<$some_name<Graph>> {
            fn from(value: $name<Graph>) -> Self {
                if value.0 == Graph::NodeIndex::max_value() {
                    None
                } else {
                    Some($some_name(value.0))
                }
            }
        }

        impl<Graph: StaticGraph> $name<Graph> {
            pub fn none() -> Self {
                $name(Graph::NodeIndex::max_value())
            }

            pub fn is_none(&self) -> bool {
                self.0 == Graph::NodeIndex::max_value()
            }
        }
    };
}

impl_index_traits!(ComponentIndex);
impl_index_traits!(BlockIndex);
impl_index_traits!(CutNodeIndex);
impl_optional_index_traits!(OptionalCutNodeIndex, CutNodeIndex);
impl_index_traits!(SPQRNodeIndex);
impl_index_traits!(SPQREdgeIndex);
