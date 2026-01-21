//! Methods for reading and writing an SPQR decomposition in the [`.spqr` file format](https://github.com/sebschmi/SPQR-tree-file-format).

use std::{
    io::{Read, Write},
};

use crate::decomposition::{SPQRDecomposition, graph::StaticGraph};

pub fn read<'graph, Graph: StaticGraph>(
    graph: &'graph Graph,
    reader: &mut impl Read,
) -> std::io::Result<SPQRDecomposition<'graph, Graph>> {
    todo!()
}

pub fn write<Graph: StaticGraph>(
    decomposition: &SPQRDecomposition<Graph>,
    writer: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(
        writer,
        "H v0.1 https://github.com/sebschmi/SPQR-tree-file-format"
    )?;

    // Write node extra data.
    for node_index in decomposition.iter_nodes() {
        let extra_data = decomposition.node_extra_data(node_index);
        if !extra_data.is_empty() {
            writeln!(writer, "N N{node_index} {extra_data}")?;
        }
    }

    // Write components.
    for (component_index, component) in decomposition.iter_components() {
        write!(writer, "G G{component_index}")?;

        for node_index in component.iter_nodes() {
            write!(writer, " N{node_index}")?;
        }

        writeln!(writer)?;

        // Write cut nodes in component.
        for cut_node_index in component.iter_cut_nodes() {
            writeln!(writer, "C N{cut_node_index}")?;

            todo!("Write block indices");
        }

        // Write blocks in component.
        for (block_index, block) in decomposition.iter_blocks_in_component(component_index) {
            write!(writer, "B B{block_index} G{component_index}")?;

            for node_index in block.iter_nodes() {
                write!(writer, " N{node_index}")?;
            }

            writeln!(writer)?;
        }
    }

    todo!()
}
