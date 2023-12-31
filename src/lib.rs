//! Progress Scryer a program to track the time loading files on the command line.

pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const CHUNK_SIZE: usize = 16 * 1024;
