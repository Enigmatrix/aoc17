use std::collections::*;
use std::time::*;
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
enum Piece{
    Vert,
    Horiz,
    Letter(char),
    Any,
    None,
}

pub fn day19_1(s: String) -> String {
    let cross_map:Vec<Vec<_>> = s.split('\n')
        .map(|line| line.chars().map(|c| match c {
            '|' => Piece::Vert,
            '-' => Piece::Horiz,
            '+' => Piece::Any,
            ' ' => Piece::None,
            x if x.is_alphabetic() => Piece::Letter(x),
            _ => panic!()
        }).collect()).collect();

    let start = cross_map[0].iter().position(|r| r == &Piece::Vert).unwrap();
    let mut pos:(i32,i32) = (0, start as i32);
    let mut dir:(i32,i32) = (1,0);
    let mut seen_letters = VecDeque::new();
    
    loop{
        //let current = cross_map.get(pos.0).unwrap().get(pos.1).unwrap();
        //println!("{:?}", pos);
        let new = cross_map.get((pos.0 + dir.0) as usize).unwrap().get((pos.1+dir.1) as usize).unwrap();
        let change_dir = if let &Piece::None = new { true } else { false };
        if let &Piece::Letter(x) = new { seen_letters.push_back(x); }
        if change_dir {
            let ndir = if dir.0 == 0 { (1,0) } else { (0,1) };
            if let &Piece::None = cross_map.get((pos.0+ndir.0) as usize).unwrap().get((pos.1+ndir.1) as usize).unwrap(){
                let ndir = if dir.0 == 0 { (-1,0) } else { (0,-1) };
                if let &Piece::None = cross_map.get((pos.0+ndir.0) as usize).unwrap().get((pos.1+ndir.1) as usize).unwrap(){
                    break;
                }
                else {
                    dir = ndir;
                }
            }
            else {
                dir = ndir;
            }
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    

    seen_letters.iter().collect()
}
pub fn day19_2(s: String) -> u32 {
    let cross_map:Vec<Vec<_>> = s.split('\n')
        .map(|line| line.chars().map(|c| match c {
            '|' => Piece::Vert,
            '-' => Piece::Horiz,
            '+' => Piece::Any,
            ' ' => Piece::None,
            x if x.is_alphabetic() => Piece::Letter(x),
            _ => panic!()
        }).collect()).collect();

    let start = cross_map[0].iter().position(|r| r == &Piece::Vert).unwrap();
    let mut pos:(i32,i32) = (0, start as i32);
    let mut dir:(i32,i32) = (1,0);
    let mut seen_letters = VecDeque::new();
    let mut count = 1;
    loop{
        //let current = cross_map.get(pos.0).unwrap().get(pos.1).unwrap();
        //println!("{:?}", pos);
        count+=1;
        let new = cross_map.get((pos.0 + dir.0) as usize).unwrap().get((pos.1+dir.1) as usize).unwrap();
        let change_dir = if let &Piece::None = new { true } else { false };
        if let &Piece::Letter(x) = new { seen_letters.push_back(x); }
        if change_dir {
            let ndir = if dir.0 == 0 { (1,0) } else { (0,1) };
            if let &Piece::None = cross_map.get((pos.0+ndir.0) as usize).unwrap().get((pos.1+ndir.1) as usize).unwrap(){
                let ndir = if dir.0 == 0 { (-1,0) } else { (0,-1) };
                if let &Piece::None = cross_map.get((pos.0+ndir.0) as usize).unwrap().get((pos.1+ndir.1) as usize).unwrap(){
                    break;
                }
                else {
                    dir = ndir;
                }
            }
            else {
                dir = ndir;
            }
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }

    count-1
}
