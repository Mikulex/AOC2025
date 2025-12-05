use std::{
    cmp::{max, min},
    error::Error,
    fs::read_to_string,
    ops::RangeInclusive,
};

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
    let mut ranges: Vec<RangeInclusive<u64>> = read_to_string(file_path)?
        .lines()
        .filter(|l| !l.is_empty())
        .take_while(|l| l.contains('-'))
        .map(|l| l.split('-'))
        .map(|p| p.map(|v| v.parse::<u64>().unwrap()))
        .map(|mut it| it.next().unwrap()..=it.next().unwrap())
        .collect();

    ranges.sort_by(|a, b| a.end().cmp(b.end()));

    let res: usize = ranges
        .iter()
        .fold(vec![], |mut acc: Vec<RangeInclusive<u64>>, i| {
            if acc.is_empty() {
                acc.push(i.clone());
            } else if acc.iter().last().unwrap().end() < i.start() {
                acc.push(i.clone());
            } else {
                let last = acc.swap_remove(acc.len() - 1);
                let start = min(last.start(), i.start());
                let end = max(last.end(), i.end());
                acc.push(start.clone()..=end.clone());
            }
            acc
        })
        .iter()
        .map(|r| r.clone().count())
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_01() {
        let res = part1("inputs/05/demo.txt").unwrap();
        assert_eq!(res, "3");
    }

    #[test]
    fn test_day05_02() {
        let res = part2("inputs/05/demo.txt").unwrap();
        assert_eq!(res, "14");
    }
}
