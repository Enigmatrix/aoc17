use std::collections::*;
use std::time::*;


type Pattern = Vec<Vec<bool>>;

struct Mapping {
    input: Pattern,
    output: Pattern,
}

impl Mapping {
    fn parse(s: &str) -> Mapping {
        let mut raw: Vec<Pattern> = s.split(" => ")
            .map(|v| {
                v.split('/')
                    .map(|v| {
                        v.chars()
                            .map(|v| match v {
                                '#' => true,
                                '.' => false,
                                _ => panic!(),
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        let mut all = raw.drain(..);
        let input = all.next().unwrap();
        let output = all.next().unwrap();
        Mapping { input, output }
    }

    fn input_match(&self, v: &mut Pattern, x: usize, y: usize, sz: usize) -> bool {
        if sz != self.input.len() {
            return false;
        }
        for n in 0..8{
            let rvs = n & 0b001;
        }
        true
    }

   fn rotate_once(v: &Pattern, rvs: bool, a:bool,b:bool) -> Pattern{
       let ln = v.len();
       let mut newv = vec![vec![false;ln]; ln];
       for r in 0..ln{
           for c in 0..ln{
               let rn = if rvs { c } else { r };
               let cn = if rvs { r } else { c };
               let rn = if a { ln-rn } else { rn };
               let cn = if b { ln-cn } else { cn };
               newv[r][c] = v[rn][cn];
           }
       }
       newv
   }


    fn is_equal(s:Pattern, v: &Pattern, x: usize, y: usize) -> bool {
        let ln = s.len();
        for r in 0..ln {
            let row = v.get(x + r).unwrap();
            for c in 0..ln {
                if s[r][c] != row[c + y] {
                    return false;
                }
            }
        }
        true
    }

    fn copy_output(&self, target: &mut Pattern, row: usize, col: usize) {
        let src = &self.output;
        let ln = src.len();
        for r in 0..ln {
            let row = target.get_mut(row + r).unwrap();
            for c in 0..ln {
                *row.get_mut(col + c).unwrap() = src[r][c];
            }
        }
    }
}

fn extend_every(sz: usize, v: &mut Pattern) {
    let current_sz = v.len();
    let extra = current_sz / sz;
    for i in 1..(extra + 1) {
        v.insert(i * sz + (i - 1), vec![false; current_sz]);
    }
    for e in v.iter_mut() {
        for i in 1..(extra + 1) {
            e.insert(i * sz + (i - 1), false);
        }
    }
}

pub fn day21_1(s: String) -> usize {
    let mappings = s.split('\n').map(|v| Mapping::parse(v));
    let mut current = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for i in 0..5 {
        if current.len() % 2 == 0 {
            extend_every(2, &mut current);
        } else {
            extend_every(3, &mut current);
        }
    }
    1
}
pub fn day21_2(s: String) -> usize {
    1
}
