use std::{
    collections::HashSet,
    error::Error,
    fs::read_to_string,
    io::{Write, stdout},
};
type Coord = (u32, u32, u32);

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let (pairs, mut circuits) = prep(file_path)?;
    println!("finished prep");

    calc(pairs, &mut circuits, 1000);

    circuits.sort_by_key(|c| c.len());
    let res: usize = circuits.iter().rev().map(|c| c.len()).take(3).product();

    Ok(res.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let (mut pairs, mut circuits) = prep(file_path)?;
    println!("finished prep");

    let mut shortest: (Coord, Coord, u64) = ((0, 0, 0), (0, 0, 0), 0);
    while circuits.len() > 1 {
        merge(&mut pairs, &mut circuits, &mut shortest);
    }

    let res = (shortest.0.0 as u64).checked_mul(shortest.1.0 as u64);

    Ok(res.unwrap().to_string())
}

fn calc(
    mut pairs: Vec<(Coord, Coord, u64)>,
    circuits: &mut Vec<Box<HashSet<Coord>>>,
    count: usize,
) {
    for _ in 0..count {
        let mut shortest: (Coord, Coord, u64) = ((0, 0, 0), (0, 0, 0), 0);
        merge(&mut pairs, circuits, &mut shortest);
    }
}

fn merge(
    pairs: &mut Vec<(Coord, Coord, u64)>,
    circuits: &mut Vec<Box<HashSet<Coord>>>,
    shortest: &mut (Coord, Coord, u64),
) {
    *shortest = pairs.remove(0);

    let c1 = circuits
        .iter()
        .position(|c| c.iter().any(|p| p == &shortest.0));
    let c2 = circuits
        .iter()
        .position(|c| c.iter().any(|p| p == &shortest.1));

    if let (Some(mut s1), Some(mut s2)) = (c1, c2) {
        if s1 != s2 {
            if s1 > s2 {
                std::mem::swap(&mut s1, &mut s2);
            }
            let set = *circuits[s2].clone();
            circuits[s1].extend(set.iter());
            circuits.remove(s2);
        }
    } else {
        panic!("No circuit for node")
    }
    println!("Current size: {}", circuits.len());
}

fn prep(
    file_path: &str,
) -> Result<(Vec<(Coord, Coord, u64)>, Vec<Box<HashSet<Coord>>>), Box<dyn Error + 'static>> {
    let file = read_to_string(file_path)?;
    let coords = coords(file);
    let mut pairs = shortest_pairs(&coords);
    pairs.sort_by(|a, b| a.2.cmp(&b.2));
    let circuits: Vec<Box<HashSet<Coord>>> = coords
        .iter()
        .map(|c: &Vec<u32>| {
            let mut set = Box::new(HashSet::new());
            set.insert((c[0], c[1], c[2]));
            set
        })
        .collect();
    Ok((pairs, circuits))
}

fn coords(file: String) -> Vec<Vec<u32>> {
    let coords: Vec<Vec<u32>> = file
        .lines()
        .map(|l| l.split(','))
        .map(|l| l.map(|n| n.parse::<u32>().unwrap()))
        .map(|l| l.collect::<Vec<u32>>())
        .collect();
    coords
}

fn shortest_pairs(coords: &Vec<Vec<u32>>) -> Vec<(Coord, Coord, u64)> {
    println!("size: {}", coords.len());
    let mut i = 0;
    coords.iter().fold(Vec::new(), |mut pairs, curr| {
        i = i + 1;
        let coord = (curr[0], curr[1], curr[2]);
        print!("\rcurr: {:?}", i);
        let _ = stdout().flush();
        for cand in coords.iter() {
            let cand = (cand[0], cand[1], cand[2]);
            if cand == coord {
                continue;
            } else {
                let dist = dist(coord, cand);

                if !pairs.iter().any(|p| p.0 == cand && p.1 == coord) {
                    pairs.push((coord, cand, dist));
                }
            }
        }
        pairs
    })
}

fn dist(curr: Coord, cand: Coord) -> u64 {
    ((curr.0 as i64 - cand.0 as i64).pow(2)
        + (curr.1 as i64 - cand.1 as i64).pow(2)
        + (curr.2 as i64 - cand.2 as i64).pow(2)) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08_01() {
        let (pairs, mut circuits) = prep("inputs/08/demo.txt").unwrap();

        calc(pairs, &mut circuits, 10);

        circuits.sort_by_key(|c| c.len());
        let res: usize = circuits.iter().rev().map(|c| c.len()).take(3).product();

        assert_eq!(res.to_string(), "40");
    }

    #[test]
    fn test_day08_02() {
        let res = part2("inputs/08/demo.txt").unwrap();
        assert_eq!(res, "25272");
    }
}
