advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut seeds = resolve_map(parse_seed_one(input), input);
    Some(*seeds.iter().min().unwrap())
}

fn resolve_map(mut seeds: Vec<u64>, input: &str) -> Vec<u64> {
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
    seeds
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

fn parse_seed_one(input: &str) -> Vec<u64> {
    input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap().trim()
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut seeds = parse_seed_two(input);
    let map_relocation = remap(input);
    let mut relocated: Vec<u64> = Vec::new();
    for seed in &mut seeds {
        relocated.push(binary_search(seed.start, seed.end, &map_relocation));
    }

    Some(*relocated.iter().min().unwrap())
}

struct Relocation {
    src: u64,
    dest: u64,
    range: u64
}
struct Seeds {
    start: u64,
    end: u64
}

fn parse_seed_two(input: &str) -> Vec<Seeds> {
    parse_seed_one(input)
        .chunks_exact(2)
        .into_iter()
        .map(|x| Seeds{start: x[0], end: x[0] + x[1] - 1})
        .collect()
}

fn remap(input: &str) -> Vec<Vec<Relocation>> {
    input.split("\n\n").skip(1).collect::<Vec<&str>>().iter().map(|x| {
        x
            .split("\n")
            .skip(1)
            .collect::<Vec<&str>>()
            .iter().filter(|x| !x.is_empty())
            .map(|line| {
                let _line = line
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<u64>().unwrap()
                    })
                    .collect::<Vec<_>>();
                Relocation {
                    src: _line[1],
                    dest: _line[0],
                    range: _line[2]
                }
            })
            .collect::<Vec<Relocation>>()
    }).collect()
}

fn binary_search(start: u64, end: u64, map: &Vec<Vec<Relocation>>) -> u64 {
    let mut high = end;
    let mut low = start;
    let mut location = low;

    while low <= high {
        let mid = low + (high - low) /2;
        let location_seed = relocate(mid, &map);
        if location_seed < location {
            location = location_seed;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    location
}
fn relocate(seed: u64, map: &Vec<Vec<Relocation>>) -> u64 {
    let mut relocation = seed;
    for _step in map {
        for _map in _step {
            if relocation >= _map.src && relocation < (_map.src + _map.range) {
                relocation = _map.dest + relocation - _map.src;
                break
            }
        }

    }
    relocation
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
        assert_eq!(result, Some(46));
    }
}
