//! Methods for reading and writing an SPQR decomposition in the [`.spqr` file format](https://github.com/sebschmi/SPQR-tree-file-format).

use std::io::{Read, Write};

use crate::decomposition::{SPQRDecomposition, SPQRNodeType, graph::StaticGraph};

pub fn read<'graph, Graph: StaticGraph>(
    _graph: &'graph Graph,
    _reader: &mut impl Read,
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
            let cut_node = decomposition.cut_node(cut_node_index);
            writeln!(writer, "C N{}", cut_node.node())?;

            for block_index in cut_node.iter_adjacent_blocks() {
                write!(writer, " B{block_index}")?;
            }
            writeln!(writer)?;
        }

        // Write blocks in component.
        for (block_index, block) in decomposition.iter_blocks_in_component(component_index) {
            write!(writer, "B B{block_index} G{component_index}")?;

            for node_index in block.iter_nodes() {
                write!(writer, " N{node_index}")?;
            }
            writeln!(writer)?;

            // Write contained SPQR nodes.
            for (spqr_node_index, spqr_node) in decomposition.iter_spqr_nodes_in_block(block_index)
            {
                let spqr_node_name = match spqr_node.spqr_node_type() {
                    SPQRNodeType::SNode => format!("S{spqr_node_index}"),
                    SPQRNodeType::PNode => format!("P{spqr_node_index}"),
                    SPQRNodeType::RNode => format!("R{spqr_node_index}"),
                };
                match spqr_node.spqr_node_type() {
                    SPQRNodeType::SNode => write!(writer, "S {spqr_node_name} B{block_index}")?,
                    SPQRNodeType::PNode => write!(writer, "P {spqr_node_name} B{block_index}")?,
                    SPQRNodeType::RNode => write!(writer, "R {spqr_node_name} B{block_index}")?,
                }

                for node_index in spqr_node.iter_nodes() {
                    write!(writer, " N{node_index}")?;
                }
                writeln!(writer)?;

                // Write edges (Q nodes).
                for edge_index in spqr_node.iter_edges() {
                    let (u, v) = decomposition.graph().edge_endpoints(edge_index);
                    write!(
                        writer,
                        "E E{edge_index} {spqr_node_name} B{block_index} N{u} N{v}",
                    )?;

                    let extra_data = decomposition.edge_extra_data(edge_index);
                    if !extra_data.is_empty() {
                        write!(writer, " {extra_data}")?;
                    }
                    writeln!(writer)?;
                }
            }

            // Write contained SPQR edges.
            for (spqr_edge_index, spqr_edge) in decomposition.iter_spqr_edges_in_block(block_index)
            {
                let (spqr_node_index_u, spqr_node_index_v) = spqr_edge.endpoints();
                let spqr_node_name_u = decomposition.spqr_node_name(spqr_node_index_u);
                let spqr_node_name_v = decomposition.spqr_node_name(spqr_node_index_v);

                writeln!(
                    writer,
                    "V V{spqr_edge_index} {spqr_node_name_u} {spqr_node_name_v} N{} N{}",
                    spqr_edge.virtual_edge().0,
                    spqr_edge.virtual_edge().1,
                )?;
            }
        }
    }

    Ok(())
}
