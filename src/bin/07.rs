use std::collections::HashMap;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let _line = line.split_whitespace().collect::<Vec<&str>>();
        let _cards = _line.first().unwrap();
        let bid: u32 = _line.last().unwrap().parse().unwrap();
        let _cards: Vec<Card> = _cards
            .chars()
            .map(|c| Card::from_char(c).unwrap())
            .collect();
        hands.push(Hand::from_cards(_cards, bid));
    }
    hands.sort_by(|a, b| {
        if a.rank == b.rank {
            if a.cards.get(0) == b.cards.get(0) {
                if a.cards.get(1) == b.cards.get(1) {
                    if a.cards.get(2) == b.cards.get(2) {
                        if a.cards.get(3) == b.cards.get(3) {
                            return a
                                .cards
                                .get(4)
                                .unwrap()
                                .partial_cmp(b.cards.get(4).unwrap())
                                .unwrap();
                        }
                        return a
                            .cards
                            .get(3)
                            .unwrap()
                            .partial_cmp(b.cards.get(3).unwrap())
                            .unwrap();
                    }
                    return a
                        .cards
                        .get(2)
                        .unwrap()
                        .partial_cmp(b.cards.get(2).unwrap())
                        .unwrap();
                }
                return a
                    .cards
                    .get(1)
                    .unwrap()
                    .partial_cmp(b.cards.get(1).unwrap())
                    .unwrap();
            }
            return a
                .cards
                .get(0)
                .unwrap()
                .partial_cmp(b.cards.get(0).unwrap())
                .unwrap();
        }
        b.rank.cmp(&a.rank)
    });
    let mut i = 0;
    Some(hands.iter().fold(0, |mut acc, x| {
        i += 1;
        acc += i * x.bid;
        acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<JokerHand> = Vec::new();
    for line in input.lines() {
        let _line = line.split_whitespace().collect::<Vec<&str>>();
        let _cards = _line.first().unwrap();
        let bid: u32 = _line.last().unwrap().parse().unwrap();
        let _cards: Vec<JokerCard> = _cards
            .chars()
            .map(|c| JokerCard::from_char(c).unwrap())
            .collect();
        hands.push(JokerHand::from_cards(_cards, bid));
    }
    hands.sort_by(|a, b| {
        if a.rank == b.rank {
            if a.cards.get(0) == b.cards.get(0) {
                if a.cards.get(1) == b.cards.get(1) {
                    if a.cards.get(2) == b.cards.get(2) {
                        if a.cards.get(3) == b.cards.get(3) {
                            return a
                                .cards
                                .get(4)
                                .unwrap()
                                .partial_cmp(b.cards.get(4).unwrap())
                                .unwrap();
                        }
                        return a
                            .cards
                            .get(3)
                            .unwrap()
                            .partial_cmp(b.cards.get(3).unwrap())
                            .unwrap();
                    }
                    return a
                        .cards
                        .get(2)
                        .unwrap()
                        .partial_cmp(b.cards.get(2).unwrap())
                        .unwrap();
                }
                return a
                    .cards
                    .get(1)
                    .unwrap()
                    .partial_cmp(b.cards.get(1).unwrap())
                    .unwrap();
            }
            return a
                .cards
                .get(0)
                .unwrap()
                .partial_cmp(b.cards.get(0).unwrap())
                .unwrap();
        }
        b.rank.cmp(&a.rank)
    });
    let mut i = 0;
    Some(hands.iter().fold(0, |mut acc, x| {
        i += 1;
        acc += i * x.bid;
        acc
    }))
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone, Copy)]
struct Card {
    value: u8,
}
impl Card {
    fn from_char(c: char) -> Result<Self, &'static str> {
        let value = match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            n => n.to_digit(10).unwrap() as u8 - 2,
        };
        Ok(Card { value })
    }
}
#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone, Copy)]
struct JokerCard {
    value: u8,
}
impl crate::JokerCard {
    fn from_char(c: char) -> Result<Self, &'static str> {
        let value = match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 0,
            'T' => 9,
            n => n.to_digit(10).unwrap() as u8 - 1,
        };
        Ok(crate::JokerCard { value })
    }
}
struct Hand {
    cards: [Card; 5],
    rank: u32,
    bid: u32,
}
impl Hand {
    fn from_cards(cards: Vec<Card>, bid: u32) -> Hand {
        let mut same_card_counts: Vec<u8> = cards
            .clone()
            .into_iter()
            .fold(HashMap::<Card, u8>::new(), |mut acc, c| {
                acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
                acc
            })
            .values()
            .copied()
            .collect::<Vec<_>>();
        same_card_counts.sort();
        same_card_counts.reverse();
        Hand {
            cards: <[Card; 5]>::try_from(cards).unwrap(),
            rank: match same_card_counts.len() {
                1 => 0,
                _ => match (same_card_counts[0], same_card_counts[1]) {
                    (4, _) => 1,
                    (3, 2) => 2,
                    (3, _) => 3,
                    (2, 2) => 4,
                    (2, 1) => 5,
                    _ => 6,
                },
            },
            bid,
        }
    }
}
struct JokerHand {
    cards: [JokerCard; 5],
    rank: u32,
    bid: u32,
}
impl JokerHand {
    fn from_cards(cards: Vec<JokerCard>, bid: u32) -> JokerHand {
        let mut same_card_counts: Vec<u8> = cards
            .clone()
            .into_iter()
            .fold(HashMap::<JokerCard, u8>::new(), |mut acc, c| {
                acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
                acc
            })
            .values()
            .copied()
            .collect::<Vec<_>>();
        same_card_counts.sort();
        same_card_counts.reverse();
        let joker_count = cards.clone().into_iter().filter(|x| x.value == 0).count();
        JokerHand {
            cards: <[JokerCard; 5]>::try_from(cards).unwrap(),
            rank: match joker_count {
                0 | 5 => match same_card_counts.len() {
                    1 => 0,
                    _ => match (same_card_counts[0], same_card_counts[1]) {
                        (4, _) => 1,
                        (3, 2) => 2,
                        (3, _) => 3,
                        (2, 2) => 4,
                        (2, 1) => 5,
                        _ => 6,
                    },
                },
                // 1 joker, 4 others
                1 => match (same_card_counts[0], same_card_counts[1]) {
                    (4, _) => 0,
                    (3, _) => 1,
                    (2, 2) => 2,
                    (2, _) => 3,
                    (1, _) => 5,
                    _ => unreachable!(),
                },
                // 2 joker, 3 other
                2 => match (same_card_counts[0], same_card_counts[1]) {
                    (3, _) => 0,
                    (2, 2) => 1,
                    (2, _) => 3,
                    _ => unreachable!(),
                },
                // 3 joker, 2 other
                3 => match same_card_counts.len() {
                    2 => 0,
                    3 => 1,
                    _ => unreachable!(),
                },
                4 => 0,
                _ => unreachable!(),
            },
            bid,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
