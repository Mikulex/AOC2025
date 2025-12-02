use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res: u64 = read_to_string(file_path)?
        .split(",")
        .map(|c: &str| c.split("-").filter_map(|s: &str| s.parse::<u64>().ok()))
        .flat_map(|mut r| r.next().unwrap()..=r.next().unwrap())
        .map(|s| s.to_string())
        .filter(|s| s.len() % 2 == 0)
        .filter(|s| {
            let (l, r): (&str, &str) = s.split_at(s.len() / 2);
            return l.eq(r);
        })
        .filter_map(|s| s.parse::<u64>().ok())
        .sum();

    return Ok(res.to_string());
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let res: u64 = read_to_string(file_path)?
        .split(",")
        .map(|c: &str| c.split("-").filter_map(|s: &str| s.parse::<u64>().ok()))
        .flat_map(|mut r| r.next().unwrap()..=r.next().unwrap())
        .map(|s| s.to_string())
        .filter(|s| {
            (1..=s.len() / 2).filter(|i| s.len() % i == 0).any(|i| {
                let mut chunks = s.as_bytes().chunks(i);

                let first = &s[..i];
                chunks.all(|c| c == first.as_bytes())
            })
        })
        .filter_map(|s| s.parse::<u64>().ok())
        .sum();

    return Ok(res.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_01() {
        let res = part1("inputs/02/demo.txt").unwrap();
        assert_eq!(res, "1227775554");
    }

    #[test]
    fn test_day02_02_single() {
        let res = part2("inputs/02/single.txt").unwrap();
        assert_eq!(res, "210");
    }

    #[test]
    fn test_day02_02_demo() {
        let res = part2("inputs/02/demo.txt").unwrap();
        assert_eq!(res, "4174379265");
    }
}
