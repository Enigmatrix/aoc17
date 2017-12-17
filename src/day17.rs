use std::collections::*;
use std::time::*;

pub fn day17_1(s: usize) -> u32 {
    let mut arr = vec![0];
    let mut cur = 0;
    for i in 1..2018{
        cur = ((cur + s) % arr.len())+1;
        arr.insert(cur, i);
    }
    arr[(cur+1)%arr.len()]
}
pub fn day17_2(s: usize) -> u32 {
    let mut cur = 0;
    let mut zer_aft = 0;
    for i in 1..50_000_001{
        cur = ((cur + s) % i)+1;
        if cur == 1 {
            zer_aft = i;
        }
    }
    zer_aft as u32
}
