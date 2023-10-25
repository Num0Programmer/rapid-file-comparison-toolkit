use std::env;
use std::fs;

use rfctk::comparison::*;

fn main() -> std::io::Result<()>
{
    // establish environment
    let env_args: Vec<_> = env::args().collect();

    // initialize statistics structure
    let mut stats = ComparisonStats::new();

    let key_tup = (
        fs::metadata(&env_args[ARG_ONE_SEL])?.is_dir(),
        fs::metadata(&env_args[ARG_TWO_SEL])?.is_dir()
    );
    match key_tup
    {
        // check for directory to directory comparison
        (true, true) =>
        {
            println!("Directory to directory comparison is not supported yet!");
        },
        // check for directory to file comparison
        (true, false) =>
        {
            dir_file_cmp(&mut stats, &env_args[ARG_ONE_SEL], &env_args[ARG_TWO_SEL])?;
        },
        (false, true) =>
        {
            dir_file_cmp(&mut stats, &env_args[ARG_TWO_SEL], &env_args[ARG_ONE_SEL])?;
        },
        // assume file to file comparison
        _ =>
        {
            file_cmp(&mut stats, &env_args[ARG_ONE_SEL], &env_args[ARG_TWO_SEL])?;
        }
    }

    println!("{} lines processed", stats.total_lines_processed);
    println!("{} out of {} lines were equivalent.",
        stats.total_lines_equal, stats.total_lines_processed
    );

    Ok(())
}
