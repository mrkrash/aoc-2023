advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = resolve_map(parse_seed_one(input), input);
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
    for maps in map_relocation {
        let mut new_seeds:Vec<Seeds> = Vec::new();
        for relocation in maps {
            for seed_range in &mut seeds {
                if relocation.end >= seed_range.start && relocation.end < seed_range.end { // Right
                    new_seeds.push(Seeds {start: relocation.end +1, end: seed_range.end, relocated: false});
                    *seed_range = Seeds {start: seed_range.start, end: relocation.end, relocated:false};
                }
                if relocation.from > seed_range.start && relocation.from <= seed_range.end { // Left
                    new_seeds.push(Seeds {start: seed_range.start, end: relocation.from -1, relocated: false});
                    *seed_range = Seeds {start: relocation.from, end: seed_range.end, relocated: false};
                }
                if seed_range.start >= relocation.from && seed_range.end <= relocation.end && !seed_range.relocated {
                    let start = seed_range.start - relocation.from + relocation.dest;
                    let end = seed_range.end - relocation.from + relocation.dest;

                    *seed_range = Seeds {start, end, relocated: true};
                }
            }
        }
        for seed in &mut seeds {
            *seed = Seeds {start: seed.start, end: seed.end, relocated: false};
        }
        seeds = [seeds, new_seeds].concat();
    }

    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    Some(seeds.first().unwrap().start)
}

struct Relocation {
    from: u64,
    end: u64,
    dest: u64
}
#[derive(Clone)]
struct Seeds {
    start: u64,
    end: u64,
    relocated: bool
}

fn parse_seed_two(input: &str) -> Vec<Seeds> {
    parse_seed_one(input)
        .chunks_exact(2)
        .into_iter()
        .map(|x| Seeds{start: x[0], end: x[0] + x[1] -1, relocated: false})
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
                    from: _line[1],
                    dest: _line[0],
                    end: _line[1] + _line[2] -1
                }
            })
            .collect::<Vec<Relocation>>()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
        //assert_eq!(result, Some(278755257));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
        //assert_eq!(result, Some(26829166));
    }
}
