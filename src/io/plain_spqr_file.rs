//! Methods for reading and writing an SPQR decomposition in the [`.spqr` file format](https://github.com/sebschmi/SPQR-tree-file-format).

use std::{
    io::{BufRead, Write},
    iter,
};

use log::{debug, trace};
use rustc_hash::FxHashMap;

use crate::{
    decomposition::{SPQRDecomposition, SPQRNodeType, builder::SPQRDecompositionBuilder},
    graph::StaticGraph,
    io::plain_spqr_file::{error::ReadError, line_reader::LineReader},
};

pub mod error;
mod line_reader;
#[cfg(test)]
pub mod tests;

impl<'graph, Graph: StaticGraph> SPQRDecomposition<'graph, Graph> {
    /// Read an SPQR decomposition in the plain SPQR file format.
    pub fn read_plain_spqr(graph: &'graph Graph, reader: impl BufRead) -> Result<Self, ReadError> {
        debug!("Reading SPQR decomposition in plain SPQR format...");
        let mut line_reader = LineReader::new(reader);

        // Parse header.
        trace!("Parsing header");
        let header = line_reader.next()?.ok_or(ReadError::MissingHeader)?;
        if &header[0] != "H" {
            return Err(ReadError::MissingHeader);
        }
        if header.column(1) != Some("v0.3") {
            return Err(ReadError::UnsupportedVersion);
        }
        if header.column(2).is_none() {
            return Err(ReadError::MissingHeaderUrl);
        }

        let mut builder = SPQRDecompositionBuilder::new(graph);
        let name_to_node_index: FxHashMap<_, _> = graph
            .node_indices()
            .map(|node_index| (graph.node_name(node_index).into_owned(), node_index))
            .collect();
        let mut name_to_component_index = FxHashMap::default();
        let mut name_to_block_index = FxHashMap::default();
        let mut name_to_spqr_node_index = FxHashMap::default();
        let mut name_to_spqr_edge_index = FxHashMap::default();

        while let Some(line) = line_reader.next()? {
            match &line[0] {
                "G" => {
                    trace!("Parsing G-line");
                    let component_name = line
                        .column(1)
                        .ok_or(ReadError::MissingComponentNameInGLine)?;
                    let nodes = line
                        .iter()
                        .skip(2)
                        .map(|node| {
                            name_to_node_index
                                .get(node)
                                .copied()
                                .ok_or_else(|| ReadError::UnknownNodeName(node.to_string()))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    trace!(
                        "Found component {component_name} with {} nodes",
                        nodes.len()
                    );

                    if nodes.is_empty() {
                        return Err(ReadError::EmptyComponent);
                    }

                    let component_index = builder.add_component(nodes);
                    name_to_component_index.insert(component_name.to_string(), component_index);
                }
                "N" => {
                    trace!("Parsing N-line");
                    let node_name = line.column(1).ok_or(ReadError::MissingNodeNameInNLine)?;
                    let extra_data = line.iter().skip(2).collect::<Vec<_>>();
                    let extra_data = extra_data.join(" ");
                    let node_index = name_to_node_index
                        .get(node_name)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(node_name.to_string()))?;
                    builder.add_extra_data_to_node(node_index, extra_data);
                }
                "B" => {
                    trace!("Parsing B-line");
                    let block_name = line.column(1).ok_or(ReadError::MissingBlockNameInBLine)?;
                    let component_name = line
                        .column(2)
                        .ok_or(ReadError::MissingComponentNameInBLine)?;
                    let component_index =
                        *name_to_component_index.get(component_name).ok_or_else(|| {
                            ReadError::UnknownComponentName(component_name.to_string())
                        })?;
                    let nodes = line
                        .iter()
                        .skip(3)
                        .map(|node| {
                            name_to_node_index
                                .get(node)
                                .copied()
                                .ok_or_else(|| ReadError::UnknownNodeName(node.to_string()))
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    if nodes.is_empty() {
                        return Err(ReadError::EmptyBlock);
                    }

                    let block_index = builder.add_block(component_index, nodes);
                    name_to_block_index.insert(block_name.to_string(), block_index);
                }
                "C" => {
                    trace!("Parsing C-line");
                    let cut_node_name = line.column(1).ok_or(ReadError::MissingNodeNameInCLine)?;
                    let cut_node_index = name_to_node_index
                        .get(cut_node_name)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(cut_node_name.to_string()))?;
                    let block_indices =
                        line.iter()
                            .skip(2)
                            .map(|block_name| {
                                name_to_block_index.get(block_name).cloned().ok_or_else(|| {
                                    ReadError::UnknownBlockName(block_name.to_string())
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;

                    if block_indices.is_empty() {
                        return Err(ReadError::EmptyCutNode);
                    }

                    builder.add_cut_node(cut_node_index, block_indices);
                }
                "S" | "P" | "R" => {
                    let spqr_node_type = match &line[0] {
                        "S" => {
                            trace!("Parsing S-line");
                            SPQRNodeType::SNode
                        }
                        "P" => {
                            trace!("Parsing P-line");
                            SPQRNodeType::PNode
                        }
                        "R" => {
                            trace!("Parsing R-line");
                            SPQRNodeType::RNode
                        }
                        _ => unreachable!(),
                    };
                    let spqr_node_name = line
                        .column(1)
                        .ok_or(ReadError::MissingSPQRNodeNameInSPRLine)?;
                    let block_name = line.column(2).ok_or(ReadError::MissingBlockNameInSPRLine)?;
                    let block_index = *name_to_block_index
                        .get(block_name)
                        .ok_or_else(|| ReadError::UnknownBlockName(block_name.to_string()))?;
                    let nodes = line
                        .iter()
                        .skip(3)
                        .map(|node| {
                            name_to_node_index
                                .get(node)
                                .copied()
                                .ok_or_else(|| ReadError::UnknownNodeName(node.to_string()))
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    if nodes.len() < 2 {
                        return Err(ReadError::LessThanTwoNodesInSPQRNode);
                    }

                    let spqr_node_index = builder.add_spqr_node(block_index, nodes, spqr_node_type);
                    name_to_spqr_node_index.insert(spqr_node_name.to_string(), spqr_node_index);
                }
                "V" => {
                    trace!("Parsing V-line");
                    let spqr_edge_name = line
                        .column(1)
                        .ok_or(ReadError::MissingSPQREdgeNameInVLine)?;
                    let spqr_node_name_u = line
                        .column(2)
                        .ok_or(ReadError::MissingSPQRNodeNameInVLine)?;
                    let spqr_node_name_v = line
                        .column(3)
                        .ok_or(ReadError::MissingSPQRNodeNameInVLine)?;
                    let node_name_u = line.column(4).ok_or(ReadError::MissingNodeNameInVLine)?;
                    let node_name_v = line.column(5).ok_or(ReadError::MissingNodeNameInVLine)?;
                    let spqr_node_index_u = *name_to_spqr_node_index
                        .get(spqr_node_name_u)
                        .ok_or_else(|| {
                            ReadError::UnknownSPQRNodeName(spqr_node_name_u.to_string())
                        })?;
                    let spqr_node_index_v = *name_to_spqr_node_index
                        .get(spqr_node_name_v)
                        .ok_or_else(|| {
                            ReadError::UnknownSPQRNodeName(spqr_node_name_v.to_string())
                        })?;
                    let node_index_u = name_to_node_index
                        .get(node_name_u)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(node_name_u.to_string()))?;
                    let node_index_v = name_to_node_index
                        .get(node_name_v)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(node_name_v.to_string()))?;

                    let block_index = builder.spqr_node_block_index(spqr_node_index_u);
                    if block_index != builder.spqr_node_block_index(spqr_node_index_v) {
                        return Err(ReadError::SPQREdgeBetweenDifferentBlocks(
                            spqr_edge_name.to_string(),
                        ));
                    }

                    let spqr_edge_index = builder.add_spqr_edge(
                        Some(block_index).into(),
                        (spqr_node_index_u, spqr_node_index_v),
                        (node_index_u, node_index_v),
                    );
                    name_to_spqr_edge_index.insert(spqr_edge_name.to_string(), spqr_edge_index);
                }
                "E" => {
                    trace!("Parsing E-line");
                    let _edge_name = line.column(1).ok_or(ReadError::MissingEdgeNameInELine)?;
                    let spqr_node_or_block_name = line
                        .column(2)
                        .ok_or(ReadError::MissingSPQRNodeOrBlockNameInELine)?;
                    let node_name_u = line.column(3).ok_or(ReadError::MissingNodeNameInELine)?;
                    let node_name_v = line.column(4).ok_or(ReadError::MissingNodeNameInELine)?;

                    let node_index_u = name_to_node_index
                        .get(node_name_u)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(node_name_u.to_string()))?;
                    let node_index_v = name_to_node_index
                        .get(node_name_v)
                        .copied()
                        .ok_or_else(|| ReadError::UnknownNodeName(node_name_v.to_string()))?;

                    let mut edges_between = graph.edges_between(node_index_u, node_index_v);
                    let Some(first) = edges_between.next() else {
                        return Err(ReadError::NoEdgeBetweenNodes(
                            node_name_u.to_string(),
                            node_name_v.to_string(),
                        ));
                    };
                    let second = edges_between.next();
                    let is_multiedge = second.is_some();

                    if let Some(spqr_node_index) = name_to_spqr_node_index
                        .get(spqr_node_or_block_name)
                        .copied()
                    {
                        for edge_index in iter::once(first).chain(second).chain(edges_between) {
                            let result = builder.add_edge_to_spqr_node(edge_index, spqr_node_index);
                            if !is_multiedge {
                                result?;
                            }
                        }
                    } else if let Some(block_index) =
                        name_to_block_index.get(spqr_node_or_block_name).copied()
                    {
                        for edge_index in iter::once(first).chain(second).chain(edges_between) {
                            let result = builder.add_edge_to_block(edge_index, block_index);
                            if !is_multiedge {
                                result?;
                            }
                        }
                    } else {
                        return Err(ReadError::UnknownSPQRNodeOrBlockName(
                            spqr_node_or_block_name.to_string(),
                        ));
                    }
                }
                other => {
                    return Err(ReadError::InvalidLineType(other.to_string()));
                }
            }
        }

        Ok(builder.build())
    }

    /// Write an SPQR decomposition in plain SPQR file format.
    pub fn write_plain_spqr(&self, mut writer: impl Write) -> std::io::Result<()> {
        writeln!(
            writer,
            "H v0.3 https://github.com/sebschmi/SPQR-tree-file-format"
        )?;

        // Write node extra data.
        for node_index in self.iter_nodes() {
            let extra_data = self.node_extra_data(node_index);
            if !extra_data.is_empty() {
                let node_name = self.graph().node_name(node_index);
                writeln!(writer, "N {node_name} {extra_data}")?;
            }
        }

        // Write components.
        for (component_index, component) in self.iter_components() {
            write!(writer, "G G{component_index}")?;

            for node_index in component.iter_nodes() {
                let node_name = self.graph().node_name(node_index);
                write!(writer, " {node_name}")?;
            }

            writeln!(writer)?;

            // Write cut nodes in component.
            for cut_node_index in component.iter_cut_nodes() {
                let cut_node = self.cut_node(cut_node_index);
                let node_name = self.graph().node_name(cut_node.node());
                write!(writer, "C {node_name}")?;

                for block_index in cut_node.iter_adjacent_blocks() {
                    write!(writer, " B{block_index}")?;
                }
                writeln!(writer)?;
            }

            // Write blocks in component.
            for (block_index, block) in self.iter_blocks_in_component(component_index) {
                write!(writer, "B B{block_index} G{component_index}")?;

                for node_index in block.iter_nodes() {
                    let node_name = self.graph().node_name(node_index);
                    write!(writer, " {node_name}")?;
                }
                writeln!(writer)?;

                // Write contained edges.
                for edge_index in block.iter_edges() {
                    let (u, v) = self.graph().edge_endpoints(edge_index);
                    let node_name_u = self.graph().node_name(u);
                    let node_name_v = self.graph().node_name(v);
                    let edge_name = format!("E{edge_index}");
                    writeln!(
                        writer,
                        "E {edge_name} B{block_index} {node_name_u} {node_name_v}",
                    )?;
                }

                // Write contained SPQR nodes.
                for (spqr_node_index, spqr_node) in self.iter_spqr_nodes_in_block(block_index) {
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
                        let node_name = self.graph().node_name(node_index);
                        write!(writer, " {node_name}")?;
                    }
                    writeln!(writer)?;

                    // Write edges (Q-nodes).
                    for edge_index in spqr_node.iter_edges() {
                        let (u, v) = self.graph().edge_endpoints(edge_index);
                        let node_name_u = self.graph().node_name(u);
                        let node_name_v = self.graph().node_name(v);
                        let edge_name = format!("E{edge_index}");

                        write!(
                            writer,
                            "E {edge_name} {spqr_node_name} {node_name_u} {node_name_v}",
                        )?;

                        let extra_data = self.edge_extra_data(edge_index);
                        if !extra_data.is_empty() {
                            write!(writer, " {extra_data}")?;
                        }
                        writeln!(writer)?;
                    }
                }

                // Write contained SPQR edges.
                for (spqr_edge_index, spqr_edge) in self.iter_spqr_edges_in_block(block_index) {
                    let (spqr_node_index_u, spqr_node_index_v) = spqr_edge.endpoints();
                    let spqr_node_name_u = self.spqr_node_name(spqr_node_index_u);
                    let spqr_node_name_v = self.spqr_node_name(spqr_node_index_v);
                    let node_name_u = self.graph().node_name(spqr_edge.virtual_edge().0);
                    let node_name_v = self.graph().node_name(spqr_edge.virtual_edge().1);

                    writeln!(
                        writer,
                        "V V{spqr_edge_index} {spqr_node_name_u} {spqr_node_name_v} {node_name_u} {node_name_v}",
                    )?;
                }
            }
        }

        Ok(())
    }
}
