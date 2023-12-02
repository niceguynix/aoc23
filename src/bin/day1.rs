fn part1(input: &str) -> u32 {
    let mut s = 0;

    for i in input.lines() {
        let f = i.chars().find_map(|x| x.to_digit(10)).unwrap();
        let l = i.chars().rev().find_map(|x| x.to_digit(10)).unwrap();
        let t = f * 10 + l;
        s += t;
    }

    s
}

fn digit_in_letters(string: &str) -> Option<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (idx, i) in digits.iter().enumerate() {
        if string.starts_with(i) {
            return Some(idx as u32 + 1);
        }
    }
    None
}

fn part2(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let f = line
            .chars()
            .enumerate()
            .find_map(|(idx, c)| c.to_digit(10).or(digit_in_letters(&line[idx..])))
            .unwrap();

        let l = line
            .chars()
            .rev()
            .enumerate()
            .find_map(|(idx, c)| {
                c.to_digit(10)
                    .or(digit_in_letters(&line[(line.len() - 1 - idx)..]))
            })
            .unwrap();
        // println!("{line} {f}{l}");
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
    let before = std::time::Instant::now();
    let result = match part.as_str() {
        "part1" => part1(input),
        "part2" => part2(input),
        _ => panic!("Specify one of 2 parts"),
    };

    println!("The result is {result} duration:{:?}", before.elapsed());
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
