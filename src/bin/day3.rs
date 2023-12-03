use std::ops::Not;

fn is_symbol(c: &char) -> bool {
    if *c != '.' && c.is_ascii_digit().not() && *c != '\r' {
        return true;
    }
    false
}

fn is_part_number(lines: &Vec<&str>, idx: usize, s: usize, e: usize) -> bool {
    if idx != 0 {
        let line = lines.get(idx - 1).unwrap();
        if s != 0 {
            let c = line.chars().nth(s - 1).unwrap();
            if is_symbol(&c) {
                return true;
            }
        }

        if let Some(c) = line.chars().nth(e + 1) {
            if is_symbol(&c) {
                return true;
            }
        }

        for c_idx in s..=e {
            if is_symbol(&line.chars().nth(c_idx).unwrap()) {
                return true;
            }
        }
    }

    if let Some(line) = lines.get(idx + 1) {
        if s != 0 {
            let c = line.chars().nth(s - 1).unwrap();
            if is_symbol(&c) {
                return true;
            }
        }

        if let Some(c) = line.chars().nth(e + 1) {
            if is_symbol(&c) {
                return true;
            }
        }

        for c_idx in s..=e {
            if is_symbol(&line.chars().nth(c_idx).unwrap()) {
                return true;
            }
        }
    }

    let c_line = lines.get(idx).unwrap();

    if s != 0 {
        if let Some(c) = c_line.chars().nth(s - 1) {
            if is_symbol(&c) {
                return true;
            }
        }
    }

    if let Some(c) = c_line.chars().nth(e + 1) {
        if is_symbol(&c) {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let mut sum = 0;

    for (idx, line) in lines.iter().enumerate() {
        let mut cont_num = false;
        let mut s = 0;
        let mut e = 0;
        for (c_idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if !cont_num {
                    cont_num = true;
                    s = c_idx;
                    e = c_idx;
                } else {
                    e = c_idx;
                }
            } else if cont_num {
                let t1 = &line[s..=e];
                let t = t1.parse::<u32>().unwrap();
                if is_part_number(&lines, idx, s, e) {
                    sum += t;
                }
                cont_num = false;
            }
        }
    }

    sum
}

fn get_no(line: &str, idx: usize) -> &str {
    let mut s = idx;
    let mut e = idx;

    // println!("{line}\n{s} {e}");

    while s > 0 && line.chars().nth(s - 1).unwrap().is_ascii_digit() {
        s -= 1;
    }

    while e < line.len() - 2 && line.chars().nth(e + 1).unwrap().is_ascii_digit() {
        e += 1;
    }

    &line[s..=e]
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    let lines = input.split('\n').collect::<Vec<_>>();
    for (idx, line) in lines.iter().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }
            let mut nos = 0;
            let _prod = 0;

            let mut x = [""; 2];
            if idx != 0 {
                if let Some(line) = lines.get(idx - 1) {
                    if line.chars().nth(c_idx).unwrap().is_ascii_digit() {
                        x[nos] = get_no(line, c_idx);
                        nos += 1;
                    } else {
                        if c_idx > 0 && line.chars().nth(c_idx - 1).unwrap().is_ascii_digit() {
                            x[nos] = get_no(line, c_idx - 1);
                            nos += 1
                        }

                        if c_idx < line.len() - 1
                            && line.chars().nth(c_idx + 1).unwrap().is_ascii_digit()
                        {
                            x[nos] = get_no(line, c_idx + 1);
                            nos += 1
                        }
                    }
                }
            }

            if c_idx != 0 && line.chars().nth(c_idx - 1).unwrap().is_ascii_digit() {
                if nos >= 2 {
                    continue;
                }
                x[nos] = get_no(line, c_idx - 1);
                nos += 1;
            }

            if c_idx < line.len() - 1 && line.chars().nth(c_idx + 1).unwrap().is_ascii_digit() {
                if nos >= 2 {
                    continue;
                }
                x[nos] = get_no(line, c_idx + 1);
                nos += 1;
            }

            if let Some(line) = lines.get(idx + 1) {
                if line.chars().nth(c_idx).unwrap().is_ascii_digit() {
                    if nos >= 2 {
                        continue;
                    }
                    x[nos] = get_no(line, c_idx);
                    nos += 1;
                } else {
                    if c_idx > 0 && line.chars().nth(c_idx - 1).unwrap().is_ascii_digit() {
                        x[nos] = get_no(line, c_idx - 1);
                        nos += 1
                    }

                    if c_idx < line.len() - 1
                        && line.chars().nth(c_idx + 1).unwrap().is_ascii_digit()
                    {
                        x[nos] = get_no(line, c_idx + 1);
                        nos += 1
                    }
                }
            }

            if nos == 2 {
                sum += x[0].parse::<u32>().unwrap() * x[1].parse::<u32>().unwrap();
            }
        }
    }

    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day3/input.txt");
    let before = std::time::Instant::now();
    let result = match part.as_str() {
        "part1" => part1(input),
        "part2" => part2(input),
        _ => panic!("Specify one of 2 parts {part}"),
    };

    println!("The result is {result} duration:{:?}", before.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1() {
        assert_eq!(part1(include_str!("../inputs/day3/part1_sample.txt")), 4361)
    }

    #[test]
    fn day3_part2() {
        assert_eq!(
            part2(include_str!("../inputs/day3/part1_sample.txt")),
            467835
        )
    }
}
