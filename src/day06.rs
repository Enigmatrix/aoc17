use std::collections::*;
pub fn day06_1(s : String) -> u32{
    let mut config:Vec<u32> = s.split('\t')
        .map(|v| v.parse().expect("v to be int")).collect();
    let mut cycles = 0;
    let mut hashv = hash(&config);
    let len = config.len();
    let mut seen:HashSet<u32> = HashSet::new();
    while !seen.contains(&hashv) {
        seen.insert(hashv);
        let (idx, max) = config.iter().cloned()
            .enumerate()
            .max_by_key(|&(i,v)| (v,-(i as i32))).unwrap();
        let lenu32 = len as u32;
        let distrib = max/lenu32;
        let extra = (max%lenu32) as usize;
        config[idx] = 0;
        for i in 1..(len+1){
            let idx = (idx + i) % len;
            config[idx] += distrib;
        }
        for i in 1..(extra+1){
            let idx = (idx + i) % len;
            config[idx] += 1;
        }
        //println!("{},{},{}", max,distrib,lenu32-1);
        //println!("{:?}", config);
        hashv = hash(&config);
        cycles+=1;
    }
    cycles
}

pub fn hash(v:&Vec<u32>)->u32{
    let mut seed = v.len() as u64;
    for i in v.iter() {
        seed ^= (*i as u64)
            .overflowing_add(0x9e3779b9u64
            .overflowing_add(seed << 6).0
            .overflowing_add(seed >> 2).0).0;
    }
    seed as u32
}

pub fn day06_2(s: String) -> u32{
    let mut config:Vec<u32> = s.split('\t')
        .map(|v| v.parse().expect("v to be int")).collect();
    let mut cycles = 0;
    let mut hashv = hash(&config);
    let len = config.len();
    let mut seen:HashSet<u32> = HashSet::new();
    let mut seenc:HashMap<u32, u32> = HashMap::new();
    while !seen.contains(&hashv) {
        seen.insert(hashv);
        seenc.entry(hashv).or_insert(cycles);
        let (idx, max) = config.iter().cloned()
            .enumerate()
            .max_by_key(|&(i,v)| (v,-(i as i32))).unwrap();
        let lenu32 = len as u32;
        let distrib = max/lenu32;
        let extra = (max%lenu32) as usize;
        config[idx] = 0;
        for i in 1..(len+1){
            let idx = (idx + i) % len;
            config[idx] += distrib;
        }
        for i in 1..(extra+1){
            let idx = (idx + i) % len;
            config[idx] += 1;
        }
        //println!("{},{},{}", max,distrib,lenu32-1);
        //println!("{:?}", config);
        hashv = hash(&config);
        cycles+=1;
    }
    cycles-*seenc.entry(hashv).or_insert(cycles)
}