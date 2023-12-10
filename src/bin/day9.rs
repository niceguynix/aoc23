fn solve_seq(nums: &[i128]) -> i128 {
    if nums.iter().all(|x| x == &0) {
        return 0;
    }

    let ele = nums.last().unwrap();
    let nums = nums.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    let diff = solve_seq(nums.as_slice());
    diff + ele
}

struct TupleWindows<'a, T>
where
    T: Clone,
{
    iterator: Box<&'a mut dyn Peekable<Item = T>>,
    prev: Option<T>,
    peek: Option<(T, T)>,
}

impl<T> Iterator for TupleWindows<'_, T>
where
    T: Clone + Clone + std::fmt::Debug,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.iterator.next();

        //    println!("{t}");
        if let Some(v) = t {
            if let Some(v2) = self.prev.clone() {
                self.prev = Some(v.clone());
                return Some((v2.clone(), v));
            } else if let Some(x) = self.iterator.next() {
                self.prev = Some(x.clone());
                return Some((v, x));
            } else {
                return None;
            }
        }

        None
    }
}

trait Peekable: Iterator {
    fn peek(&mut self) -> Option<Self::Item>;
}

struct PeekableMap<I, T>
where
    I: Iterator,
{
    iterator: I,
    peek: Option<T>,
}

impl<I, T> Iterator for PeekableMap<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.peek {
            self.peek = None;
            Some(x)
        } else {
            self.iterator.next()
        }
    }
}

impl<I, T> Peekable for PeekableMap<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    fn peek(&mut self) -> Option<Self::Item> {
        match self.peek {
            None => {
                self.peek = self.iterator.next();
                self.peek
            }
            Some(x) => {
                self.peek = None;
                Some(x)
            }
        }
    }
}

impl<'a, I, T> PeekableMap<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    fn new(iterator: I) -> Self {
        Self {
            iterator,
            peek: None,
        }
    }

    fn into_peekable(&'a mut self) -> Box<&'a mut dyn Peekable<Item = T>> {
        Box::new(self)
    }
}

impl<'a, T: Clone> TupleWindows<'a, T> {
    fn new(it: Box<&'a mut dyn Peekable<Item = T>>) -> Self {
        TupleWindows {
            iterator: it,
            prev: None,
            peek: None,
        }
    }
}

fn test_solver(nums: Box<&mut dyn Peekable<Item = i128>>) -> i128 {
    match nums.peek() {
        Some(ele) => {
            // println!("{ele}");
            let test = TupleWindows::new(nums);
            let x = &mut test.map(|x| {
                // println!("{x:?}");
                x.0 - x.1
            });

            let _y = x.peekable();
            let mut a = PeekableMap::new(x);
            let c = a.into_peekable();

            ele + test_solver(c)
        }
        None => 0,
    }
}

fn parse_input(input: &'static str) -> impl Iterator<Item = impl Iterator<Item = i128>> {
    input
        .lines()
        .map(|x| x.split_ascii_whitespace().map(|x| x.parse().unwrap()))
}

fn test(nums: Vec<i128>) {
    println!("{nums:?}");
    let _t = nums.iter();
    // let x = TupleWindows::new(&mut t);

    // for i in x{
    // println!("{i:?}");
    // }
}

fn part1(input: &'static str) -> i128 {
    parse_input(input)
        .map(|x| {
            let t = x.collect::<Vec<_>>();
            // test(t);
            // 0
            let z = t.into_iter().rev();
            let mut c = PeekableMap::new(z);
            test_solver(c.into_peekable())
        })
        .sum()
}

fn part2(input: &'static str) -> i128 {
    parse_input(input)
        .map(|x| {
            let mut t = x.collect::<Vec<_>>();
            t.reverse();
            solve_seq(t.as_slice())
        })
        .sum()
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
        assert_eq!(
            part1(include_str!("../inputs/day9/part1_sample.txt")),
            1707278
        )
    }

    #[test]
    fn day9_part2() {
        // assert_eq!(part2(include_str!("../inputs/day9/part2_sample.txt")), 6)
    }
}
