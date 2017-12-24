use std::collections::*;
use std::cell::*;
use std::rc::*;

pub fn day24_1(s: String) -> u32 {
    let mut g: Vec<_> = s.split('\n')
        .map(|v| {
            let g: Vec<u32> = v.split('/').map(|h| h.parse().unwrap()).collect();
            (g[0], g[1])
        })
        .collect();
    g.sort_by(|a, b| b.cmp(a));
    let used = vec![false; g.len()];
    max(0, 0, &g, &Rc::new(RefCell::new(used)))
}

fn max(cur: u32, d: usize, g: &Vec<(u32, u32)>, used: &Rc<RefCell<Vec<bool>>>) -> u32 {
    g.iter()
        .enumerate()
        .filter(|&(i, &io)| {
            (io.0 == cur || io.1 == cur) && !used.borrow()[i]
        })
        .map(|(i, &io)| {
            let not_used = if io.0 == cur { io.1 } else { io.0 };
            *used.borrow_mut().get_mut(i).unwrap() = true;
            let m = max(not_used, d + 1, g, used);
            *used.borrow_mut().get_mut(i).unwrap() = false;
            io.0 + io.1 + m
        })
        .max()
        .unwrap_or(0)
}



pub fn day24_2(s: String) -> u32 {
    let mut g: Vec<_> = s.split('\n')
        .map(|v| {
            let g: Vec<u32> = v.split('/').map(|h| h.parse().unwrap()).collect();
            (g[0], g[1])
        })
        .collect();
    g.sort_by(|a, b| b.cmp(a));
    let used = vec![false; g.len()];
    max2(0, 0, &g, &Rc::new(RefCell::new(used))).1
}

fn max2(cur: u32, d: u32, g: &Vec<(u32, u32)>, used: &Rc<RefCell<Vec<bool>>>) -> (u32, u32) {
    g.iter()
        .enumerate()
        .filter(|&(i, &io)| {
            (io.0 == cur || io.1 == cur) && !used.borrow()[i]
        })
        .map(|(i, &io)| {
            let not_used = if io.0 == cur { io.1 } else { io.0 };
            *used.borrow_mut().get_mut(i).unwrap() = true;
            let (l, m) = max2(not_used, d + 1, g, used);
            *used.borrow_mut().get_mut(i).unwrap() = false;
            (l, io.0 + io.1 + m)
        })
        .max()
        .unwrap_or((d, 0))
}
