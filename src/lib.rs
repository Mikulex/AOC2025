mod day01;
mod day02;
mod day03;

pub enum Part {
    BOTH,
    FIRST,
    SECOND,
}

pub fn run(day: String, part: Part) {
    let path = format!("inputs/{day}/input.txt");
    if day.eq("01") {
        if matches!(part, Part::FIRST | Part::BOTH) {
            println!("{}", day01::part1(&path).unwrap());
        }

        if matches!(part, Part::SECOND | Part::BOTH) {
            println!("{}", day01::part2(&path).unwrap());
        }
    } else if day.eq("02") {
        if matches!(part, Part::FIRST | Part::BOTH) {
            println!("{}", day02::part1(&path).unwrap());
        }

        if matches!(part, Part::SECOND | Part::BOTH) {
            println!("{}", day02::part2(&path).unwrap());
        }
    } else if day.eq("03") {
        if matches!(part, Part::FIRST | Part::BOTH) {
            println!("{}", day03::part1(&path).unwrap());
        }

        if matches!(part, Part::SECOND | Part::BOTH) {
            println!("{}", day03::part2(&path).unwrap());
        }
    } else {
        panic!("day not implemented");
    }
}
