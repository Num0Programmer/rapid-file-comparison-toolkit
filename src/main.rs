fn main()
{
    let total_lines = 0; // total lines in file one
    let mut lines_equal = 0;
    let mut processed_lines = 0;
    
    // try to open first file

    // try to open second file

    // check files have same length
    {
        // log warning for different length files
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
}

