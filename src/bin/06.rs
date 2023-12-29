use std::str::SplitWhitespace;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, records) = parse_input_one(input);
    let mut j = times.len();
    let mut ways: Vec<u32> = vec![0; j];
    let mut i: u32 = 0;
    j -= 1;
    loop {
        if i * (times[j] - i) > records[j] {
            ways[j] += 1;
        }
        i += 1;
        if i == times[j] {
            if j == 0 {
                break;
            }
            j -= 1;
            i = 0;
        }
    }
    Some(ways.iter().product::<u32>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, record) = parse_input_two(input);
    let mut i: u64 = 0;
    let mut ways: u64 = 0;
    loop {
        if i * (time - i) > record {
            ways += 1
        }
        i += 1;
        if i == time {
            break;
        }
    }
    Some(ways)
}

fn split_line(line: &str) -> SplitWhitespace {
    line.split(':').last().unwrap().split_whitespace()
}
fn parse_input_as_vec(split: SplitWhitespace) -> Vec<u32> {
    split.map(|x| x.parse().unwrap()).collect()
}

fn parse_input_one(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (times, records) = input.split_once('\n').unwrap();
    let times = parse_input_as_vec(split_line(times));
    let records = parse_input_as_vec(split_line(records));
    (times, records)
}
fn parse_input_as_number(split: SplitWhitespace) -> u64 {
    split.collect::<Vec<&str>>().concat().parse().unwrap()
}
fn parse_input_two(input: &str) -> (u64, u64) {
    let (time, record) = input.split_once('\n').unwrap();
    let time = parse_input_as_number(split_line(time));
    let record = parse_input_as_number(split_line(record));
    (time, record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
