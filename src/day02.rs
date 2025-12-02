use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res: u64 = read_to_string(file_path)?
        .split(",")
        .map(|c: &str| c.split("-").map(|s: &str| s.parse::<u64>().unwrap()))
        .flat_map(|mut r| {
            (r.next().unwrap()..=r.next().unwrap())
                .map(|s| s.to_string())
                .filter(|s| s.len() % 2 == 0)
                .filter(|s| {
                    let (l, r): (&str, &str) = s.split_at(s.len() / 2);
                    return l.eq(r);
                })
                .map(|s| s.parse::<u64>().unwrap())
        })
        .sum();

    return Ok(res.to_string());
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let res = part1("inputs/02/demo.txt").unwrap();
        assert_eq!(res, "1227775554");
    }
}
