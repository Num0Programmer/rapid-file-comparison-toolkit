use std::env;
use std::fs;

use rapid_file_comparison_toolkit::comparison::*;

fn main() -> std::io::Result<()>
{
    // establish environment
    let env_args: Vec<_> = env::args().collect();

    // check for dir to file compare
    if fs::metadata(&env_args[FILE_ONE_SEL])?.is_dir() ||
        fs::metadata(&env_args[FILE_TWO_SEL])?.is_dir()
    {
        // compare directory/ies
    }
    // otherwise, assume comparing two files
    else
    {
        file_cmp(&env_args[FILE_ONE_SEL], &env_args[FILE_TWO_SEL])?;
    }

    Ok(())
}
