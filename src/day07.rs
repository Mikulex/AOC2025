use std::{collections::HashSet, error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let grid = file.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    let max_y = grid.len();

    let start = grid[0].iter().position(|c| c == &b'S').unwrap();
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut split_counter = 0;
    points.insert((start, 0));

    for _ in 0..(max_y - 1) {
        let mut new_points: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in &points {
            if grid[y + 1][*x] != b'.' {
                new_points.insert((x + 1, y + 1));
                new_points.insert((x - 1, y + 1));
                split_counter += 1;
            } else {
                new_points.insert((*x, y + 1));
            }
        }
        points = new_points
    }

    Ok(split_counter.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07_01() {
        let res = part1("inputs/07/demo.txt").unwrap();
        assert_eq!(res, "21");
    }

    fn test_day07_02() {
        let res = part2("inputs/07/demo.txt").unwrap();
        assert_eq!(res, "");
    }
}
