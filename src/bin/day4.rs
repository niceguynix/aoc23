use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for i in input.split('\n') {
        let cards_1 = i.split_once(':').unwrap().1;
        let (winning_cards, own_cards) = cards_1.split_once('|').unwrap();
        let own_cards = own_cards
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<HashSet<_>>();
        let winning_cards = winning_cards
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<HashSet<_>>();

        let no_of_winning_cards = own_cards.intersection(&winning_cards).count();

        if no_of_winning_cards == 0 {
            continue;
        }
        sum += 2_u32.pow(no_of_winning_cards as u32 - 1);
    }

    sum
}

fn part2(input: &str) -> u32 {
    let line_count = input.split('\n').count();
    let mut arr: Vec<u32> = Vec::with_capacity(line_count);
    for _i in 0..line_count {
        arr.push(1);
    }
    for (idx, i) in input.split('\n').enumerate() {
        let cards_1 = i.split_once(':').unwrap().1;
        let (winning_cards, own_cards) = cards_1.split_once('|').unwrap();
        let own_cards = own_cards
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<HashSet<_>>();
        let winning_cards = winning_cards
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<HashSet<_>>();

        let no_of_won_cards = own_cards.intersection(&winning_cards).count();
        for i in idx + 1..=idx + no_of_won_cards {
            arr[i] += arr[idx];
        }
    }
    arr.iter().sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day4/input.txt");
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
    fn day4_part1() {
        assert_eq!(part1(include_str!("../inputs/day4/part1_sample.txt")), 13)
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(include_str!("../inputs/day4/part1_sample.txt")), 30)
    }
}
