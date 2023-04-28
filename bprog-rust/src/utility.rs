use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

/// get_tokens retrieves text tokens from a specified file or STDIN
///
/// # Arguments
///
/// `in_file` - if None, reads from STDIO, if Some(&mut File), reads from file.
///
/// # Returns
///
/// Vec<String> on a successful read, io::Error otherwise.
///
pub fn get_tokens(in_file: Option<&mut File>) -> Result<Vec<String>, io::Error> {
    let mut program_text = String::new();
    let mut read = match in_file {
        Some(f) => {
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut program_text)
        },
        None => io::stdin().read_line(&mut program_text)
    };
    return match read {
        Ok(_) => {
            let tokens = program_text
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            Ok(tokens)
        }
        Err(e) => Err(e)
    }
}