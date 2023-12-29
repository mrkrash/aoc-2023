advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let red = 12;
    let green = 13;
    let blue = 14;
    Some(input.lines().fold(0, |acc: u32, line: &str| {
        let _game = line.split(':').collect::<Vec<&str>>()[0]
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let subsets = line.split(':').collect::<Vec<&str>>()[1]
            .split(';')
            .collect::<Vec<&str>>();

        let mut _subsets = subsets.iter();
        let valid = loop {
            if let Some(subset) = _subsets.next() {
                let _subset = subset.split(',').collect::<Vec<&str>>();
                let mut cubes = _subset.iter();
                let _valid = loop {
                    if let Some(cube) = cubes.next() {
                        let _cube = cube.trim().split(' ').collect::<Vec<&str>>();
                        if _cube.last() == Some(&"red")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > red
                        {
                            break false;
                        }
                        if _cube.last() == Some(&"green")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > green
                        {
                            break false;
                        }
                        if _cube.last() == Some(&"blue")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > blue
                        {
                            break false;
                        }
                    } else {
                        break true;
                    }
                };
                if !_valid {
                    break _valid;
                }
            } else {
                break true;
            }
        };

        if valid {
            return acc + _game;
        }

        acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc: u32, line: &str| {
        let subsets = line.split(':').collect::<Vec<&str>>()[1]
            .split(';')
            .collect::<Vec<&str>>();

        let mut _subsets = subsets.iter();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        loop {
            if let Some(subset) = _subsets.next() {
                let _subset = subset.split(',').collect::<Vec<&str>>();
                let mut cubes = _subset.iter();
                loop {
                    if let Some(cube) = cubes.next() {
                        let _cube = cube.trim().split(' ').collect::<Vec<&str>>();
                        if _cube.last() == Some(&"red")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > red
                        {
                            red = _cube.first().unwrap().parse::<u32>().unwrap();
                        }
                        if _cube.last() == Some(&"green")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > green
                        {
                            green = _cube.first().unwrap().parse::<u32>().unwrap();
                        }
                        if _cube.last() == Some(&"blue")
                            && _cube.first().unwrap().parse::<u32>().unwrap() > blue
                        {
                            blue = _cube.first().unwrap().parse::<u32>().unwrap();
                        }
                    } else {
                        break true;
                    }
                }
            } else {
                break true;
            }
        }

        acc + red * green * blue
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
