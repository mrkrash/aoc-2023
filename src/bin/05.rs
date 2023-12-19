advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut seeds = input
        .lines()
        .take(1)
        .next()
        .unwrap().trim()
        .split(':')
        .last()
        .unwrap().trim()
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    for seed in &mut seeds {
        let mut gained = false;
        for (_, line) in input.lines().skip(1).into_iter().enumerate() {
            if line.is_empty() || line.contains("map") {
                gained = false;
                continue;
            }
            if !gained {
                let _seed = translate(line.trim(), seed);
                if _seed.is_some() {
                    *seed = _seed.unwrap();
                    gained = true;
                }
            }
        }
    }
    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn translate(line: &str, seed: &u64) -> Option<u64> {
    let param: Vec<u64> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    if seed >= &param[1] && seed < &(param[1] + param[2]) {
        let _aa = param[0] + seed - param[1];
        return Some(param[0] + seed - param[1]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(278755257));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
