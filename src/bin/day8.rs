use std::{collections::HashMap, fmt::Debug, ops::Not};

fn gcd(a:u128,b:u128)->u128
{
    if b==0{
        a
    }else if a<b{
        gcd(a,b%a)
    }else{
        gcd(b,a%b)
    }


}

fn lcm(a:u128,b:u128)->u128{
    (a*b)/gcd(a, b)
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    lines.next();

    let mut hash = HashMap::new();

    lines
        .map(|x| {
            let (t1, t2) = x.split_once("=").unwrap();
            let t1 = t1.trim();

            let x = &t2.trim()[1..t2.len() - 2];
            println!("{x}");
            let (t2, t3) = x.split_once(",").unwrap();

            (t1.trim(), t2.trim(), t3.trim())
        })
        .map(|(n, l, r)| hash.insert(n, (l, r)))
        .count();

    let mut cur = "AAA";
    let mut count = 0;

    while cur != "ZZZ" {
        let l = hash.get(cur).unwrap();
        let i = directions
            .chars()
            .nth(count as usize % directions.len())
            .unwrap();
        cur = match i {
            'L' => l.0,
            'R' => l.1,
            _ => unreachable!(),
        };
        count += 1;
    }
    count
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    lines.next();

    let mut hash = HashMap::new();

    lines
        .map(|x| {
            let (t1, t2) = x.split_once("=").unwrap();
            let t1 = t1.trim();

            let x = &t2.trim()[1..t2.len() - 2];
            let (t2, t3) = x.split_once(",").unwrap();

            (t1.trim(), t2.trim(), t3.trim())
        })
        .map(|(n, l, r)| hash.insert(n, (l, r)))
        .count();

    let mut cur = hash.keys().filter(|x| x.ends_with('A')).collect::<Vec<_>>();
    let mut count = 0;

    let mut c_to_z = Vec::new();

    // println!("{:?}", cur);

    let mut mem=HashMap::new();

    for i in cur.iter_mut() {
        let mut dp = 0_u128;
        let orig=i.clone();
        let mut lm=Vec::new();
        while i.ends_with('Z').not() {
            match mem.get(&(**i,dp as usize % directions.len())){
                Some(c)=>{
                    dp+=c;
                    println!("hitttt");
                    break;
                },
                None=>()
            };
            // println!("{}", i);
            let l = hash.get(*i).unwrap();
            let d = directions
                .chars()
                .nth(dp as usize % directions.len())
                .unwrap();
            *i = match d {
                'L' => &l.0,
                'R' => &l.1,
                _ => unreachable!(),
            };
            dp += 1;
            lm.push(((i.clone(),dp as usize%directions.len()),dp));
        }

        lm.into_iter().map(  |(x,c)| mem.insert(x, dp-c)).count() ;
        c_to_z.push(dp);    
    }

    println!("{mem:?}");

    // println!("c to z {:?}",c_to_z);
    let x:u128 = c_to_z.into_iter().reduce( |a, b| { lcm(a, b)}).unwrap();

    x as u64
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day8/input.txt");
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
    fn day8_part1() {
        assert_eq!(part1(include_str!("../inputs/day8/part1_sample.txt")), 6)
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(include_str!("../inputs/day8/part2_sample.txt")), 6)
    }
}
