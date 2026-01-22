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

    #[error("the header line does not contain a URL pointing to the specification")]
    MissingHeaderUrl,
}
