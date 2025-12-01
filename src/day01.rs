use std::error::Error;
use std::fs;

pub fn part1(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut zero_count = 0;

    std::fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.split_at(1))
        .map(|(dir, c)| (dir, c.parse::<i32>().unwrap()))
        .map(|r| if r.0 == "L" { -r.1 } else { r.1 })
        .fold(50, |acc, i| {
            let res = (acc + i) % 100;
            if res == 0 {
                zero_count += 1;
            }
            res
        });

    return Ok(zero_count.to_string());
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut zero_count = 0;

    fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.split_at(1))
        .map(|(dir, c)| (dir, c.parse::<i32>().unwrap()))
        .map(|r| if r.0 == "L" { -r.1 } else { r.1 })
        .fold(50, |acc: i32, mut i| {
            zero_count += i.abs() / 100;
            i %= 100;

            let res = acc + i;

            if res % 100 == 0 || (acc != 0 && (res < 0 || res > 100)) {
                zero_count += 1
            }

            (if res < 0 { res + 100 } else { res }) % 100
        });

    return Ok(zero_count.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let res = part1("inputs/01/demo.txt").unwrap();
        assert_eq!(res, "3");
    }

    #[test]
    fn test_day01_part2_res() {
        let res = part2("inputs/01/demo.txt").unwrap();
        assert_eq!(res, "6");
    }

    #[test]
    fn test_day01_part2_single_lap() {
        let res = part2("inputs/01/1klap.txt").unwrap();
        assert_eq!(res, "10");
    }

    #[test]
    fn test_day01_part2_lap_with_undershoot() {
        let res = part2("inputs/01/lap_under.txt").unwrap();
        assert_eq!(res, "10");
    }

    #[test]
    fn test_day01_part2_alt_after_zero() {
        let res = part2("inputs/01/alt_after_zero.txt").unwrap();
        assert_eq!(res, "2");
    }
}
