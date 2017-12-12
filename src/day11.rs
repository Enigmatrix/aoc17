use std::collections::*;
pub fn day11_1(s: String) -> i32{
    let out = s.split(',');
    let mut x = 0i32;
    let mut y = 0i32;
    let mut z = 0i32;
    for i in out{
        match i{
            "s" => {x-=1;y+=1;},
            "se" => {x-=1;z+=1;},
            "sw" => {z-=1;y+=1;},
            "n" => {x+=1;y-=1;},
            "ne" => {z+=1;y-=1;},
            "nw" => {x+=1;z-=1;},
            _ => panic!()
        }
    }
    (x.abs() + y.abs() + z.abs())/2
}

pub fn day11_2(s: String) -> i32 {
    let out = s.split(',');
    let mut x = 0i32;
    let mut y = 0i32;
    let mut z = 0i32;
    let mut max = 0i32;
    for i in out{
        match i{
            "s" => {x-=1;y+=1;},
            "se" => {x-=1;z+=1;},
            "sw" => {z-=1;y+=1;},
            "n" => {x+=1;y-=1;},
            "ne" => {z+=1;y-=1;},
            "nw" => {x+=1;z-=1;},
            _ => panic!()
        }
        let ndist = (x.abs() + y.abs() + z.abs())/2;
        max = if ndist > max { ndist } else { max }; 
    }
    max
}
