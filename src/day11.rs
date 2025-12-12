use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

pub fn part1(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let map = get_map(&file);
    let mut cache: HashMap<&str, u64> = HashMap::new();
    let sum = build_path("out", &map.get("out").unwrap(), &map, &mut cache);

    Ok(sum.to_string())
}

pub fn part2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = read_to_string(file_path)?;
    let map = get_map(&file);
    let mut cache: HashMap<(String, bool, bool), u64> = HashMap::new();
    let exit = &map.get("out").unwrap();
    let sum = build_path_2("out", "svr", exit, false, false, &map, &mut cache);

    Ok(sum.to_string())
}

fn get_map<'a>(file: &'a String) -> HashMap<&'a str, HashSet<&'a str>> {
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
    map
}

fn build_path_2<'a>(
    exit: &'a str,
    start: &'a str,
    next: &HashSet<&'a str>,
    seen_dac: bool,
    seen_fft: bool,
    map: &HashMap<&'a str, HashSet<&'a str>>,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let key = (exit.to_string(), seen_dac, seen_fft);

    if let Some(n) = cache.get(&key) {
        *n
    } else if exit == start {
        if seen_dac && seen_fft { 1 } else { 0 }
    } else if next.is_empty() {
        0
    } else {
        let res = next
            .into_iter()
            .map(|n| {
                let seen_dac = if *n == "dac" { true } else { seen_dac };
                let seen_fft = if *n == "fft" { true } else { seen_fft };
                let fallback = HashSet::new();
                let next = map.get(n).unwrap_or(&fallback);
                build_path_2(n, start, next, seen_dac, seen_fft, map, cache)
            })
            .sum();

        cache.insert(key, res);
        res
    }
}

fn build_path<'a>(
    exit: &'a str,
    next: &HashSet<&'a str>,
    map: &HashMap<&str, HashSet<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(n) = cache.get(exit) {
        *n
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_01() {
        let res = part1("inputs/11/demo.txt").unwrap();
        assert_eq!(res.to_string(), "5");
    }

    #[test]
    fn test_day11_02() {
        let res = part2("inputs/11/demo2.txt").unwrap();
        assert_eq!(res.to_string(), "2");
    }
}
