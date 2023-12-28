use std::collections::HashMap;
use num::integer::lcm;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = generator(input);
    Some(steps("AAA", &directions, &nodes))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = generator(input);
    Some(nodes
        .iter()
        .filter(|(k,_)| k.substring(2, 1) == "A")
        .map(|(k, _)| steps(k, &directions, &nodes))
        .fold(1, |acc, steps| lcm(acc, steps as u64)))
}

fn generator(input: &str) -> (Vec<u8>, HashMap<&str, (&str,&str)>) {
    let (directions, map) = input.split_once("\n\n").unwrap();
    let nodes = map.lines().fold(HashMap::<&str, (&str, &str)>::new(), |mut acc, line| {
        let (key, line) = line.split_once(" = ").unwrap();
        acc.insert(key,(line.substring(1, 3), line.substring(6, 3)));
        acc
    });
    (
        directions.chars().into_iter().map(|c| if c == 'L' { 0 } else { 1 }).collect::<Vec<u8>>(),
        nodes
    )
}
fn steps(start: &str, directions: &Vec<u8>, nodes: &HashMap<&str, (&str,&str)>) -> u32 {
    let mut step = 1;
    let mut it = directions.iter().cycle();
    let mut dest = nodes.get(&start).unwrap();
    while let Some(i) = it.next() {
        let next = if i == &0 { dest.0 } else { dest.1 };
        let (est, pest) = nodes.get_key_value(next).unwrap();
        if est == &"ZZZ" && start == "AAA" || start != "AAA" && est.substring(2,1) == "Z" {
            break
        }
        dest = pest;
        step += 1;
    }
    step
}
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
}
impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else { break; }
        }
        &self[byte_start..byte_end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
