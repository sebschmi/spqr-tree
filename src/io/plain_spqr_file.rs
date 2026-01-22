//! Methods for reading and writing an SPQR decomposition in the [`.spqr` file format](https://github.com/sebschmi/SPQR-tree-file-format).

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
};

use crate::{
    decomposition::{
        SPQRDecomposition, SPQRNodeType, builder::SPQRDecompositionBuilder, graph::StaticGraph,
    },
    io::plain_spqr_file::{error::ReadError, read_utils::read_next_line},
};

pub mod error;
mod read_utils;

pub fn read<'graph, Graph: StaticGraph>(
    graph: &'graph Graph,
    reader: &mut impl Read,
) -> Result<SPQRDecomposition<'graph, Graph>, ReadError> {
    let mut reader = BufReader::new(reader);

    // Parse header.
    let header = read_next_line(&mut reader)?.ok_or_else(|| ReadError::MissingHeader)?;
    if header[0].as_str() != "H" {
        return Err(ReadError::MissingHeader);
    }
    if header.get(1).map(String::as_str) != Some("v0.1") {
        return Err(ReadError::UnsupportedVersion);
    }
    if header.get(2).is_none() {
        return Err(ReadError::MissingHeaderUrl);
    }

    let mut builder = SPQRDecompositionBuilder::new(graph);
    let mut name_to_component_index = HashMap::new();
    //let mut name_to_block_index = HashMap::new();
    //let mut name_to_spqr_node_index = HashMap::new();
    //let mut name_to_spqr_edge_index = HashMap::new();

    while let Some(line) = read_next_line(&mut reader)? {
        match line[0].as_str() {
            "G" => {
                let component_name = line.get(1).ok_or_else(|| {
                    ReadError::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "missing component name",
                    ))
                })?;
                let nodes = line
                    .iter()
                    .skip(2)
                    .map(|node| graph.node_index_from_name(node))
                    .collect();

                let component_index = builder.add_component(nodes);
                name_to_component_index.insert(component_name.clone(), component_index);
            }
            "N" => {
                let node_name = line.get(1).ok_or_else(|| {
                    ReadError::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "missing node name",
                    ))
                })?;
                let extra_data = line[2..].join(" ");
                let node_index = graph.node_index_from_name(node_name);
                builder.add_extra_data_to_node(node_index, extra_data);
            }
            other => {
                return Err(ReadError::InvalidLineType(other.to_string()));
            }
        }
    }

    Ok(builder.build())
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
            let node_name = decomposition.graph().node_name(node_index);
            writeln!(writer, "N {node_name} {extra_data}")?;
        }
    }

    // Write components.
    for (component_index, component) in decomposition.iter_components() {
        write!(writer, "G G{component_index}")?;

        for node_index in component.iter_nodes() {
            let node_name = decomposition.graph().node_name(node_index);
            write!(writer, " {node_name}")?;
        }

        writeln!(writer)?;

        // Write cut nodes in component.
        for cut_node_index in component.iter_cut_nodes() {
            let cut_node = decomposition.cut_node(cut_node_index);
            let node_name = decomposition.graph().node_name(cut_node.node());
            writeln!(writer, "C {node_name}")?;

            for block_index in cut_node.iter_adjacent_blocks() {
                write!(writer, " B{block_index}")?;
            }
            writeln!(writer)?;
        }

        // Write blocks in component.
        for (block_index, block) in decomposition.iter_blocks_in_component(component_index) {
            write!(writer, "B B{block_index} G{component_index}")?;

            for node_index in block.iter_nodes() {
                let node_name = decomposition.graph().node_name(node_index);
                write!(writer, " {node_name}")?;
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
                    let node_name = decomposition.graph().node_name(node_index);
                    write!(writer, " {node_name}")?;
                }
                writeln!(writer)?;

                // Write edges (Q-nodes).
                for edge_index in spqr_node.iter_edges() {
                    let (u, v) = decomposition.graph().edge_endpoints(edge_index);
                    let node_name_u = decomposition.graph().node_name(u);
                    let node_name_v = decomposition.graph().node_name(v);
                    let edge_name = decomposition.graph().edge_name(edge_index);

                    write!(
                        writer,
                        "E {edge_name} {spqr_node_name} B{block_index} {node_name_u} {node_name_v}",
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
                let node_name_u = decomposition.graph().node_name(spqr_edge.virtual_edge().0);
                let node_name_v = decomposition.graph().node_name(spqr_edge.virtual_edge().1);

                writeln!(
                    writer,
                    "V V{spqr_edge_index} {spqr_node_name_u} {spqr_node_name_v} {node_name_u} {node_name_v}",
                )?;
            }
        }
    }

    Ok(())
}
