use std::{
    error::Error,
    fs::read_to_string,
    io::{Write, stdout},
};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let coords: Vec<(u64, u64)> = file
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    let mut i = 0;
    let pairs: Vec<((u64, u64), (u64, u64), u64)> =
        coords.iter().fold(Vec::new(), |mut pairs, coord| {
            i = i + 1;
            print!("\rcurr: {:?}", i);
            let _ = stdout().flush();
            for cand in coords.iter() {
                if cand == coord {
                    continue;
                } else {
                    let dist = cand
                        .0
                        .abs_diff(coord.0 + 1)
                        .strict_mul(cand.1.abs_diff(coord.1 + 1));

                    if !pairs.iter().any(|p| p.0 == *cand && p.1 == *coord) {
                        pairs.push((*coord, *cand, dist));
                    }
                }
            }
            pairs
        });

    let max = pairs.iter().max_by_key(|s| s.2).unwrap();
    println!("{:?}", max);
    Ok(max.2.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09_01() {
        let res = part1("inputs/09/demo.txt").unwrap();
        assert_eq!(res.to_string(), "50");
    }
}
