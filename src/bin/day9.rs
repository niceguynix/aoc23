
use std::collections::binary_heap::Iter;

use itertools::Itertools;

fn solve_seq(nums:&[i128])->i128{
    if nums.iter().all(|x| x==&0){
        return 0;
    }

    let ele=nums.last().unwrap();
    let nums=nums.windows(2).map(|x| x[1]-x[0]).collect::<Vec<_>>();
    let diff=solve_seq(nums.as_slice());
    diff+ele
}

struct TupleWindows<'a,T>
where T:Clone
{
    iterator:&'a mut dyn Iterator<Item = T>,
    prev:Option<T>,
}

impl<T> Iterator for TupleWindows<'_,T>
where T:Clone+Clone
{
    type Item = (T,T);

    fn next(&mut self) -> Option<Self::Item> {
        let t=self.iterator.next();
        let t=self.iterator.next();

        if let Some(v)=t{
            if let Some(v2)=self.prev.clone(){
                    self.prev=Some(v.clone());
                return Some((v2.clone(),v));
            }else{
              let x=self.iterator.next().unwrap();
              let x2=self.iterator.next().unwrap();
              self.prev=Some(x2.clone());  
              return Some((x,x2))
            }
        }

        None

    }
}

impl<'a,T:Clone> TupleWindows<'a,T>{

    fn new( it:&'a mut impl Iterator<Item=T>)->Self{
        TupleWindows { iterator:it , prev: None }
    }
}

fn test_solver(mut nums:&mut impl Iterator<Item=i128>)->i128{
    
    match nums.next(){
        Some(ele)=>{
            let mut test = TupleWindows::new(&mut nums);
            ele+test_solver(&mut test.map(|x| x.1-x.0).into_iter())
        },
        None=>0
    }
}

fn parse_input(input:&'static str)->impl Iterator<Item = impl Iterator<Item=i128>>{
    input.lines().map(|x| x.split_ascii_whitespace().map(|x| x.parse().unwrap()))
}

fn part1(input:&'static str)->i128{
    parse_input(input).map(|x|{
        let t=x.collect::<Vec<_>>();
        test_solver(&mut t.into_iter())
    }).sum()
}

fn part2(input:&'static str)->i128{
    parse_input(input).map(|x|{
        let mut t=x.collect::<Vec<_>>();
        t.reverse();
        solve_seq(t.as_slice())
    }).sum()
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day9/input.txt");
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
    fn day9_part1() {
        assert_eq!(part1(include_str!("../inputs/day9/part1_sample.txt")),1707278 )
    }

    #[test]
    fn day9_part2() {
        // assert_eq!(part2(include_str!("../inputs/day9/part2_sample.txt")), 6)
    }
}
