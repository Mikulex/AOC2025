use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let map: HashMap<&str, HashSet<&str>> = file
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(i, o)| (i, o.trim().split_whitespace().collect::<Vec<&str>>()))
        .fold(HashMap::new(), |res, (i, o)| {
            o.iter().fold(res, |mut acc, o_entry| {
                acc.entry(o_entry).or_insert_with(HashSet::new).insert(&i);
                acc
            })
        });

    let mut cache: HashMap<&str, u64> = HashMap::new();

    let sum = build_path("out", &map.get("out").unwrap(), &map, &mut cache);

    Ok(sum.to_string())
}

fn build_path<'a>(
    exit: &'a str,
    next: &HashSet<&'a str>,
    map: &HashMap<&str, HashSet<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if cache.get(exit).is_some() {
        *cache.get(exit).unwrap()
    } else if exit == "you" {
        1
    } else if next.is_empty() {
        0
    } else {
        let res = next
            .into_iter()
            .map(|n| build_path(n, map.get(n).unwrap_or(&HashSet::new()), map, cache))
            .sum();

        cache.insert(exit, res);
        res
    }
}

pub fn part2(_file_path: &str) -> Result<String, Box<dyn Error>> {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_01() {
        let res = part1("inputs/11/demo.txt").unwrap();
        assert_eq!(res.to_string(), "5");
    }

    fn _test_day11_02() {
        let res = part2("inputs/11/demo.txt").unwrap();
        assert_eq!(res.to_string(), "");
    }
}
