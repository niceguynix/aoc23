use core::time;

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u32>>();

    let mut d = distances.iter();
    let mut m = 1;
    for i in times {
        let ds = d.next().unwrap();
        let mut c = 0;
        for speed in (0..=i) {
            if (i - speed) * speed > *ds {
                c += 1;
            }
        }

        m *= c;
    }

    m
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace().into_iter().collect::<String>().parse::<u64>().unwrap();

    let distances = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace().into_iter().collect::<String>().parse::<u64>().unwrap();
        

    let mut c = 0;
    println!("{times} {distances}");
    for speed in (0..times) {
        if (times - speed) * speed > distances {
            c += 1;
        }
    }

    c
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day6/input.txt");
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
    fn day6_part1() {
        assert_eq!(part1(include_str!("../inputs/day6/part1_sample.txt")), 288)
    }

    #[test]
    fn day6_part2() {
        assert_eq!(
            part2(include_str!("../inputs/day6/part1_sample.txt")),
            71503
        )
    }
}
