use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    calculate(file_path, |l| largest_num(l, 2))
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    calculate(file_path, |l| largest_num(l, 12))
}

fn calculate(file_path: &str, f: impl Fn(&str) -> u64) -> Result<String, Box<dyn Error>> {
    let sum: u64 = read_to_string(file_path)?.lines().map(f).sum();
    Ok(sum.to_string())
}

fn largest_num(line: &str, digits: usize) -> u64 {
    let mut idx = 0;
    let mut res = 0;
    for num in (0..digits).rev() {
        let (n_idx, val) = largest_digit(&line[idx..(line.len() - num)]);
        idx += 1 + n_idx;
        res += val * 10_u64.pow(num as u32)
    }
    res
}

fn largest_digit(arr: &str) -> (usize, u64) {
    arr.chars()
        .map(|s| (s.to_digit(10)).unwrap() as u64)
        .enumerate()
        .fold((0, 0), |(c_idx, c_val), (idx, val)| {
            if val > c_val {
                (idx, val)
            } else {
                (c_idx, c_val)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_01() {
        let res = part1("inputs/03/demo.txt").unwrap();
        assert_eq!(res, "357");
    }

    #[test]
    fn test_day03_01_single_line() {
        let res = largest_num("987654321111111", 2);
        assert_eq!(res, 98);
    }

    #[test]
    fn test_day03_02_single_line() {
        let res = largest_num("987654321111111", 12);
        assert_eq!(res, 987654321111);
    }

    #[test]
    fn test_day03_02_demo() {
        let res = part2("inputs/03/demo.txt").unwrap();
        assert_eq!(res, "3121910778619");
    }
}
