use std::env;

use rapid_file_comparison_toolkit::comparison::*;

fn main() -> std::io::Result<()>
{
    // establish environment
    let env_args: Vec<_> = env::args().collect();

    file_cmp(&env_args[FILE_ONE_SEL], &env_args[FILE_TWO_SEL])?;

    Ok(())
}
