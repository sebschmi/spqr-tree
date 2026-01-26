#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid line type: {0}. Expected one of G, N, B, C, S, P, R, V, E")]
    InvalidLineType(String),

    #[error("the file does not start with a header line")]
    MissingHeader,

    #[error("the file fromat version is unsupported. Supported is version v0.1")]
    UnsupportedVersion,

    #[error("the header does not contain a URL pointing to the specification")]
    MissingHeaderUrl,

    #[error("a G-line is missing the component name")]
    MissingComponentNameInGLine,

    #[error("an empty component was declared")]
    EmptyComponent,

    #[error("an N-line is missing the node name")]
    MissingNodeNameInNLine,

    #[error("unknown node name: {0}")]
    UnknownNodeName(String),

    #[error("a B-line is missing the block name")]
    MissingBlockNameInBLine,

    #[error("a B-line is missing the component name")]
    MissingComponentNameInBLine,

    #[error("unknown component name: {0}")]
    UnknownComponentName(String),

    #[error("an empty block was declared")]
    EmptyBlock,

    #[error("a C-line is missing the node name")]
    MissingNodeNameInCLine,

    #[error("unknown block name: {0}")]
    UnknownBlockName(String),

    #[error("a cut node with no incident blocks was declared")]
    EmptyCutNode,

    #[error("a S/P/R-node is missing its name")]
    MissingSPQRNodeNameInSPRLine,

    #[error("a S/P/R-node is missing its block name")]
    MissingBlockNameInSPRLine,

    #[error("a S/P/R-node has less than two nodes")]
    LessThanTwoNodesInSPQRNode,

    #[error("a V-line is missing the SPQR edge name")]
    MissingSPQREdgeNameInVLine,

    #[error("a V-line is missing an SPQR node name")]
    MissingSPQRNodeNameInVLine,

    #[error("a V-line is missing a node name")]
    MissingNodeNameInVLine,

    #[error("unknown SPQR node name: {0}")]
    UnknownSPQRNodeName(String),

    #[error("a SPQR edge connects SPQR nodes from different blocks: {0}")]
    SPQREdgeBetweenDifferentBlocks(String),

    #[error("an E-line is missing the edge name")]
    MissingEdgeNameInELine,

    #[error("an E-line is missing the SPQR node name")]
    MissingSPQRNodeNameInELine,

    #[error("an E-line is missing the block name")]
    MissingBlockNameInELine,

    #[error("an E-line is missing a node name")]
    MissingNodeNameInELine,

    #[error("the declared edge {0} does not exist in the graph")]
    EdgeDoesNotExist(String),
}
