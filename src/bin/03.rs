use std::collections::HashMap;
advent_of_code::solution!(3);

type Pos = (usize, usize);
type Digits = HashMap<Pos, char>;
type Symbols = Vec<(usize, usize, char)>;
struct Sectors {
    sec1: Option<String>,
    sec2: Option<String>,
    sec3: Option<String>,
    sec4: Option<String>,
    sec6: Option<String>,
    sec7: Option<String>,
    sec8: Option<String>,
    sec9: Option<String>,
}

impl Sectors {
    fn new() -> Self {
        // sec1: None, sec2: None, sec3: None,
        // sec4: None,             sec6: None,
        // sec7: None, sec8: None, sec9: None,
        Sectors {
            sec1: None,
            sec2: None,
            sec3: None,
            sec4: None,
            sec6: None,
            sec7: None,
            sec8: None,
            sec9: None,
        }
    }
}

fn input_generator(input: &str) -> (Digits, Symbols) {
    let mut digits = HashMap::new();
    let mut symbols = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                digits.insert((col, row), ch);
            } else if ch != '.' {
                symbols.push((col, row, ch));
            }
        }
    }

    (digits, symbols)
}

fn read_number(pos: Pos, digits: &Digits) -> Option<Vec<u32>> {
    let mut sectors = Sectors::new();
    let mut parts: Vec<u32> = Vec::new();
    for col in 0..3usize {
        for row in 0..3usize {
            if (row, col) == (1, 1) {
                continue;
            }
            if digits.contains_key(&(col + pos.0 - 1, row + pos.1 - 1)) {
                let mut val = vec![digits.get(&(col + pos.0 - 1, row + pos.1 - 1)).unwrap()];
                let mut n = 1;
                if col == 0 {
                    while pos.0 > n && digits.contains_key(&(pos.0 - 1 - n, row + pos.1 - 1)) {
                        val.push(digits.get(&(pos.0 - 1 - n, row + pos.1 - 1)).unwrap());
                        n += 1;
                    }
                    let val = val.into_iter().rev().collect();
                    if row == 0 {
                        sectors.sec1 = Some(val);
                    } else if row == 1 {
                        sectors.sec4 = Some(val);
                    } else {
                        sectors.sec7 = Some(val);
                    }
                } else if col == 2 {
                    while digits.contains_key(&(pos.0 + 1 + n, row + pos.1 - 1)) {
                        val.push(digits.get(&(pos.0 + 1 + n, row + pos.1 - 1)).unwrap());
                        n += 1;
                    }
                    let val = val.into_iter().collect();
                    if row == 0 {
                        sectors.sec3 = Some(val);
                    } else if row == 1 {
                        sectors.sec6 = Some(val);
                    } else {
                        sectors.sec9 = Some(val);
                    }
                } else {
                    let val = val.into_iter().collect();
                    if row == 0 {
                        sectors.sec2 = Some(val);
                    } else {
                        sectors.sec8 = Some(val);
                    }
                }
            }
        }
    }

    if sectors.sec2.is_none() {
        if sectors.sec1.is_some() {
            parts.push(sectors.sec1.unwrap().parse::<u32>().unwrap());
        }
        if sectors.sec3.is_some() {
            parts.push(sectors.sec3.unwrap().parse::<u32>().unwrap());
        }
    } else {
        let mut cen = sectors.sec2.unwrap();
        if sectors.sec1.is_some() {
            cen = sectors.sec1.unwrap() + &cen;
        }
        if sectors.sec3.is_some() {
            cen = cen + &sectors.sec3.unwrap();
        }
        parts.push(cen.parse::<u32>().unwrap())
    }
    if sectors.sec4.is_some() {
        parts.push(sectors.sec4.unwrap().parse::<u32>().unwrap());
    }
    if sectors.sec6.is_some() {
        parts.push(sectors.sec6.unwrap().parse::<u32>().unwrap());
    }
    if sectors.sec8.is_none() {
        if sectors.sec7.is_some() {
            parts.push(sectors.sec7.unwrap().parse::<u32>().unwrap());
        }
        if sectors.sec9.is_some() {
            parts.push(sectors.sec9.unwrap().parse::<u32>().unwrap());
        }
    } else {
        let mut cen = sectors.sec8.unwrap();
        if sectors.sec7.is_some() {
            cen = sectors.sec7.unwrap() + &cen;
        }
        if sectors.sec9.is_some() {
            cen = cen + &sectors.sec9.unwrap();
        }
        parts.push(cen.parse::<u32>().unwrap())
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let inpt = input_generator(input);
    let digits = &inpt.0;
    let symbols = &inpt.1;
    let mut sum: u32 = 0;

    for symbol in symbols.iter() {
        if let Some(parts) = read_number((symbol.0, symbol.1), digits) {
            sum += parts.iter().sum::<u32>();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let inpt = input_generator(input);
    let digits = &inpt.0;
    let symbols = &inpt.1;
    let mut sum: u32 = 0;

    for symbol in symbols.iter().filter(|(_, _, s)| *s == '*') {
        if let Some(parts) = read_number((symbol.0, symbol.1), digits) {
            if parts.len() == 2 {
                sum += parts[0] * parts[1];
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
