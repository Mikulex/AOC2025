use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let sum: u32 = read_to_string(file_path)?
        .lines()
        .map(|l| largest_num(l))
        .sum();
    Ok(sum.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let sum: u32 = read_to_string(file_path)?
        .lines()
        .map(|l| largest_num(l))
        .sum();
    Ok(sum.to_string())
}

fn largest_num(line: &str) -> u32 {
    let (l_idx, l_val) = largest_digit(&line[..line.len() - 1]);
    let (_, r_val) = largest_digit(&line[(l_idx + 1)..line.len()]);
    l_val * 10 + r_val
}

fn largest_digit(arr: &str) -> (usize, u32) {
    arr.split("")
        .filter_map(|s| s.parse::<u32>().ok())
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
        let res = largest_num("123456789");
        assert_eq!(res, 89);
    }

    fn test_day03_02_demo() {
        let res = part2("inputs/03/demo.txt").unwrap();
        assert_eq!(res, "");
    }
}
