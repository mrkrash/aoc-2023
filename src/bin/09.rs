advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().fold(0, |mut acc, line| {
        let seq: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        acc += next_value(seq.clone(), true) + seq.last().unwrap();
        acc
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input.lines().fold(0, |mut acc, line| {
        let seq: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let pre = next_value(seq.clone(), false);
        acc += seq.first().unwrap() - pre;

        acc
    }))
}

fn next_value(seq: Vec<i32>, spin: bool) -> i32 {
    let mut _seq: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < seq.len() - 1 {
        let current = seq[i];
        i += 1;
        let next = seq[i];
        _seq.push(next - current);
    }
    if _seq.iter().min().unwrap() == _seq.iter().max().unwrap() && _seq.last().unwrap() == &0 {
        return 0;
    }
    if spin {
        return next_value(_seq.clone(), spin) + *_seq.last().unwrap();
    } else {
        return *_seq.first().unwrap() - crate::next_value(_seq.clone(), spin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2));
    }
}
