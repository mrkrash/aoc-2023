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

type Cards = Vec<u32>;
pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Cards = Vec::new();
    cards.push(0);
    let count = input.lines().count();
    for line in input.lines() {
        let card = line.split(':').collect::<Vec<&str>>();
        let card_number: usize = card[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let numbers = card[1].split('|').collect::<Vec<&str>>();
        let winning_numbers: Vec<usize> = numbers[0]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let my_numbers: Vec<usize> = numbers[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut exp: usize = 1;
        if cards.get(card_number).is_none() {
            cards.push(1);
        }
        for winning_number in winning_numbers {
            for my_number in &my_numbers {
                if &winning_number == my_number {
                    let multiple = cards[card_number];
                    if cards.get(card_number + exp).is_some() {
                        cards[card_number + exp] += multiple;
                    } else if (card_number + exp) <= count {
                        cards.push(multiple + 1);
                    }
                    exp += 1;
                }
            }
        }
    }

    Some(cards.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
