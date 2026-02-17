use std::{
    fs::{self, File},
    io::BufReader,
};

use bidirected_adjacency_array::{
    graph::BidirectedAdjacencyArray,
    io::gfa1::{PlainGfaEdgeData, PlainGfaNodeData},
};
use regex::Regex;

use crate::decomposition::SPQRDecomposition;

#[test]
fn test_tiny1() {
    let remove_edge_labels = Regex::new(r" E\d+ ").unwrap();

    let graph = BidirectedAdjacencyArray::<u8, PlainGfaNodeData, PlainGfaEdgeData>::read_gfa1(
        BufReader::new(File::open("test_files/tiny1.gfa").unwrap()),
    )
    .unwrap();
    let spqr_decomposition_file = fs::read_to_string("test_files/tiny1.spqr").unwrap();
    let spqr_decomposition =
        SPQRDecomposition::read_plain_spqr(&graph, &mut spqr_decomposition_file.as_bytes())
            .unwrap();

    let mut buffer = Vec::new();
    spqr_decomposition.write_plain_spqr(&mut buffer).unwrap();

    let spqr_decomposition_file = remove_edge_labels.replace_all(&spqr_decomposition_file, " E ");
    let mut spqr_decomposition_file_sorted = spqr_decomposition_file
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    spqr_decomposition_file_sorted.sort_unstable();
    let spqr_decomposition_file_sorted = spqr_decomposition_file_sorted;

    let buffer = std::str::from_utf8(&buffer).unwrap();
    let buffer = remove_edge_labels.replace_all(buffer, " E ");
    let mut buffer_sorted = buffer
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    buffer_sorted.sort_unstable();
    let buffer_sorted = buffer_sorted;

    assert_eq!(spqr_decomposition_file_sorted, buffer_sorted);
}
