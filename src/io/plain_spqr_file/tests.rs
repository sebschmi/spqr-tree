use std::fs::{self, File};

use bidirected_adjacency_array::io::gfa1::read_gfa1;
use regex::Regex;

use crate::io::plain_spqr_file::{read_plain_spqr, write_plain_spqr};

#[test]
fn test_tiny1() {
    let remove_edge_labels = Regex::new(r" E\d+ ").unwrap();

    let graph = read_gfa1::<u8>(&mut File::open("test_files/tiny1.gfa").unwrap()).unwrap();
    let spqr_decomposition_file = fs::read_to_string("test_files/tiny1.spqr").unwrap();
    let spqr_decomposition =
        read_plain_spqr(&graph, &mut spqr_decomposition_file.as_bytes()).unwrap();

    let mut buffer = Vec::new();
    write_plain_spqr(&spqr_decomposition, &mut buffer).unwrap();

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
