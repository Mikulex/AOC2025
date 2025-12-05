use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub enum Part {
    BOTH,
    FIRST,
    SECOND,
}

type Func = fn(&str) -> Result<String, Box<dyn Error>>;

fn run_day(path: String, part: Part, part1: Func, part2: Func) {
    if matches!(part, Part::FIRST | Part::BOTH) {
        println!("{}", part1(&path).unwrap());
    }

    if matches!(part, Part::SECOND | Part::BOTH) {
        println!("{}", part2(&path).unwrap());
    }
}

pub fn run(day: String, part: Part) {
    let path = format!("inputs/{day}/input.txt");
    if day.eq("01") {
        run_day(path, part, day01::part1, day01::part2);
    } else if day.eq("02") {
        run_day(path, part, day02::part1, day02::part2);
    } else if day.eq("03") {
        run_day(path, part, day03::part1, day03::part2);
    } else if day.eq("04") {
        run_day(path, part, day04::part1, day04::part2);
    } else if day.eq("05") {
        run_day(path, part, day05::part1, day05::part2);
    } else {
        panic!("day not implemented");
    }
}
