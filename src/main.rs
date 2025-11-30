use aoc_2025::{Part, run};

fn main() {
    let mut args = std::env::args();

    if args.len() < 2 {
        panic!("Call with day number and part (optional)");
    }

    args.next();
    let day = args.next().unwrap();
    let part = args.next();

    let part: Part = match part {
        Some(s) if s.eq("1") => Part::FIRST,
        Some(s) if s.eq("2") => Part::SECOND,
        Some(_) => Part::BOTH,
        None => Part::BOTH,
    };

    run(day, part);
}
