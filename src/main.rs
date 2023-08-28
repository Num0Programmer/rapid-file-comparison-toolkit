use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use log::{info, warn, error};

// arg selecting
const FILE_ONE_SEL: usize = 1;
const FILE_TWO_SEL: usize = 2;


fn main() -> std::io::Result<()>
{
    let args: Vec<_> = env::args().collect();
    let file_1_str = &args[FILE_ONE_SEL];
    let file_2_str = &args[FILE_TWO_SEL];

    // TODO: impl custom errors
    // try to open first file
    let file_1 = File::open(&file_1_str)?;

    // try to open second file
    let file_2 = File::open(&file_2_str)?;

    let total_lines = fs::metadata(&file_1_str)?.len(); // total lines in file one
    let mut lines_equal = 0;
    let mut processed_lines = 0;    

    // check files have same length
    if total_lines != fs::metadata(&file_2_str)?.len()
    {
        warn!("Warning: The length of provided files is not the same!");
    }

    // create BufReaders for files

    // read next lines until EOF of either or both files
    {
        // check line bufs eq
        {
            lines_equal += 1;
        }
        // otherwise, assume lines are not equal
        {
            // log line number and text from file(s)
        }
        processed_lines += 1;
        
        // clear buffers?
    }

    // log number of lines processed
    // log number of lines equal out of total lines in first file

    Ok(())
}

