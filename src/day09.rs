use std::{
    cmp::{max, min},
    collections::HashSet,
    error::Error,
    fs::read_to_string,
    io::{Write, stdout},
};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let coords: Vec<(i64, i64)> = file
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect();

    let mut i = 0;
    let pairs: Vec<((i64, i64), (i64, i64), u64)> =
        coords.iter().fold(Vec::new(), |mut pairs, coord| {
            i = i + 1;
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
    Ok(max.2.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let mut coords: Vec<(i64, i64)> = file
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect();

    coords.push(coords[0]);

    let mut boundary: Vec<(i64, i64)> = Vec::new();
    boundary.push(coords.remove(0));

    println!("Building Boundary");
    for coord in &coords {
        let last = boundary.last().unwrap().clone();

        if coord.0 == last.0 {
            for i in min(coord.1, last.1)..max(coord.1, last.1) {
                boundary.push((coord.0, i));
            }
        } else if coord.1 == last.1 {
            for i in min(coord.0, last.0)..max(coord.0, last.0) {
                boundary.push((i, coord.1));
            }
        }
        boundary.push(*coord);
    }

    let boundary: HashSet<(i64, i64)> = HashSet::from_iter(boundary);

    println!("Filling...");
    let filled = flood_fill(&boundary, (97614, 50000));
    println!("Filling Finished");

    let mut i = 0;
    let mut pairs: Vec<((i64, i64), (i64, i64), u64)> =
        coords.iter().fold(Vec::new(), |mut pairs, coord| {
            i = i + 1;
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

    pairs.sort_by_key(|s| s.2);

    let find = pairs.iter().rev().find(|(p1, p2, _)| {
        let (min_x, max_x) = (min(p1.0, p2.0), max(p1.0, p2.0));
        let (min_y, max_y) = (min(p1.1, p2.1), max(p1.1, p2.1));

        let mut res = true;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !filled.contains(&(x, y)) {
                    res = false;
                }
            }
        }
        return res;
    });

    println!("{:?}", find.unwrap());

    Ok(find.unwrap().2.to_string())
}

fn flood_fill(boundary: &HashSet<(i64, i64)>, start: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();

    to_visit.push(start);
    while !to_visit.is_empty() {
        flood_fill_calc(&boundary, &mut to_visit, &mut visited);
    }

    visited.extend(boundary);
    visited
}

fn flood_fill_calc(
    boundary: &HashSet<(i64, i64)>,
    to_visit: &mut Vec<(i64, i64)>,
    visited: &mut HashSet<(i64, i64)>,
) {
    if to_visit.is_empty() {
        return;
    }

    let (c1, c2) = to_visit.pop().unwrap();
    visited.insert((c1, c2));
    for x in -1..=1 {
        for y in -1..=1 {
            let next = (c1 + x, c2 + y);
            if !visited.contains(&next) && !boundary.contains(&next) && !to_visit.contains(&next) {
                to_visit.push(next);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09_01() {
        let res = part1("inputs/09/demo.txt").unwrap();
        assert_eq!(res.to_string(), "50");
    }
    #[test]
    fn test_day09_02() {
        let res = part2("inputs/09/demo.txt").unwrap();
        assert_eq!(res.to_string(), "24");
    }
}
