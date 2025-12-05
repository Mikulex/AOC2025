use std::{error::Error, fs::read_to_string, ops::RangeInclusive};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let input = read_to_string(file_path)?;

    let (ranges, vals): (Vec<_>, Vec<_>) = input
        .lines()
        .filter(|l| !l.is_empty())
        .partition(|l| l.contains('-'));

    let ranges: Vec<RangeInclusive<u64>> = ranges
        .iter()
        .map(|l| l.split('-'))
        .map(|p| p.map(|v| v.parse::<u64>().unwrap()))
        .map(|mut it| it.next().unwrap()..=it.next().unwrap())
        .collect::<_>();

    let res = vals
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|n| ranges.iter().any(|r| r.contains(n)))
        .count();

    Ok(res.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_01() {
        let res = part1("inputs/05/demo.txt").unwrap();
        assert_eq!(res, "3");
    }

    fn test_day05_02() {
        let res = part2("inputs/05/demo.txt").unwrap();
    }
}
