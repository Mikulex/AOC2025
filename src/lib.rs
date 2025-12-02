mod day01;
mod day02;

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
    }
    if day.eq("02") {
        if matches!(part, Part::FIRST | Part::BOTH) {
            println!("{}", day02::part1(&path).unwrap());
        }

        if matches!(part, Part::SECOND | Part::BOTH) {
            println!("{}", day02::part2(&path).unwrap());
        }
    }
}
