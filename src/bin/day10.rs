use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Not,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn connects(&self, d: Direction) -> bool {
        if self.0 == d || self.1 == d {
            return true;
        }

        false
    }

    fn is_corner_piece(&self) -> bool {
        use Direction::*;
        match (self.0, self.1) {
            (North, South) => false,
            (South, North) => false,
            (East, West) => false,
            (West, East) => false,
            _ => true,
        }
    }

    fn progressable_dir(&self) -> (Direction, Direction) {
        let x = match self.connects(Direction::North) {
            true => Direction::North,
            false => Direction::South,
        };

        let y = match self.connects(Direction::East) {
            true => Direction::East,
            false => Direction::West,
        };
        (x, y)
    }

    fn update_momentum(&self, m: Direction) -> Direction {
        use Direction::*;
        let n = self.connects(North);
        let s = self.connects(South);
        let e = self.connects(East);
        let w = self.connects(West);

        if let East = m {
            if n {
                North
            } else {
                South
            }
        } else if let South = m {
            if w {
                West
            } else {
                East
            }
        } else if let North = m {
            if e {
                East
            } else {
                West
            }
        } else {
            if n {
                North
            } else {
                South
            }
        }
    }

    fn get_reflexive_dir(&self, m: (Direction, Direction)) -> (Direction, Direction) {
        use Direction::*;
        let mut x = m.0;
        let mut y = m.1;

        // println!("still inside {x:?} {y:?}");
        if let (North, East) = (m.0, m.1) {
            if self.connects(North) {
                y = West;
            }
            if self.connects(East) {
                x = South;
            }
        }

        if let (South, East) = (m.0, m.1) {
            if self.connects(South) {
                y = West;
            }
            if self.connects(East) {
                x = North;
            }
        }

        if let (North, West) = (m.0, m.1) {
            if self.connects(North) {
                y = East;
            }
            if self.connects(West) {
                x = South;
            }
        }

        if let (South, West) = (m.0, m.1) {
            if self.connects(South) {
                y = East;
            }
            if self.connects(West) {
                x = North;
            }
        }

        (x, y)
    }

    fn get_new_reflexive_dir(&self, m: Direction) -> Direction {
        use Direction::*;
        let n = self.connects(North);
        let s = self.connects(South);
        let e = self.connects(East);
        let w = self.connects(West);

        if let North = m {
            if (n && w) || (s && e) {
                return West;
            } else {
                return East;
            }
        }

        if let South = m {
            if (n && e) || (s && w) {
                return West;
            } else {
                return East;
            }
        }

        if let East = m {
            if (e && n) || (w && s) {
                return North;
            } else {
                return South;
            }
        }

        if let West = m {
            if (s && w) || (e && n) {
                return South;
            } else {
                return North;
            }
        }

        panic!()
    }

    fn get_start_dir(&self) -> Direction {
        if self.connects(Direction::North) {
            match self.connects(Direction::West) {
                true => Direction::North,
                false => Direction::East,
            }
        } else {
            if self.connects(Direction::West) {
                Direction::West
            } else {
                Direction::South
            }
        }
    }

    fn get_start_pipe(st: (usize, usize), a: (usize, usize), b: (usize, usize)) -> Self {
        use Direction::*;

        let x = match st.0 as i8 - a.0 as i8 {
            1 => North,
            -1 => South,
            _ => match st.0 as i8 - b.0 as i8 {
                1 => North,
                -1 => South,
                _ => panic!(),
            },
        };

        let y = match st.1 as i8 - a.1 as i8 {
            1 => West,
            -1 => East,
            _ => match st.1 as i8 - b.1 as i8 {
                1 => West,
                -1 => East,
                _ => panic!(),
            },
        };
        Pipe(x, y)
    }
}

impl TryFrom<char> for Pipe {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        let p = match value {
            '|' => Pipe(North, South),
            '-' => Pipe(East, West),
            'L' => Pipe(North, East),
            'J' => Pipe(North, West),
            '7' => Pipe(South, West),
            'F' => Pipe(South, East),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Not a valid pipe: {}", value),
                ))
            }
        };
        Ok(p)
    }
}
#[derive(Debug, Clone, Copy)]
enum Field {
    Ground,
    Start,
    Pipe(Pipe),
}

impl Field {
    fn connects(&self, d: Direction) -> bool {
        if let Field::Pipe(p) = self {
            return p.connects(d);
        }
        false
    }
}

impl TryFrom<char> for Field {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let f = match value {
            '.' => Field::Ground,
            'S' => Field::Start,
            s => Field::Pipe(Pipe::try_from(value)?),
        };

        Ok(f)
    }
}
struct Grid {
    start_pos: (usize, usize),
    field: Vec<Vec<Field>>,
    loop_pts: HashSet<(usize, usize)>,
}

impl Grid {
    fn get_connecting_pipes(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let x = point.0;
        let y = point.1;
        let mut t = Vec::new();
        if x != 0 {
            let row = self.field.get(x - 1).unwrap();
            let p = row.get(y).unwrap();
            if p.connects(Direction::South) {
                t.push((x - 1, y));
            }
        }
        if y != 0 {
            let row = self.field.get(x).unwrap();
            let p = row.get(y - 1).unwrap();
            if p.connects(Direction::East) {
                t.push((x, y - 1));
            }
        }

        if x < self.field.len() - 1 {
            let row = self.field.get(x + 1).unwrap();
            let p = row.get(y).unwrap();
            if p.connects(Direction::North) {
                t.push((x + 1, y));
            }
        }
        if y < self.field.get(1).unwrap().len() - 1 {
            let row = self.field.get(x).unwrap();
            let p = row.get(y + 1).unwrap();
            if p.connects(Direction::West) {
                t.push((x, y + 1));
            }
        }

        t
    }

    fn detect_loops() {
        todo!()
    }

    fn get_farthest_pos(&self) -> u32 {
        let mut hash = HashSet::new();
        let mut queue = VecDeque::new();

        queue.extend(self.get_connecting_pipes(self.start_pos));
        let mut max = 0;

        loop {
            let mut t = VecDeque::new();

            while let Some(point) = queue.pop_front() {
                if hash.contains(&point) {
                    continue;
                }
                hash.insert(point);
                t.extend(self.get_connecting_pipes(point));
            }
            queue = t;
            if queue.len() == 0 {
                break;
            }
            max += 1;
        }

        max
    }

    fn get_field_in_dir(&self, cur_pt: (usize, usize), d: &Direction) -> Option<&Field> {
        let (x, y): (i32, i32) = (cur_pt.0 as i32, cur_pt.1 as i32);

        let (x, y) = match d {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        };

        if x < 0 || y < 0 {
            return None;
        }

        self.field.get(x as usize)?.get(y as usize)
    }

    fn get_r_field(&self, cur_pt: (usize, usize), d: &Direction) -> (usize, usize) {
        let (x, y) = (cur_pt.0, cur_pt.1);
        match d {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        }
    }

    fn is_not_in_loop(&self, pt: (usize, usize)) -> bool {
        self.loop_pts.contains(&pt).not()
    }

    fn save_loop_pts(&mut self) {
        let x = self.start_pos;
        let t = self.get_connecting_pipes(x);

        let x = Pipe::get_start_pipe(x, t[0], t[1]);
        let mut m = x.0;
        println!("{x:?} {m:?}");

        let mut t = self.get_r_field(self.start_pos, &m);

        self.loop_pts.insert(self.start_pos);

        let t1 = self.field.get(t.0).unwrap().get(t.1).unwrap();
        if let Field::Pipe(p) = t1 {
            if p.is_corner_piece() {
                m = p.update_momentum(m);
            }
        }

        while t != self.start_pos {
            self.loop_pts.insert(t);

            t = self.get_r_field(t, &m);
            println!("Loop calculation {t:?} {m:?}");
            let t1 = self.field.get(t.0).unwrap().get(t.1).unwrap();
            if let Field::Pipe(p) = t1 {
                if p.is_corner_piece() {
                    m = p.update_momentum(m);
                }
            }
        }
    }

    fn log(&self, pt: (usize, usize)) {
        let mut x: (usize, usize) = (0, 0);
        let mut min = i32::MAX;
        for i in &self.loop_pts {
            let t = pt.0 as i32 - i.0 as i32;
            let t2 = pt.1 as i32 - i.1 as i32;
            let t3 = t.abs() + t2.abs();

            if t3 < min {
                min = t3;
                x = *i;
            }
        }
        println!("pt:{pt:?} min:{min} loop:{x:?}");
    }

    fn raycast(&self,mut x:usize,mut y:usize)->bool{
        let mut crossings=0;
        let debug=x==3 && y==5;
        while y>0{
            y-=1;
            if debug{
                println!("x{x} y{y} cross:{crossings}");
            } 
            let t=self.field[x][y];
            if !self.loop_pts.contains(&(x,y)){continue;}
            if let Field::Pipe(p)=t{
                use Direction::*;
                if p.connects(North){
                    crossings+=1;
                } 
            }
            if let Field::Start=t{
                crossings+=1;
            }
        }
        crossings%2==1
    }

    fn loop_maze(&self) -> i32 {
        println!("loop len {:?}",self.loop_pts);
        for (x,i) in self.field.iter().enumerate(){
            for (y,j) in i.iter().enumerate(){
                if self.loop_pts.contains(&(x,y)){
                    print!("X",);
                }else{
                    print!(".");
                }
            }
            println!("");
        }
        let mut sum=0;
        for (x,i) in self.field.iter().enumerate(){
            for (y,c) in i.iter().enumerate(){
                if !self.loop_pts.contains(&(x,y)){
                    if self.raycast(x, y){
                        println!("x{x} y{y} true");
                        sum+=1;
                    }
                }
            }
        }
        sum
    }

    fn new_flood_fill(
        &self,
        cur_pt: (usize, usize),
        hash: &mut HashSet<(usize, usize)>,
        emit_dir: Direction,
    ) -> i32 {
        println!("ff c:{cur_pt:?}");
        if hash.contains(&cur_pt)
            || self.loop_pts.contains(&cur_pt)
            || !(0..self.field.len()).contains(&cur_pt.0)
            || !(0..self.field[0].len()).contains(&cur_pt.1)
        {
            return 0;
        }
        hash.insert(cur_pt);
        let mut sum = 0;
        sum += self.new_flood_fill(self.get_r_field(cur_pt, &emit_dir), hash, emit_dir);
        sum+1
    }

    fn flood_fill(
        &mut self,
        cur_pt: (usize, usize),
        hash: &mut HashSet<(usize, usize)>,
        x: Direction,
    ) -> i64 {
        if hash.contains(&cur_pt) {
            return 0;
        }

        hash.insert(cur_pt);
        let mut sum = 0;
        println!("temp {cur_pt:?}");
        let f = self.field.get(cur_pt.0).unwrap().get(cur_pt.1).unwrap();
        println!("{cur_pt:?} d:{x:?} f:{f:?} len:{}", hash.len());
        if self.is_not_in_loop(cur_pt) {
            let z = x;
            println!("Woah there {cur_pt:?}");
            // self.log(cur_pt);
            if let Some(t) = self.get_field_in_dir(cur_pt, &z) {
                let t = self.get_r_field(cur_pt, &z);
                if self.is_not_in_loop(t) {
                    // println!("Debug {t:?}");
                    sum += self.flood_fill(t, hash, x);
                }
            }

            // if let Some(t) = self.get_field_in_dir(cur_pt, &Direction::West) {
            // let t = self.get_r_field(cur_pt, &Direction::West);
            // if self.is_not_in_loop(t) {
            // sum += self.flood_fill(t, hash, x);
            // }
            // }

            // if let Some(t) = self.get_field_in_dir(cur_pt, &Direction::North) {
            // let t = self.get_r_field(cur_pt, &Direction::North);
            // if self.is_not_in_loop(t) {
            // sum += self.flood_fill(t, hash, x);
            // }
            // }

            // if let Some(t) = self.get_field_in_dir(cur_pt, &Direction::South) {
            // let t = self.get_r_field(cur_pt, &Direction::South);
            // if self.is_not_in_loop(t) {
            // sum += self.flood_fill(t, hash, x);
            // }
            // }
        } else {
            let mut t = x;
            let ct = self
                .field
                .get(cur_pt.0)
                .unwrap()
                .get(cur_pt.1)
                .unwrap()
                .clone();
            if let Some(c) = self.get_field_in_dir(cur_pt, &t) {
                let z = self.get_r_field(cur_pt, &t);
                if self.is_not_in_loop(z) {
                    // sum += self.flood_fill(z, hash, t);
                }
            }
            if let Field::Pipe(p) = ct {
                // sum -= 1;
                if p.is_corner_piece() {
                    t = p.get_new_reflexive_dir(t);
                }
            }
            // println!("Inside {t:?}");
            if let Some(c) = self.get_field_in_dir(cur_pt, &t) {
                let z = self.get_r_field(cur_pt, &t);
                if self.is_not_in_loop(z) {
                    sum += self.flood_fill(z, hash, t);
                }
            }
            println!("hi x{t:?}");

            let dir = match t {
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            };
            // let dir = match t{
            // Direction::South=>Direction::East,
            // Direction::East=>Direction::North,
            // Direction::North=>Direction::West,
            // Direction::West=>Direction::South
            // };

            let x = self.get_r_field(cur_pt, &dir);
            sum += self.flood_fill(x, hash, t);
        }
        if self.loop_pts.contains(&cur_pt) {
            println!("sum:{sum}");
            return sum;
        }
        sum + 1
    }
}

impl TryFrom<&str> for Grid {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.lines();
        let mut field = Vec::new();
        for i in lines {
            let mut t = Vec::new();
            for j in i.chars() {
                t.push(j.try_into()?)
            }
            field.push(t);
        }

        let mut start = (0, 0);

        for (x, i) in field.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if let Field::Start = j {
                    start = (x, y);
                }
            }
        }

        Ok(Self {
            loop_pts: HashSet::new(),
            field: field,
            start_pos: start,
        })
    }
}

fn part1(input: &str) -> u32 {
    let mut grid: Grid = input.try_into().unwrap();
    grid.save_loop_pts();
    // println!("hmm {:?}", grid.loop_pts);

    let cur_pos = grid.start_pos;
    let start = grid.field.get(cur_pos.0).unwrap().get(cur_pos.1).unwrap();
    let z = grid.get_connecting_pipes(cur_pos);
    let s = Pipe::get_start_pipe(cur_pos, z[0], z[1]);
    // println!("start = {s:?} {}", grid.loop_pts.len());
    let mut t = 0;
    stacker::maybe_grow(32 * 1024 * 32, 1024 * 32 * 1024, || {
        t=grid.loop_maze();
    });
    // println!("{} {}", grid.get_farthest_pos(), t);
    t as u32
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day10/input.txt");
    let before = std::time::Instant::now();
    let result = match part.as_str() {
        "part1" => part1(input),
        // "part2" => part2(input),
        _ => panic!("Specify one of 2 parts {part}"),
    };

    println!("The result is {result} duration:{:?}", before.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1() {
        assert_eq!(
            part1(include_str!("../inputs/day10/part1_sample.txt")),
            1707278
        )
    }

    #[test]
    fn day10_part2() {
        // assert_eq!(part2(include_str!("../inputs/day9/part2_sample.txt")), 6)
    }
}
