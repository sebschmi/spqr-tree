use std::{io::BufRead, ops::Index};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LineReaderError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
}

pub struct LineReader<Reader> {
    reader: Reader,
    buffer: Vec<u8>,
    columns: Vec<usize>,
}

pub struct Columns<'a> {
    buffer: &'a [u8],
    columns: &'a [usize],
}

pub struct ColumnsIter<'a, 'columns> {
    columns: &'columns Columns<'a>,
    index: usize,
}

impl<Reader: BufRead> LineReader<Reader> {
    pub fn new(reader: Reader) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            columns: Vec::new(),
        }
    }

    /// Advances to the next non-empty line, returning `true` if a line was read and `false` if the end of the file was reached.
    pub fn next(&mut self) -> Result<Option<Columns<'_>>, LineReaderError> {
        loop {
            self.buffer.clear();
            // UTF-8 encoded strings contain the byte 0A only for newline chars.
            // Therefore, we can read a complete line simply like this.
            let bytes_read = self.reader.read_until(b'\n', &mut self.buffer)?;
            let string = str::from_utf8(&self.buffer)?;

            if bytes_read == 0 {
                return Ok(None);
            }

            self.columns.clear();
            let mut has_non_whitespace = false;
            for (index, c) in string.char_indices() {
                if c == ' ' || c == '\n' {
                    self.columns.push(index);
                } else if c == '#' {
                    // Skip the rest of the line after a comment character.
                    self.columns.push(index);
                    break;
                } else {
                    has_non_whitespace = true;
                }
            }

            if has_non_whitespace {
                return Ok(Some(Columns {
                    buffer: &self.buffer,
                    columns: &self.columns,
                }));
            }
        }
    }
}

impl<'a> Columns<'a> {
    /// Returns the column at the given index, or `None` if the index is out of bounds.
    pub fn column(&self, index: usize) -> Option<&'a str> {
        if index >= self.columns.len() {
            return None;
        }

        let end = self.columns[index];
        if index == 0 {
            // Safety: The `columns` vector is constructed from valid UTF-8 strings, so the indices are guaranteed to be valid UTF-8 boundaries.
            Some(unsafe { str::from_utf8_unchecked(&self.buffer[..end]) })
        } else {
            // Add one to exclude the space character.
            let start = self.columns[index - 1] + 1;
            // Safety: The `columns` vector is constructed from valid UTF-8 strings, so the indices are guaranteed to be valid UTF-8 boundaries.
            Some(unsafe { str::from_utf8_unchecked(&self.buffer[start..end]) })
        }
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn iter(&self) -> ColumnsIter<'a, '_> {
        ColumnsIter {
            columns: self,
            index: 0,
        }
    }
}

impl<'a> Index<usize> for Columns<'a> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.column(index).unwrap()
    }
}

impl<'a, 'columns> Iterator for ColumnsIter<'a, 'columns> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.columns.len() {
            return None;
        }

        let column = self.columns.column(self.index);
        self.index += 1;
        column
    }
}
