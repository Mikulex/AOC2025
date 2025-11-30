use std::{error::Error, fs};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let _contents = fs::read_to_string(file_path)?;
    return Ok(String::from("TODO"));
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let _contents = fs::read_to_string(file_path)?;
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let res = part1("inputs/01/test.txt").unwrap();
        assert_eq!(res, "TODO");
    }
}
