use std::io::BufRead;

use crate::io::plain_spqr_file::error::ReadError;

pub fn read_next_line(reader: &mut impl BufRead) -> Result<Option<Vec<String>>, ReadError> {
    for line in reader.lines() {
        let line = line?;
        let line = if let Some(index) = line.find('#') {
            &line[..index]
        } else {
            &line
        };
        let line = line.trim();

        if !line.is_empty() {
            return Ok(Some(
                line.to_string().split(' ').map(|s| s.to_string()).collect(),
            ));
        }
    }

    Ok(None)
}
