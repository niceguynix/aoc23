use std::collections::HashMap;

use itertools::Itertools;

fn is_row_empty(grid:&Vec<Vec<char>>,row_num:usize)->bool{
    let mut is_empty=grid.get(row_num).unwrap().iter().filter(|x| **x=='#').count();

    if is_empty==0 {true} else {false}
}


fn is_column_empty(grid:&Vec<Vec<char>>,col_num:usize)->bool{
    let mut is_empty=true;
    for i in grid{
        let t = i.get(col_num).unwrap();
        if *t=='#'{
            is_empty=false;
        }
    }

    is_empty
}

fn insert_col(grid:&mut Vec<Vec<char>>,idx:usize){
    for i in (0..grid.len()){
        let mut t =grid.get_mut(i).unwrap();
        t.insert(idx, '.');
    }
}

fn parse_grid(input:&str)->Vec<Vec<char>>{
    let x=input.lines().collect::<Vec<_>>();
    let mut x=x.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    x
}

fn get_galaxy_loc(grid:&Vec<Vec<char>>)->Vec<(usize,usize)>{

    let mut galaxy_loc:Vec<(usize,usize)>=Vec::new();

    for (x,i) in grid.iter().enumerate(){
        for (y,c) in i.iter().enumerate(){
            if *c=='#'{
                galaxy_loc.push((x,y));
            }
        }
    }
    galaxy_loc
}
fn part1(input:&str)->u128{
    
    let mut x=parse_grid(input);
    let mut i=0;
    let cl=x.get(0).unwrap().len();
    while i<x.len(){
        let emp=is_row_empty(&x, i);
        if emp{
            x.insert(i, (0..cl).map(|_| '.').collect_vec());
            i+=1;
        }
        i+=1;
    }


    let mut i=0;
    while i<x.get(0).unwrap().len(){
        if is_column_empty(&x, i){
            insert_col(&mut x, i);
            i+=1;
        }
        i+=1;
    }
    // println!("{x:?}");
    
    let mut sum=0;
    let galaxy_loc=get_galaxy_loc(&x);
    for i in 0..galaxy_loc.len(){

        for j in (i+1)..galaxy_loc.len(){
           let t1=galaxy_loc.get(i).unwrap();
           let t2=galaxy_loc.get(j).unwrap();
           sum+=(t1.0 as i32 -t2.0 as i32).abs() + (t1.1 as i32 - t2.1 as i32).abs(); 
        }
    }

    sum as u128
}

fn part2(input:&str)->u128{
    let grid=parse_grid(input);
    let galaxy_loc=get_galaxy_loc(&grid);

    let mut sum=0;
    let mut hash1:HashMap<usize, bool>=HashMap::new();
    let mut hash2:HashMap<usize, bool>=HashMap::new();

    let mut row_cache = |grid:&Vec<Vec<char>>,i:usize|->bool{
        if let Some(c)=hash1.get(&i){
            return *c;
        }
        let t=is_row_empty(grid, i);
        hash1.insert(i, t);
        t
    }; 

    let mut col_cache= |grid:&Vec<Vec<char>>,i:usize|->bool{
        if let Some(c)=hash2.get(&i){
            return *c;
        }
        let t=is_column_empty(grid,i);
        hash2.insert(i, t);
        t
    }; 

    for (a,b) in galaxy_loc.iter().tuple_combinations(){
        for i in a.0.min(b.0)..a.0.max(b.0){
            if row_cache(&grid, i){
                sum+=1_000_000;
            }else{
                sum+=1;
            }
        }

        for j in a.1.min(b.1)..a.1.max(b.1){
            if col_cache(&grid, j){
                sum+=1_000_000;
            }else{
                sum+=1;
            }
        }
    }

    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day11/input.txt");
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
    fn day11_part1() {
        assert_eq!(
            part1(include_str!("../inputs/day11/part1_sample.txt")),
            1707278
        )
    }

    #[test]
    fn day11_part2() {
        assert_eq!(part2(include_str!("../inputs/day11/part1_sample.txt")), 6)
    }
}
