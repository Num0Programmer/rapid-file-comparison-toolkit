use std::env;
use std::fs;

use rapid_file_comparison_toolkit::comparison::*;

fn main() -> std::io::Result<()>
{
    // establish environment
    let env_args: Vec<_> = env::args().collect();

    let key_tup = (
        fs::metadata(&env_args[FILE_ONE_SEL])?.is_dir(),
        fs::metadata(&env_args[FILE_TWO_SEL])?.is_dir()
    );
    match key_tup
    {
        // check for directory to directory comparison
        (true, true) => {
            println!("Directory to directory comparison is not supported yet!");
        },
        // check for directory to file comparison
        (true, false) => {
            dir_file_cmp(&env_args[FILE_ONE_SEL], &env_args[FILE_TWO_SEL])?;
        },
        (false, true) => {
            dir_file_cmp(&env_args[FILE_TWO_SEL], &env_args[FILE_ONE_SEL])?;
        },
        // assume file to file comparison
        _ => { file_cmp(&env_args[FILE_ONE_SEL], &env_args[FILE_TWO_SEL])?; }
    }

    Ok(())
}
