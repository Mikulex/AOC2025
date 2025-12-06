use std::{error::Error, fs::read_to_string};

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let lines: Vec<Vec<&str>> = file
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect::<_>();

    let res: u64 = lines[0]
        .iter()
        .zip(lines[1].iter())
        .zip(lines[2].iter())
        .zip(lines[3].iter())
        .zip(lines[4].iter())
        .map(|((((a, b), c), d), op)| {
            (
                vec![
                    a.parse::<u64>().unwrap(),
                    b.parse::<u64>().unwrap(),
                    c.parse::<u64>().unwrap(),
                    d.parse::<u64>().unwrap(),
                ],
                op,
            )
        })
        .map(|(nums, op)| match op {
            &"+" => nums.iter().sum::<u64>(),
            &"*" => nums.iter().product::<u64>(),
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
