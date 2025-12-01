use std::{
    error::Error,
    fs::{self},
};

#[derive(Debug)]
struct Rot {
    pub count: i32,
    pub dir: String,
}

impl Rot {
    pub fn new(line: &str) -> Self {
        let split = line.split_at(1);
        let count = split.1.parse().unwrap();
        let dir = split.0.to_owned();
        Self { count, dir }
    }
}

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut current = 50;
    let mut zero_count = 0;

    fs::read_to_string(file_path)?
        .lines()
        .map(Rot::new)
        .for_each(|r| {
            if r.dir == "R" {
                current += r.count;
            } else {
                current -= r.count;
            }
            current %= 100;

            if current == 0 {
                zero_count += 1;
            }
        });
    return Ok(zero_count.to_string());
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
        let res = part1("inputs/01/demo.txt").unwrap();
        assert_eq!(res, "3");
    }

    #[test]
    fn test_rot_new() {
        let input = "L32";
        let res = Rot {
            count: 32,
            dir: "L".to_owned(),
        };
        let test = Rot::new(input);

        assert_eq!(res.count, test.count);
        assert_eq!(res.dir, test.dir);
    }
}
