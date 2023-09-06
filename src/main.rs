use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[macro_use]
extern crate simple_log;

// arg select
const FILE_ONE_SEL: usize = 1;
const FILE_TWO_SEL: usize = 2;


fn main() -> std::io::Result<()>
{
    let args: Vec<_> = env::args().collect();
    simple_log::quick!();

    let file_1_str = &args[FILE_ONE_SEL];
    let file_2_str = &args[FILE_TWO_SEL];

    // TODO: impl custom errors
    // try to open first file
    let file_1 = File::open(&file_1_str)?;

    // try to open second file
    let file_2 = File::open(&file_2_str)?;

    // initialize comparison information
    let mut lines_equal = 0;
    let mut processed_lines = 0;

    // create BufReaders for files
    let file_1_buf = BufReader::new(file_1);
    let file_2_buf = BufReader::new(file_2);

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

    println!("{} lines processed", processed_lines);
    println!(
        "{} out of {} lines were equivalent.",
        lines_equal, processed_lines
    );

    Ok(())
}

