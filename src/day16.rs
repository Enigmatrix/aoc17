use std::collections::*;
use std::time::Instant;

pub fn day16_1(s: String) -> String {
    let instrs: Vec<Vec<char>> = s.split(',').map(|v| v.chars().collect()).collect();
    let mut pos: Vec<char> = "abcdefghijklmnop".chars().collect();
    for instr in instrs {
        let rest_chars = &instr[1..];
        let rest: String = rest_chars.iter().cloned().collect();
        //println!("{:?}, b4: {:?}", instr, pos);
        match instr[0] {
            's' => {
                let x: usize = rest.parse().unwrap();
                let rx = pos.len() - x;

                let b4 = pos.clone();
                for i in x..pos.len() {
                    pos[i] = b4[i - x];
                }
                for i in 0..x {
                    pos[i] = b4[rx + i];
                }
            }
            'x' => {
                let l: Vec<usize> = rest.split('/').map(|v| v.parse().unwrap()).collect();
                let tmp = pos[l[0]];
                pos[l[0]] = pos[l[1]];
                pos[l[1]] = tmp;
            }
            'p' => {
                let a = rest_chars[0];
                let b = rest_chars[2];
                let l0 = pos.iter().position(|&r| r == a).unwrap();
                let l1 = pos.iter().position(|&r| r == b).unwrap();
                let tmp = pos[l0];
                pos[l0] = pos[l1];
                pos[l1] = tmp;
            }
            _ => panic!(),
        }
    }
    pos.iter().collect()
}
#[derive(Debug)]
enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

/*
struct Node {
    data: char,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

struct LinkedList<'a> {
    head: &'a Box<Node>,
    tail: &'a Box<Node>,
}

impl<'a> LinkedList<'a> {
    pub fn len(&self) -> usize {
        let mut tmp = self.head;
        let mut ln = 0;
        while let Some(ref node) = tmp.next {
            ln += 1;
            tmp = node;
        }
        ln
    }
    pub fn at(&self, idx: usize) -> Option<&'a Box<Node>> {
        if idx == 0 {
            None
        } else {
            let mut tmp = self.head;
            let mut ln = 0;
            while let Some(ref node) = tmp.next {
                if idx == ln {break;}
                ln += 1;
                tmp = node;
            }
            Some(tmp)
        }
    }
    pub fn spin(&mut self, rx: usize){
        if let Some(node) = self.at(rx){
            let Node{data:_, next:_, ref prev} = **node;
            if let &Some(ref prev) = prev{
                let oldhead = self.head;
                
            }
        }
    }
}

fn to_linked_list<'a>(tail: &'a Option<Box<Node>>) -> LinkedList<'a> {
    if let &Some(ref tail) = tail {
        let mut head = tail;
        while let Some(ref hd) = head.prev {
            head = hd;
        }
        LinkedList { head, tail }
    } else {
        panic!()
    }
}*/

pub fn day16_2(s: String) -> String {
    let instrs: Vec<_> = s.split(',')
        .map(|v| {
            let chars: Vec<_> = v.chars().collect();
            let rest_chars = &chars[1..];
            let rest: String = rest_chars.iter().cloned().collect();
            match chars[0] {
                's' => Op::Spin(rest.parse().unwrap()),
                'x' => {
                    let l: Vec<usize> = rest.split('/').map(|v| v.parse().unwrap()).collect();
                    Op::Exchange(l[0], l[1])
                }
                'p' => Op::Partner(rest_chars[0], rest_chars[2]),
                _ => panic!(),
            }
        })
        .collect();
    let mut pos:LinkedList<char> = "abcdefghijklmnop".chars().collect();

    let len = pos.len();
    let start = Instant::now();
    
    let mut seen = HashMap::new();
    let mut sseen = HashMap::new();
    let mut idx = 0;
    
    for _r in 0..1_000_000_000 {
        if _r % 100_000 == 0{
            println!("{}: {:?}", _r,start.elapsed());
        }
        for instr in instrs.iter() {
            //println!("{:?}", instr);
            match *instr {
                Op::Spin(x) => {
                    let rx = len - x;
                    let mut d = pos.split_off(rx);
                    d.append(&mut pos);
                    pos = d;
                }
                Op::Exchange(a,b) => {
                    let mut iter = pos.iter_mut();
                    let ma = if a < b { a } else { b };
                    let mb = if a > b { a } else { b };
                    let a = iter.nth(ma).unwrap();
                    let b = iter.nth(mb-ma-1).unwrap();
                    let tmp = *a;
                    *a = *b;
                    *b = tmp;
                }
                Op::Partner(a,b) => {
                    let mut iter = pos.iter_mut();
                    let ma = iter.find(|&&mut v| v == a || v == b).unwrap();
                    let mb = iter.find(|&&mut v| v == b || v == a).unwrap();
                    if *ma == a{
                        *ma = b;
                        *mb = a;
                    }
                    else {
                        *ma = a;
                        *mb = b;
                    }
                }
            }
        }
        
        let val = *seen.entry(pos.clone()).or_insert(_r);
        sseen.entry(_r).or_insert(pos.clone());
        println!("{}, {}", val,_r);
        if val != _r{
            //cycle
            let cycle = _r-val;
            idx = ((1_000_000_000-val)%cycle)+val-1;
            //answer = 
            break;
        }
    }
    

    (*sseen.entry(idx).or_insert(pos.clone())).iter().collect()
}
