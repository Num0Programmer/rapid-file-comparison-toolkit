use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;


/// constants to increase readability when accessing command line input
pub const ARG_ONE_SEL: usize = 1;
pub const ARG_TWO_SEL: usize = 2;


/// structure to store statistics about the comparison
pub struct ComparisonStats
{
    /// tracks number of lines which were equal between all files given
    pub total_lines_equal: u32,
    /// tracks number of lines which were processed between all files given
    pub total_lines_processed: u32
}

impl ComparisonStats
{
    /// initializes a new comparison statistics structure
    pub fn new() -> Self
    {
        Self
        {
            total_lines_equal: 0,
            total_lines_processed: 0
        }
    }
}


/// compares contents of a directory to a single file
pub fn dir_file_cmp(
    stats: &mut ComparisonStats,
    dir: &String,
    cmp_file_str: &String
) -> std::io::Result<()>
{
    for entry in fs::read_dir(dir)?
    {
        // convert path buffer into file path
        let file_str = entry?.path()
            .into_os_string()
            .into_string()
            .unwrap();

        println!("Comparing {} to {}...\n", file_str, cmp_file_str);
        file_cmp(stats, &file_str, cmp_file_str)?;
    }

    Ok(())
}


/// compares two files at given file paths
pub fn file_cmp(
    stats: &mut ComparisonStats,
    file_str: &String,
    cmp_file_str: &String
) -> std::io::Result<()>
{
    // try to open first file
    let file_1 = File::open(&file_str)?;

    // try to open second file
    let file_2 = File::open(&cmp_file_str)?;

    // tracks number of lines equal between these two files
    let mut lines_equal = 0;
    // tracks number of lines processed in these two files
    let mut lines_processed = 0;

    // create BufReaders for files
    let mut file_1_buf = BufReader::new(file_1);
    let mut file_2_buf = BufReader::new(file_2);
    let mut str_1_buf = String::new();
    let mut str_2_buf = String::new();

    // read another line from both buffers until EOF for either file
    while file_1_buf.read_line(&mut str_1_buf)? > 0
        && file_2_buf.read_line(&mut str_2_buf)? > 0
    {
        // check line bufs eq
        if str_1_buf.eq(&str_2_buf)
        {
            lines_equal += 1;
        }
        // otherwise, assume lines are not equal
        else
        {
            // log line number and text from file(s)
            println!("Warning: The following lines do not match!");
            println!("{}: {}: {}",
                file_str, lines_processed + 1, str_1_buf.trim()
            );
            println!("{}: {}: {}\n",
                cmp_file_str, lines_processed + 1, str_2_buf.trim()
            );
        }

        lines_processed += 1;
    }

    stats.total_lines_equal += lines_equal;
    stats.total_lines_processed += lines_processed;

    Ok(())
}
