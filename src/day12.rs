use std::collections::*;
use disjoint_sets::UnionFind;
pub fn day12_1(s: String) -> usize {
    let m:Vec<_> = s.split('\n').map(|line| {
        let dat = line.split(" <-> ").collect::<Vec<&str>>();
        (
            dat[0].parse::<usize>().unwrap(),
            dat[1]
                .split(", ")
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        )
    }).collect();
    let mut uf = UnionFind::new(m.len());
    
    for &(src, ref dests) in m.iter() {
        for dst in dests {
            if !uf.equiv(src, *dst) {
                uf.union(src, *dst);
            }
        }
    }
    m.iter().filter(|v| uf.equiv(0, v.0)).count()
}

pub fn day12_2(s: String) -> usize {
    
    let m:Vec<_> = s.split('\n').map(|line| {
        let dat = line.split(" <-> ").collect::<Vec<&str>>();
        (
            dat[0].parse::<usize>().unwrap(),
            dat[1]
                .split(", ")
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        )
    }).collect();
    let mut uf = UnionFind::new(m.len());
    
    for &(src, ref dests) in m.iter() {
        for dst in dests {
            if !uf.equiv(src, *dst) {
                uf.union(src, *dst);
            }
        }
    }
    m.iter().map(|v| uf.find(v.0)).collect::<HashSet<usize>>().len()
}
