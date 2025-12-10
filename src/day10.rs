use std::{cmp::min, error::Error, fs, usize::MAX};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let res: usize = fs::read_to_string(file_path)?
        .lines()
        .map(|l| l.split(&['[', ']', '(', ')', '{', '}']))
        .map(|l| l.map(|s| String::from(s)))
        .map(|l| l.filter(|s| !s.trim_start().is_empty()))
        .map(|l| l.collect::<Vec<String>>())
        .map(|mut v| {
            let start = v.remove(0);
            let end = v.remove(v.len() - 1);
            (start, v, end)
        })
        .map(|(p, b, c)| (pattern(p), buttons(&b), c))
        .map(|(pattern, buttons, _)| {
            let mut current_min = MAX;
            for but_sel_mask in 0..(1 << buttons.len()) {
                let mut press_count = 0;
                let mut test: Vec<usize> = pattern.clone();
                for b in 0..buttons.len() {
                    if (but_sel_mask >> b) & 1 == 1 {
                        press_count += 1;
                        for d in &buttons[b] {
                            test[*d] = (test[*d] + 1) % 2;
                        }
                    }
                }
                if test.iter().sum::<usize>() == 0 {
                    current_min = min(current_min, press_count);
                }
            }
            current_min
        })
        .sum();

    Ok(res.to_string())
}

fn pattern(a: String) -> Vec<usize> {
    a.chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!(),
        })
        .collect()
}

fn buttons(b: &[String]) -> Vec<Vec<usize>> {
    b.iter()
        .map(|s| s.split(','))
        .map(|v| v.map(|d| d.parse::<usize>().unwrap()))
        .map(|c| c.collect())
        .collect()
}

pub fn part2(_file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_01() {
        let res = part1("inputs/10/demo.txt").unwrap();
        assert_eq!(res.to_string(), "7");
    }
    fn _test_day10_02() {
        todo!()
    }
}
