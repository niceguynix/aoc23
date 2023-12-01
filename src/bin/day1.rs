fn part1(input: &str) -> u32 {
    let mut s = 0;

    for i in input.lines() {
        let mut f = 0;
        let mut l = 0;
        for j in i.chars() {
            match j.to_digit(10) {
                Some(d) => {
                    f = d;
                    break;
                }
                None => continue,
            }
        }
        for j in i.chars().rev() {
            match j.to_digit(10) {
                Some(d) => {
                    l = d;
                    break;
                }
                None => continue,
            }
        }
        let t = f * 10 + l;
        s += t;
    }

    s
}

fn digit_in_letters(string: &str) -> i32 {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (idx, i) in digits.iter().enumerate() {
        let size = std::cmp::min(i.len(), string.len());
        if &string[..size] == *i {
            // println!("comparison    {i} {/}",&string);
            return idx as i32 + 1;
        }
    }
    -1
}

fn part2(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let mut f = 0;
        let mut l = 0;
        for (index, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                Some(d) => {
                    f = d;
                    break;
                }
                None => (),
            }
            match digit_in_letters(&line[index..]) {
                -1 => (),
                x => {
                    f = x as u32;
                    break;
                }
            }
        }

        for (index, c) in line.chars().rev().enumerate() {
            // println!("{}",line.len()-index-1);
            match c.to_digit(10) {
                Some(d) => {
                    l = d;
                    break;
                }
                None => (),
            }
            match digit_in_letters(&line[(line.len() - index - 1)..]) {
                -1 => (),
                x => {
                    l = x as u32;
                    break;
                }
            }
        }
        let t = f * 10 + l;
        s += t;
    }
    s
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(2);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run"),
    };

    let input = include_str!("../inputs/day1/input.txt");
    let result = match part.as_str() {
        "part1" => part1(input),
        "part2" => part2(input),
        _ => panic!("Specify one of 2 parts"),
    };

    println!("The result is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        assert_eq!(part1(include_str!("../inputs/day1/part1_sample.txt")), 142)
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(include_str!("../inputs/day1/part2_sample.txt")), 281)
    }
}
