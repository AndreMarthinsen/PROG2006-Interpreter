use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

/// get_tokens retrieves text from a specified file or STDIN
///
/// # Arguments
///
/// `in_file` - if None, reads from STDIO, if Some(&mut File), reads from file.
///
/// # Returns
///
/// Vec\<String\> on a successful read, io::Error otherwise.
///
/// # Examples
///
/// ```
/// use bprog::utility::get_input;
/// use tempfile::tempfile;
/// use std::io::{Seek, Write, SeekFrom};
///
/// let input_str = "Hello, world!";
///     let mut input_file = tempfile::tempfile().unwrap();
///     input_file.write_all(input_str.as_bytes()).unwrap();
///     input_file.seek(SeekFrom::Start(0)).unwrap();
///     let result = get_input(Some(&mut input_file));
///     assert_eq!(result.unwrap(), input_str.to_string());
/// ```
///
pub fn get_input(in_file: Option<&mut File>) -> Result<String, io::Error> {
    let mut program_text = String::new();
    let read = match in_file {
        Some(f) => {
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut program_text)
        },
        None => io::stdin().read_line(&mut program_text)
    };
     match read {
        Ok(_) => {
            Ok(program_text)
        }
        Err(e) => Err(e)
    }
}

/// Splits a string into tokens delimited by whitespace.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use bprog::utility::{string_vec_deque, to_tokens};
/// let tokens = string_vec_deque(&["one", "two", "three"]);
///
/// assert_eq!(tokens, to_tokens("one two three"));
///
/// ```
///
pub fn to_tokens(input: &str) -> VecDeque<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<VecDeque<_>>()
}


pub fn string_vec(vec: &[&str]) -> Vec<String> {
    vec.iter().map(|s| s.to_string()).collect()
}

pub fn string_vec_deque(vec: &[&str]) -> VecDeque<String> {
    VecDeque::from(string_vec(vec))
}