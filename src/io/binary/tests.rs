use std::fs::{self, File};

use bidirected_adjacency_array::io::gfa1::read_gfa1;

use crate::{decomposition::SPQRDecomposition, io::plain_spqr_file::read_plain_spqr};

#[test]
fn test_tiny1() {
    let graph = read_gfa1::<u8>(&mut File::open("test_files/tiny1.gfa").unwrap()).unwrap();
    let spqr_decomposition_file = fs::read_to_string("test_files/tiny1.spqr").unwrap();
    let spqr_decomposition =
        read_plain_spqr(&graph, &mut spqr_decomposition_file.as_bytes()).unwrap();

    let mut buffer = Vec::new();
    spqr_decomposition.write_binary(&mut buffer).unwrap();
    let read_spqr_decomposition = SPQRDecomposition::read_binary(&graph, &buffer[..]).unwrap();

    assert_eq!(
        spqr_decomposition.components,
        read_spqr_decomposition.components,
    );
    assert_eq!(spqr_decomposition.blocks, read_spqr_decomposition.blocks);
    assert_eq!(
        spqr_decomposition.cut_nodes,
        read_spqr_decomposition.cut_nodes,
    );
    assert_eq!(
        spqr_decomposition.spqr_nodes,
        read_spqr_decomposition.spqr_nodes,
    );
    assert_eq!(
        spqr_decomposition.spqr_edges,
        read_spqr_decomposition.spqr_edges,
    );
    assert_eq!(
        spqr_decomposition.node_data,
        read_spqr_decomposition.node_data,
    );
    assert_eq!(
        spqr_decomposition.edge_data,
        read_spqr_decomposition.edge_data,
    );
}
