use std::io::Read;

use smallvec::SmallVec;

use crate::{
    decomposition::{
        Block, Component, CutNode, SPQRDecomposition, SPQRDecompositionEdgeData,
        SPQRDecompositionNodeData, SPQREdge, SPQRNode, SPQRNodeType, indices::GraphIndexInteger,
    },
    graph::StaticGraph,
};

#[cfg(test)]
mod tests;

impl<'graph, Graph: StaticGraph> SPQRDecomposition<'graph, Graph> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(graph: &'graph Graph, mut reader: impl Read) -> std::io::Result<Self> {
        let component_amount = read_usize_binary(&mut reader)?;
        let components = (0..component_amount)
            .map(|_| Component::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let block_amount = read_usize_binary(&mut reader)?;
        let blocks = (0..block_amount)
            .map(|_| Block::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let cut_node_amount = read_usize_binary(&mut reader)?;
        let cut_nodes = (0..cut_node_amount)
            .map(|_| CutNode::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let spqr_node_amount = read_usize_binary(&mut reader)?;
        let spqr_nodes = (0..spqr_node_amount)
            .map(|_| SPQRNode::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let spqr_edge_amount = read_usize_binary(&mut reader)?;
        let spqr_edges = (0..spqr_edge_amount)
            .map(|_| SPQREdge::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let node_data_amount = read_usize_binary(&mut reader)?;
        let node_data = (0..node_data_amount)
            .map(|_| SPQRDecompositionNodeData::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        let edge_data_amount = read_usize_binary(&mut reader)?;
        let edge_data = (0..edge_data_amount)
            .map(|_| SPQRDecompositionEdgeData::read_binary(&mut reader))
            .collect::<std::io::Result<_>>()?;

        Ok(Self {
            graph,
            components,
            blocks,
            cut_nodes,
            spqr_nodes,
            spqr_edges,
            node_data,
            edge_data,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_usize_binary(self.components.len(), &mut writer)?;
        for component in self.components.iter_values() {
            component.write_binary(&mut writer)?;
        }

        write_usize_binary(self.blocks.len(), &mut writer)?;
        for block in self.blocks.iter_values() {
            block.write_binary(&mut writer)?;
        }

        write_usize_binary(self.cut_nodes.len(), &mut writer)?;
        for cut_node in self.cut_nodes.iter_values() {
            cut_node.write_binary(&mut writer)?;
        }

        write_usize_binary(self.spqr_nodes.len(), &mut writer)?;
        for spqr_node in self.spqr_nodes.iter_values() {
            spqr_node.write_binary(&mut writer)?;
        }

        write_usize_binary(self.spqr_edges.len(), &mut writer)?;
        for spqr_edge in self.spqr_edges.iter_values() {
            spqr_edge.write_binary(&mut writer)?;
        }

        write_usize_binary(self.node_data.len(), &mut writer)?;
        for node_data in self.node_data.iter_values() {
            node_data.write_binary(&mut writer)?;
        }

        write_usize_binary(self.edge_data.len(), &mut writer)?;
        for edge_data in self.edge_data.iter_values() {
            edge_data.write_binary(&mut writer)?;
        }

        Ok(())
    }
}

impl<NodeIndex: Copy, IndexType: Copy> Component<NodeIndex, IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            nodes: read_vec_binary(&mut reader)?,
            blocks: read_vec_binary(&mut reader)?,
            cut_nodes: read_vec_binary(&mut reader)?,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_slice_binary(&self.nodes, &mut writer)?;
        write_slice_binary(&self.blocks, &mut writer)?;
        write_slice_binary(&self.cut_nodes, &mut writer)?;
        Ok(())
    }
}

impl<NodeIndex: Copy, IndexType: Copy> Block<NodeIndex, IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            component: read_binary(&mut reader)?,
            nodes: read_vec_binary(&mut reader)?,
            cut_nodes: read_vec_binary(&mut reader)?,
            spqr_nodes: read_vec_binary(&mut reader)?,
            spqr_edges: read_vec_binary(&mut reader)?,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.component, &mut writer)?;
        write_slice_binary(&self.nodes, &mut writer)?;
        write_slice_binary(&self.cut_nodes, &mut writer)?;
        write_slice_binary(&self.spqr_nodes, &mut writer)?;
        write_slice_binary(&self.spqr_edges, &mut writer)?;
        Ok(())
    }
}

impl<NodeIndex: Copy, IndexType: Copy> CutNode<NodeIndex, IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            component: read_binary(&mut reader)?,
            node: read_binary(&mut reader)?,
            adjacent_blocks: read_vec_binary(&mut reader)?.into(),
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.component, &mut writer)?;
        write_binary(&self.node, &mut writer)?;
        write_slice_binary(&self.adjacent_blocks, &mut writer)?;
        Ok(())
    }
}

impl<NodeIndex: Copy, EdgeIndex: Copy, IndexType: Copy> SPQRNode<NodeIndex, EdgeIndex, IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            block: read_binary(&mut reader)?,
            nodes: read_vec_binary(&mut reader)?,
            edges: read_vec_binary(&mut reader)?,
            spqr_node_type: SPQRNodeType::read_binary(&mut reader)?,
            spqr_edges: read_vec_binary(&mut reader)?.into(),
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.block, &mut writer)?;
        write_slice_binary(&self.nodes, &mut writer)?;
        write_slice_binary(&self.edges, &mut writer)?;
        self.spqr_node_type.write_binary(&mut writer)?;
        write_slice_binary(&self.spqr_edges, &mut writer)?;
        Ok(())
    }
}

impl SPQRNodeType {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        match byte[0] {
            0 => Ok(Self::SNode),
            1 => Ok(Self::PNode),
            2 => Ok(Self::RNode),
            _ => panic!(),
        }
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        let byte = match self {
            Self::SNode => 0u8,
            Self::PNode => 1u8,
            Self::RNode => 2u8,
        };
        writer.write_all(&[byte])?;
        Ok(())
    }
}

impl<NodeIndex: Copy, IndexType: Copy> SPQREdge<NodeIndex, IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            endpoints: read_binary(&mut reader)?,
            virtual_edge: read_binary(&mut reader)?,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.endpoints, &mut writer)?;
        write_binary(&self.virtual_edge, &mut writer)?;
        Ok(())
    }
}

impl<IndexType: GraphIndexInteger> SPQRDecompositionNodeData<IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            component_index: read_binary(&mut reader)?,
            block_indices: read_vec_binary(&mut reader)?.into(),
            cut_node_index: read_binary(&mut reader)?,
            spqr_node_indices: read_vec_binary(&mut reader)?.into(),
            extra_data: read_string_binary(&mut reader)?,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.component_index, &mut writer)?;
        write_slice_binary(&self.block_indices, &mut writer)?;
        write_binary(&self.cut_node_index, &mut writer)?;
        write_slice_binary(&self.spqr_node_indices, &mut writer)?;
        write_str_binary(&self.extra_data, &mut writer)?;
        Ok(())
    }
}

impl<IndexType: Copy> SPQRDecompositionEdgeData<IndexType> {
    /// Reads a bidirected adjacency array from a platform-dependent binary format.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        Ok(Self {
            component_index: read_binary(&mut reader)?,
            block_index: read_binary(&mut reader)?,
            spqr_node_index: read_binary(&mut reader)?,
            extra_data: read_string_binary(&mut reader)?,
        })
    }

    /// Writes the bidirected adjacency array into a platform-dependent binary format.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        write_binary(&self.component_index, &mut writer)?;
        write_binary(&self.block_index, &mut writer)?;
        write_binary(&self.spqr_node_index, &mut writer)?;
        write_str_binary(&self.extra_data, &mut writer)?;
        Ok(())
    }
}

fn read_vec_binary<T: Copy>(mut reader: impl Read) -> std::io::Result<Vec<T>> {
    let mut len_bytes = [0u8; std::mem::size_of::<usize>()];
    reader.read_exact(&mut len_bytes)?;
    let len = usize::from_ne_bytes(len_bytes);

    let mut vec = Vec::with_capacity(len);
    unsafe {
        let vec_bytes = std::slice::from_raw_parts_mut(
            vec.as_mut_ptr() as *mut u8,
            len * std::mem::size_of::<T>(),
        );
        reader.read_exact(vec_bytes)?;
        vec.set_len(len);
    }

    Ok(vec)
}

fn write_slice_binary<T: Copy>(
    slice: &[T],
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    writer.write_all(&slice.len().to_ne_bytes())?;

    let slice: &[u8] = unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
    };
    writer.write_all(slice)?;

    Ok(())
}

fn read_string_binary(mut reader: impl Read) -> std::io::Result<String> {
    let bytes = read_vec_binary(&mut reader)?;
    Ok(String::from_utf8(bytes).unwrap())
}

fn write_str_binary(s: &str, mut writer: impl std::io::Write) -> std::io::Result<()> {
    write_slice_binary(s.as_bytes(), &mut writer)
}

fn read_usize_binary(mut reader: impl Read) -> std::io::Result<usize> {
    let mut bytes = [0u8; std::mem::size_of::<usize>()];
    reader.read_exact(&mut bytes)?;
    Ok(usize::from_ne_bytes(bytes))
}

fn write_usize_binary(value: usize, mut writer: impl std::io::Write) -> std::io::Result<()> {
    writer.write_all(&value.to_ne_bytes())?;
    Ok(())
}

fn read_binary<T: Copy>(mut reader: impl Read) -> std::io::Result<T> {
    let mut vec = SmallVec::<[T; 1]>::new();
    unsafe {
        vec.set_len(1);
        let vec_bytes =
            std::slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, std::mem::size_of::<T>());
        reader.read_exact(vec_bytes)?;
    }
    Ok(vec.into_iter().next().unwrap())
}

fn write_binary<T: Copy>(value: &T, mut writer: impl std::io::Write) -> std::io::Result<()> {
    let value_bytes = unsafe {
        std::slice::from_raw_parts(value as *const T as *const u8, std::mem::size_of::<T>())
    };
    writer.write_all(value_bytes)?;
    Ok(())
}
