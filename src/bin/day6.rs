fn parse_times_and_distances(
    input: &'static str,
) -> (
    impl Iterator<Item = &str>,
    impl Iterator<Item = &'static str>,
) {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace();

    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace();

    (times, distances)
}

fn part1(input: &'static str) -> u32 {
    let (times, distances) = parse_times_and_distances(input);

    let (times, distances) = (
        times.map(|x| x.parse().unwrap()),
        distances.map(|x| x.parse().unwrap()),
    );

    let mut m = 1;
    for (i, ds) in times.zip(distances) {
        let mut c = 0;
        for speed in 0..=i {
            if (i - speed) * speed > ds {
                c += 1;
            }
        }
        m *= c;
    }

    m
}

fn part2(input: &'static str) -> u32 {
    let (time, distance) = parse_times_and_distances(input);

    let (time, distance) = (
        time.collect::<String>().parse::<u64>().unwrap(),
        distance.collect::<String>().parse::<u64>().unwrap(),
    );

    // let mut c = 0;
    // // println!("{times} {distances}");
    // for speed in 0..time {
    //     if (time - speed) * speed > distance {
    //         c += 1;
    //     }
    // }
    let c= time;
    let d=distance;
    
    let intersection1 = (c as f64+ f64::sqrt((c.pow(2) - 4*d) as f64))/2_f64;
    let intersection2 = (c as f64- f64::sqrt((c.pow(2) - 4*d) as f64))/2_f64;

    let c=1_f64-intersection2.ceil() + intersection1.floor();
    println!("{intersection1} {intersection2} {c}");
    c as u32
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
