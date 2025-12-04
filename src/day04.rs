use std::{error::Error, fs::read_to_string};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut matrix: Vec<Vec<char>> = read_to_string(file_path)?
        .lines()
        .map(|l| l.chars().collect::<_>())
        .collect();

    let count = remove(&mut matrix);

    return Ok(String::from(count.to_string()));
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut matrix: Vec<Vec<char>> = read_to_string(file_path)?
        .lines()
        .map(|l| l.chars().collect::<_>())
        .collect();

    let mut count = 0;
    let mut removed: i32 = remove(&mut matrix);
    while removed > 0 {
        count += removed;
        removed = remove(&mut matrix);
    }

    return Ok(String::from(count.to_string()));
}

fn remove(matrix: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    let len = matrix.len() as i32;

    for x in 0..len {
        for y in 0..len {
            if matrix[x as usize][y as usize] != '@' {
                continue;
            }

            let mut free = 0;

            for i in (x - 1)..=(x + 1) {
                for j in (y - 1)..=(y + 1) {
                    if i == x && j == y {
                        continue;
                    }

                    if outbounds(i, len) {
                        free += 1;
                    } else if outbounds(j, len) {
                        free += 1;
                    } else {
                        let (i, j): (usize, usize) = (i.try_into().unwrap(), j.try_into().unwrap());
                        if matrix[i][j] == '.' {
                            free += 1;
                        }
                    }
                }
            }

            if free > 4 {
                count += 1;
                matrix[x as usize][y as usize] = 'x';
            }
        }
    }

    for x in 0..len {
        for y in 0..len {
            if matrix[x as usize][y as usize] == 'x' {
                matrix[x as usize][y as usize] = '.';
            }
        }
    }
    count
}

fn outbounds(i: i32, len: i32) -> bool {
    i < 0 || i >= len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_01() {
        let res = part1("inputs/04/demo.txt").unwrap();
        assert_eq!(res, "13");
    }

    #[test]
    fn test_day04_02() {
        let res = part2("inputs/04/demo.txt").unwrap();
        assert_eq!(res, "43");
    }
}
