advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, x| {
        let numbers = x.split("").filter(|y| !y.is_empty() && y.parse::<u32>().is_ok()).collect::<Vec<&str>>();
        if numbers.iter().count() > 2 {
            return (numbers[0].to_owned() + numbers.last().unwrap()).parse::<u32>().unwrap() + acc;
        } else if numbers.iter().count() == 1 {
            return (numbers[0].to_owned() + numbers[0]).parse::<u32>().unwrap() + acc;
        }
        numbers.concat().parse::<u32>().unwrap() + acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(54968));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
