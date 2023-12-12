advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let card = line.split(':').collect::<Vec<&str>>();
        let numbers = card[1].split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split(' ').collect::<Vec<&str>>();
        let my_numbers = numbers[1].split(' ').collect::<Vec<&str>>();

        let mut exp = 0;
        for winning_number in winning_numbers.iter().filter(|x| x != &&"") {
            for my_number in &my_numbers {
                if winning_number == my_number {
                    exp += 1;
                }
            }
        }

        if exp > 0 {
            return acc + u32::pow(2, exp - 1);
        }
        acc
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
        assert_eq!(result, Some(25183));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
