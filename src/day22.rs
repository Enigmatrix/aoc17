use std::collections::*;
use std::time::*;

pub fn day22_1(s: String) -> u32 {
    let mut map = HashMap::new();
    let mut current = (0,0);
    let mut dir = (-1,0);
    let t:Vec<Vec<_>> = s.split('\n').map(|v| {
            v.chars().map(|b| match b {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                }).collect()
        }).collect();
    copy_to_center(&mut map, t);
    //println!("PRINTING ORIGINAL MAP");
    //print_map(&map, current);
    let mut infected = 0;
    for i in 0..10_000{
        let current_loc = map.entry(current).or_insert(false);
        dir = if *current_loc {right(dir)} else {left(dir)};
        *current_loc = !*current_loc;
        if *current_loc {
            infected+=1;
        }
        current = (current.0+dir.0, current.1+dir.1);
    }
    //println!("PRINTING NEW MAP");
    //print_map(&map, current);
    infected
}

fn print_map(m:&HashMap<(i32,i32), bool>, c:(i32,i32)){
    let ymax = m.keys().map(|f| f.0).max().unwrap();
    let ymin = m.keys().map(|f| f.0).min().unwrap();
    let xmax = m.keys().map(|f| f.1).max().unwrap();
    let xmin = m.keys().map(|f| f.1).min().unwrap();

    for y in ymin..ymax+1{
        for x in xmin..xmax+1{
            let res = if *m.get(&(y,x)).unwrap_or(&false) { '#' } else { '.' };
            if c == (y,x){
                print!("[{}]", res);
            }
            else{
                print!(" {} ", res);
            }
        }
        println!();
    }
    
}

fn left(v:(i32,i32)) -> (i32, i32){
    if v.0 != 0{
        (v.1, v.0)
    }
    else{
        (-v.1,v.0)
    }
}
fn right(v:(i32,i32)) -> (i32, i32){
    if v.0 != 0{
        (v.1, -v.0)
    }
    else{
        (v.1,v.0)
    }
}

fn reverse(v:(i32,i32)) -> (i32,i32){
    (-v.0, -v.1)
}

fn copy_to_center(map: &mut HashMap<(i32,i32), bool>, cp:Vec<Vec<bool>>) {
    let h = cp.len() as i32;
    let w = cp[0].len() as i32;
    let left = -w/2;
    let top = -h/2;
    for x in 0..w{
        for y in 0..h{
            map.insert((top+y, left+x), cp[y as usize][x as usize]);
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State{
    Clean,
    Weakened,
    Infected,
    Flagged
}

pub fn day22_2(s: String) -> u32 {
    let mut map = HashMap::new();
    let mut current = (0,0);
    let mut dir = (-1,0);
    let t:Vec<Vec<_>> = s.split('\n').map(|v| {
            v.chars().map(|b| match b {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                }).collect()
        }).collect();
    copy_to_center2(&mut map, t);
    //println!("PRINTING ORIGINAL MAP");
    //print_map(&map, current);
    let mut infected = 0;
    for i in 0..10000000{
        let current_loc = map.entry(current).or_insert(State::Clean);
        match current_loc{
            &mut State::Clean => { *current_loc = State::Weakened; dir = left(dir); }
            &mut State::Weakened => { *current_loc = State::Infected; },
            &mut State::Infected => { *current_loc = State::Flagged; dir = right(dir); },
            &mut State::Flagged => { *current_loc = State::Clean; dir = reverse(dir); },
        }
        if *current_loc == State::Infected {
            infected+=1;
        }
        current = (current.0+dir.0, current.1+dir.1);
    }
    //println!("PRINTING NEW MAP");
    //print_map(&map, current);
    infected
}

fn copy_to_center2(map: &mut HashMap<(i32,i32), State>, cp:Vec<Vec<bool>>) {
    let h = cp.len() as i32;
    let w = cp[0].len() as i32;
    let left = -w/2;
    let top = -h/2;
    for x in 0..w{
        for y in 0..h{
            map.insert((top+y, left+x), if cp[y as usize][x as usize] {State::Infected} else {State::Clean});
        }
    }
}