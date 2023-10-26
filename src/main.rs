use std::env;
use std::fs;

use rfctk::*;

fn main() -> std::io::Result<()>
{
    // establish environment
    let env_args: Vec<String> = env::args().collect();

    // check user wants help
    if env_args[ARG_ONE_SEL].eq("help")
    {
        help();
    }
    // otherwise, assume user wants to compare filesystem objects
    else
    {
        // initialize statistics structure
        let mut stats = ComparisonStats::new();
        // collect filesystem object names for comparison
        let key_tup = (
            fs::metadata(&env_args[ARG_ONE_SEL])?.is_dir(),
            fs::metadata(&env_args[ARG_TWO_SEL])?.is_dir()
        );

        match key_tup
        {
            // check for directory to directory comparison
            (true, true) =>
            {
                dir_dir_cmp(&mut stats, &env_args[ARG_ONE_SEL], &env_args[ARG_TWO_SEL])?;
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

        // give report of comparison
        println!("{} lines processed", stats.total_lines_processed);
        println!("{} out of {} lines were equivalent.",
            stats.total_lines_equal, stats.total_lines_processed
        );
    }

    Ok(())
}


fn help()
{
    // present general usage information
    println!("usage: rfc <file> <other_file>");
    println!("           <file> <dir>");
    println!("           <dir>  <other_dir>\n");
}
