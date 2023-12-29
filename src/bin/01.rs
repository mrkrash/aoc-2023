advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_once("\n\n")
            .unwrap()
            .0
            .lines()
            .fold(0, |acc, x| {
                let numbers = x
                    .split("")
                    .filter(|y| !y.is_empty() && y.parse::<u32>().is_ok())
                    .collect::<Vec<&str>>();
                if numbers.len() > 2 {
                    return (numbers[0].to_owned() + numbers.last().unwrap())
                        .parse::<u32>()
                        .unwrap()
                        + acc;
                } else if numbers.len() == 1 {
                    return (numbers[0].to_owned() + numbers[0]).parse::<u32>().unwrap() + acc;
                }
                numbers.concat().parse::<u32>().unwrap() + acc
            }),
    )
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit_end(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.ends_with(n) {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn parse_digit_start(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.starts_with(n) {
            return Some(i as u32 + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_once("\n\n")
            .unwrap()
            .1
            .lines()
            .fold(0, |acc, x| {
                let mut first = 0;
                let mut last = 0;

                let mut xx = x.chars();
                loop {
                    if let Some(o) = parse_digit_start(xx.as_str()) {
                        first = o;
                        break;
                    }
                    if let Some(o) = xx.next() {
                        if o.is_ascii_digit() {
                            first = o.to_digit(10).unwrap();
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let mut xx = x.chars();
                loop {
                    if let Some(o) = parse_digit_end(xx.as_str()) {
                        last = o;
                        break;
                    }
                    if let Some(o) = xx.next_back() {
                        if o.is_ascii_digit() {
                            last = o.to_digit(10).unwrap();
                            break;
                        }
                    } else {
                        break;
                    }
                }

                first * 10 + last + acc
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
