use std::collections::*;
use disjoint_sets::UnionFind;
use day10::day10_2;

pub fn day14_1(s: String) -> u32 {
    (0..128)
        .map(|v| -> u32 {
            day10_2(s.clone() + "-" + &v.to_string())
                .chars()
                .map(used)
                .sum()
        })
        .sum()
}

fn used(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 1,
        '3' => 2,
        '4' => 1,
        '5' => 2,
        '6' => 2,
        '7' => 3,
        '8' => 1,
        '9' => 2,
        'a' => 2,
        'b' => 3,
        'c' => 2,
        'd' => 3,
        'e' => 3,
        'f' => 4,
        _ => panic!(),
    }
}

pub fn day14_2(s: String) -> usize {
    let maxlen = 128;
    let arr: Vec<Vec<_>> = (0..maxlen)
        .map(|v| -> Vec<bool> {
            let code = s.clone() + "-" + &v.to_string();
            day10_2(code).chars()
                .map(used_vec)
                .fold(Vec::new(), |a, c| [&a[..], &c[..]].concat())
        })
        //.inspect(|v| println!("{}", v.len())) 
        .collect();
        
    let mut uf: UnionFind<usize> = UnionFind::new(128*1000+128);
    for i in 0..maxlen as i32 {
        for j in 0..maxlen as i32 {
            if !arr[i as usize][j as usize] {
                continue;
            }
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for dir in dirs {
                let ni = dir.0 + i;
                let nj = dir.1 + j;
                if ni >= 128 || ni < 0 || nj <0 || nj >= 128 || !arr[ni as usize][nj as usize] {
                    continue;
                }
                if !uf.equiv((ni*1000+nj) as usize, (i*1000+j) as usize) {
                   uf.union((ni*1000+nj) as usize, (i*1000+j) as usize);
                }
            }
        }
    }
    let mut hs = HashSet::new();
    for i in 0..maxlen {
        for j in 0..maxlen {
            if !arr[i as usize][j as usize] {continue}
            hs.insert(uf.find(i*1000+j));
        }
    }
    hs.len()
}

fn used_vec(c: char) -> Vec<bool> {
    match c {
        '0' => vec![false, false, false, false],
        '1' => vec![false, false, false, true],
        '2' => vec![false, false, true, false],
        '3' => vec![false, false, true, true],
        '4' => vec![false, true, false, false],
        '5' => vec![false, true, false, true],
        '6' => vec![false, true, true, false],
        '7' => vec![false, true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        'a' => vec![true, false, true, false],
        'b' => vec![true, false, true, true],
        'c' => vec![true, true, false, false],
        'd' => vec![true, true, false, true],
        'e' => vec![true, true, true, false],
        'f' => vec![true, true, true, true],
        _ => panic!(),
    }
}
