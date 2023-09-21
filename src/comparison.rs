use std::fs::{self, File, DirEntry};
use std::io::BufReader;
use std::io::prelude::*;


/// constants to increase readability when accessing command line input
pub const FILE_ONE_SEL: usize = 1;
pub const FILE_TWO_SEL: usize = 2;


/// compares contents of a directory to a single file
pub fn dir_file_cmp(dir: &String, file_str: &String) -> std::io::Result<()>
{
    // try to open directory
    //let dir = fs::read_dir(&dir)?;

    // try to open file
    //let file = File::open(&file_str)?;

    for entry in fs::read_dir(dir)?
    {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    Ok(())
}


/// compares two files at given file paths
pub fn file_cmp(file_1_str: &String, file_2_str: &String) -> std::io::Result<()>
{
    // try to open first file
    let file_1 = File::open(&file_1_str)?;

    // try to open second file
    let file_2 = File::open(&file_2_str)?;

    // initialize comparison information
    let mut lines_equal = 0;
    let mut processed_lines = 0;

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
                file_1_str, processed_lines + 1, str_1_buf.trim()
            );
            println!("{}: {}: {}\n",
                file_2_str, processed_lines + 1, str_2_buf.trim()
            );
        }

        processed_lines += 1;
    }

    println!("{} lines processed", processed_lines);
    println!("{} out of {} lines were equivalent.",
        lines_equal, processed_lines
    );

    Ok(())
}