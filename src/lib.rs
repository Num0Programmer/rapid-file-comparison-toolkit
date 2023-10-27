pub mod comparison;

// pub use comparison::*;
pub use comparison::*;

#[cfg(test)]
mod test
{
    use crate::ComparisonStats;
    use crate::dir_cmp;
    use crate::file_cmp;
    
    #[test]
    fn file_to_file_cmp() -> std::io::Result<()>
    {
        // instantiate comparison stats
        let mut stats = ComparisonStats::new();
        // initialize strings for testing
        let test_file_str = String::from("tests/data/test_file.txt");
        let other_test_file_str = String::from("tests/data/other_test_file.txt");

        file_cmp(&mut stats, &test_file_str, &other_test_file_str)?;

        // check stats for accuracy
        assert_eq!(stats.total_lines_equal, 3);
        assert_eq!(stats.total_lines_processed, 3);

        Ok(())
    }

    #[test]
    fn file_to_short_file_cmp() -> std::io::Result<()>
    {
        // instantiate comparison stats
        let mut stats = ComparisonStats::new();
        // initialize strings for testing
        let test_file_str = String::from("tests/data/test_file.txt");
        let small_test_file_str = String::from("tests/data/small_test_file.txt");

        file_cmp(&mut stats, &test_file_str, &small_test_file_str)?;

        // check stats for accuracy
        assert_eq!(stats.total_lines_equal, 0);
        assert_eq!(stats.total_lines_processed, 1);

        Ok(())
    }

    #[test]
    fn dir_to_dir_cmp() -> std::io::Result<()>
    {
        // instantiate comparison stats
        let mut stats = ComparisonStats::new();
        // initialize strings for testing
        let test_dir_str = String::from("tests/data/test_directory");
        let other_test_dir_str = String::from("tests/data/other_test_directory");

        dir_cmp(&mut stats, &test_dir_str, &other_test_dir_str)?;

        // check stats for accuracy
        assert_eq!(stats.total_lines_equal, 13);
        assert_eq!(stats.total_lines_processed, 13);
        assert_eq!(stats.total_shared_files, 3);

        Ok(())
    }
}
