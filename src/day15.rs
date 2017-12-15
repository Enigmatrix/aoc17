use std::collections::*;

pub fn day15_1(a: u64, b:u64) -> u32 {
    let mut count = 0;
    let mut a = a;
    let mut b = b;
    let factor_a = 16807;
    let factor_b = 48271 ;
    for _ in 0..40_000_000{
        a = (a*factor_a) % 0x7FFFFFFF;
        b = (b*factor_b) % 0x7FFFFFFF;
        if (a & 0xFFFF) == (b & 0xFFFF) {
            count+=1;
        }
    }
    count
}
pub fn day15_2(a:u64, b:u64) -> u32 {
    let mut count = 0;
    let mut a = a;
    let mut b = b;
    let factor_a = 16807;
    let factor_b = 48271 ;
    let mut a_q = VecDeque::new();
    let mut b_q = VecDeque::new();
    let mut counted = 0;
    while counted < 5_000_000{
        a = (a*factor_a) % 0x7FFFFFFF;
        b = (b*factor_b) % 0x7FFFFFFF;
        if a % 4 == 0 {a_q.push_back(a);}
        if b % 8 == 0 {b_q.push_back(b);}
        if let (Some(_),Some(_)) = (a_q.front(),b_q.front()){
            let af = a_q.pop_front().unwrap();
            let bf = b_q.pop_front().unwrap();
            counted+=1;
            if (af & 0xFFFF) == (bf & 0xFFFF) {
                count+=1;
            }
        }
    }
    count
}