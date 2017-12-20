use std::collections::*;
use std::time::*;
use std::ops::*;
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
struct Vec3{
    x:i64,
    y:i64,
    z:i64
}
impl Vec3{
    fn parse(s: &str) -> Vec3{
        let ln = s.len()-1;
        let vecraw:Vec<_> = s[3..ln].split(',')
            .map(|v| v.parse().unwrap()).collect();
        Vec3{
            x: vecraw[0],
            y: vecraw[1],
            z: vecraw[2],
        }
    }

    fn dist(&self) -> i64{
        self.x.abs()+self.y.abs()+self.z.abs()
    }
}
impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
#[derive(Debug)]
struct Particle{
    p: Vec3,
    v: Vec3,
    a: Vec3
}
impl Particle{
    fn parse(s: &str) -> Particle{
        let particleraw:Vec<_> = s.split(", ")
            .map(|v| Vec3::parse(v)).collect();
        Particle{
            p: particleraw[0],
            v: particleraw[1],
            a: particleraw[2],
        }
    }
}
pub fn day20_1(s: String) -> usize {
    let mut data:Vec<_> = s.split("\n").map(|line| Particle::parse(line)).collect();
    for _ in 0..10_000{
        for dat in data.iter_mut(){
            dat.v += dat.a;
            dat.p += dat.v;
        }
    }
    data.iter().enumerate().min_by_key(|v| v.1.p.dist()).unwrap().0
}
pub fn day20_2(s: String) -> usize {
    let mut data:Vec<_> = s.split("\n")
        .map(|line| Particle::parse(line)).collect();
    let mut deleted = vec![false; data.len()];
    for _ in 0..1000{

        for (i, dat) in data.iter_mut().enumerate(){
            if deleted[i] {
                continue;
            }
            dat.v += dat.a;
            dat.p += dat.v;
        }

        let mut uniq:HashMap<Vec3, usize> = HashMap::new();
        let mut dups = HashSet::new();

        for (i, dat) in data.iter().enumerate(){
            if deleted[i]{
                continue;
            }
            if let Some(&idx) = uniq.get(&dat.p) {
                //println!("dups {}",i );
                dups.insert(i);
                dups.insert(idx);
            }
            else{
                //println!("uniq insert {}",i );
                uniq.insert(dat.p, i);
            }
        }

        
        for &i in dups.iter(){
            //just set a bool instead
            //println!("remove {}", idx);
            *deleted.get_mut(i).unwrap() = true;
        }
    }
    deleted.iter().filter(|&&v| !v).count()
}