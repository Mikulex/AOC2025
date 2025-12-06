use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let lines: Vec<Vec<&str>> = file
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let num_cols = lines[0].len();
    let num_rows = lines.len();

    let res: u64 = (0..num_cols)
        .map(|col| {
            let mut nums = vec![];
            for row in 0..num_rows - 1 {
                nums.push(lines[row][col].parse::<u64>().unwrap());
            }
            let op = lines[num_rows - 1][col];
            (nums, op)
        })
        .map(|(nums, op)| match op {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            a => panic!("inval: {a}, inputs: {:?} {:?}", nums, op),
        })
        .sum();

    Ok(res.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let _ = file_path;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06_01() {
        let res = part1("inputs/06/demo.txt").unwrap();
        assert_eq!(res, "4277556");
    }

    fn test_day06_02() {
        let res = part2("inputs/06/demo.txt").unwrap();
        assert_eq!(res, "14");
    }
}
