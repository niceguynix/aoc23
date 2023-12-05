use core::panic;
use std::ops::{Not, RangeInclusive};

#[derive(Debug)]
struct MapRange {
    source: std::ops::RangeInclusive<u64>,
    destination: std::ops::RangeInclusive<u64>,
}

impl TryFrom<&str> for MapRange {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // println!("{value}");
        let mut value = value.split_whitespace();

        let parseInput = |x: Option<&str>| {
            Ok::<_, Self::Error>(
                x.ok_or(std::io::ErrorKind::NotFound)?
                    .parse::<u64>()
                    .ok()
                    .ok_or(std::io::ErrorKind::InvalidInput)?,
            )
        };
        let destination_start = parseInput(value.next())?;
        let source_start = parseInput(value.next())?;
        let range_length: u64 = parseInput(value.next())?;

        Ok(Self {
            source: (source_start..=(source_start + range_length - 1)),
            destination: (destination_start..=destination_start + range_length - 1),
        })
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<MapRange>,
}

impl TryFrom<&str> for Mapper {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let _map_name = lines.next();
        let ranges: Result<_, _> = lines
            .map(|c| {
                let t = MapRange::try_from(c);
                match t {
                    Ok(maprange) => Ok(maprange),
                    Err(err) => Err::<_, Self::Error>(err),
                }
            })
            .collect();

        let ranges = ranges?;
        Ok(Self { ranges })
    }
}

impl Mapper {
    fn convert_seeds(&self, soils: &mut Vec<u64>) {
        soils
            .iter_mut()
            .map(|x| {
                *x = *self
                    .ranges
                    .iter()
                    .filter_map(|c| {
                        match c.source.contains(x) {
                            true => Some(c.destination.start() + *x - c.source.start()),
                            false => None, // println!("{x}");
                        }
                    })
                    .next()
                    .get_or_insert(*x);
            })
            .count();
    }

    fn convert_single_seed(&self, soil: &mut u64) {
        for x in &self.ranges {
            if x.source.contains(&soil) {
                *soil = x.destination.start() + *soil - x.source.start();
                return;
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut sections = input.split("\r\n\r\n");
    let mut soils = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u64>>();

    // println!("{soils:?}");
    let _t = sections
        .map(|c| {
            let t = Mapper::try_from(c).unwrap();
            t.convert_seeds(&mut soils);
            // println!("{t:?}");
        })
        .collect::<Vec<_>>();
    // println!("{soils:?}");

    *soils.iter().min().unwrap()
}

fn map_range(
    ranges: Vec<(RangeInclusive<u64>, bool)>,
    src: RangeInclusive<u64>,
    dest: RangeInclusive<u64>,
) -> Vec<(RangeInclusive<u64>, bool)> {
    let mut t = Vec::new();

    let mut m = false;
    let mut c = 0_u128;
    for (i, updated) in ranges.clone() {
        m = false;
        if updated {
            t.push((i, true));
            continue;
        }
        if i.start() < src.start() && i.end() > src.end() {
            m = true;
            t.push((*i.start()..=(src.start() - 1), false));
            c += t.last().unwrap().0.clone().count() as u128;
            t.push((dest.clone(), true));
            c += t.last().unwrap().0.clone().count() as u128;
            t.push(((*src.end() + 1)..=*i.end(), false));
            c += t.last().unwrap().0.clone().count() as u128;
        }

        if i.start() == src.start() && i.end() == src.end() {
            t.push((dest.clone(), true));
            c += t.last().unwrap().0.clone().count() as u128;
            m = true;
        }

        if src.start() < i.start() && src.end() > i.end() {
            m = true;
            let tstart = dest.start() + (i.start() - src.start());
            let tend = dest.end() - (src.end() - i.end());
            t.push((tstart..=tend, true));
            c += t.last().unwrap().0.clone().count() as u128;
        }

        if src.end() < i.end() && src.start() <= i.start() && src.end() >= i.start() {
            m = true;
            let tstart = dest.start() + (i.start() - src.start());
            t.push((tstart..=*dest.end(), true));
            c += t.last().unwrap().0.clone().count() as u128;
            t.push(((src.end() + 1)..=*i.end(), false));
            c += t.last().unwrap().0.clone().count() as u128;
        }

        if src.end() >= i.end() && src.start() > i.start() && src.start() <= i.end() {
            m = true;
            let tend = dest.end() - (src.end() - i.end());
            t.push((*i.start()..=*src.start() - 1, false));
            c += t.last().unwrap().0.clone().count() as u128;
            t.push((*dest.start()..=tend, true));
            c += t.last().unwrap().0.clone().count() as u128;
        }
        if m.not() {
            t.push((i, false));
        }
    }
    // println!("\n\n\nHere\n\n\n");
    // println!("{t:?}");
    // println!("Ayo wtf {c} {}",u64::MAX);

    let x=t.clone().iter().fold(0, |p,(c,_)| {
        let t=c.clone().count() as u128;
        // println!("{t}");
        p+t
    });
    if x!=u64::MAX as u128{
        println!("Ayo wtf {ranges:?} {src:?} {c} {}",u64::MAX);
        panic!();
    }

    t.iter()
        .map(|x| {
            if x.0.start() > x.0.end() {
                println!("{:?}", x.0);
                panic!("fuck");
            }

            x.clone()
        })
        .collect()
}

fn part2(input: &str) -> u64 {
    let mut sections = input.split("\r\n\r\n");
    let soils = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u64>>();

    let seeds = soils.chunks_exact(2).flat_map(|x| {
        println!("{x:?}");
        x[0]..x[0] + x[1]
    });

    let mappers: Vec<_> = sections
        .map(Mapper::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    println!("{soils:?}");

    // let seeds = soils.map(|x| {
    //     // let mut t = vec![x];
    //     // mappers.iter().map(|x| x.convert_seeds(&mut t)).count();
    //     // *t.first().unwrap()
    //     let mut t = x;
    //     mappers
    //         .iter()
    //         .map(|m| m.convert_single_seed(&mut t))
    //         .count();
    //     // println!("{x} {t}");
    //     t
    // });

    let mut ranges = vec![(0..=u64::MAX-1, false)];

    let mut c = 0;
    for i in mappers.iter() {
        for z in ranges.iter_mut() {
            z.1 = false;
        }
        println!("hello");
        // println!("{ranges:?}");
        for r in &i.ranges {
            ranges = map_range(ranges, r.source.clone(), r.destination.clone());
            if c == 3 {
                // panic!();
            }
        }
        c += 1;
    }

    // println!("{ranges:?}");

    // let locations = seeds
    //     .map(|x| {
    //         let mut cur = 0;
    //         for (i, _) in &ranges {
    //             let t= i.clone().count() as u64;
    //             if x >= cur && x <= cur + t- 1 {
    //                 let z=i.start() + (x - cur);
    //                 // println!("{z} {x}");
    //                 return z;
    //             }
    //             cur+=t;
    //         }
    //         return 0;
    //     })
    //     ;

    // let mut c = 0;
    // for (i, _) in ranges {
    //     c += i.clone().count() as u128;
    //     // println!("{i:?} {c}");
    // }

    // println!("{locations:?}");
    // let x = ranges.iter().fold(0_u128, |prev,cur|
    //     let t= &cur.0.clone().count();

    //     return prev+ *t as u128;

    // });
    // println!("{ranges:?}");

    let mut ranges=ranges.into_iter().flat_map(|(r,_)|{
        r
    });

    let mut soil2=Vec::new();
    for i in soils.chunks_exact(2){
        soil2.push((i[0],i[1]));
    }
    
    soil2.sort_by_key(|x| x.0);

    let mut c=0;
    let locations=soil2.iter().flat_map(|(s,l)|{
        let mut x=vec![];
        let t=s-c;
        for i in 0..t{
            c+=1;
            ranges.next();
        }

        for i in 0..*l{
            c+=1;
            x.push(ranges.next().unwrap());
        }

        x        
        
    });
    locations.min().unwrap()
    // 0
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day5/input.txt");
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
    fn day5_part1() {
        assert_eq!(part1(include_str!("../inputs/day5/part1_sample.txt")), 35)
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(include_str!("../inputs/day5/part1_sample.txt")), 46)
    }
}
