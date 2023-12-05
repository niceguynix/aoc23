use std::fmt::Error;

#[derive(Debug)]
struct MapRange {
    source: std::ops::Range<u64>,
    destination: std::ops::Range<u64>,
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
            source: (source_start..(source_start + range_length + 1)),
            destination: (destination_start..destination_start + range_length + 1),
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
        let map_name = lines.next();
        let ranges: Result<_, _> = lines
            .map(|c| {
                let t = MapRange::try_from(c);
                match t {
                    Ok(maprange) => Ok(maprange),
                    Err(err) => return Err::<_, Self::Error>(err.into()),
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
                            true => Some(c.destination.start + *x - c.source.start),
                            false => None, // println!("{x}");
                        }
                    })
                    .next()
                    .get_or_insert(*x);
            })
            .count();
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

    println!("{soils:?}");
    let t = sections
        .map(|c| {
            let t = Mapper::try_from(c).unwrap();
            t.convert_seeds(&mut soils);
            // println!("{t:?}");
        })
        .collect::<Vec<_>>();
    println!("{soils:?}");

    *soils.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
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

    let mut soils= soils
        .chunks_exact(2)
        .flat_map(|x| {
            println!("{x:?}");
            (x[0]..x[0] + x[1]).into_iter()
        });

    let mappers:Vec<_> = sections.map(|x| Mapper::try_from(x)).into_iter().collect::<Result<_,_>>().unwrap();
    println!("{soils:?}");

    let soils = soils.map(|x| {
        let mut t=vec![x];
        // println!("{x}");
        mappers.iter().map(|x| x.convert_seeds(&mut t)).count();
        t.iter().next().unwrap().clone()
    });

    soils.min().unwrap()
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
