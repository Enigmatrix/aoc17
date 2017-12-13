use std::collections::*;

pub fn day13_1(s: String) -> usize {
    let list: HashMap<usize, usize> = s.split('\n')
        .map(|v| {
            let line: Vec<_> = v.split(": ").collect();
            (line[0].parse().unwrap(), line[1].parse().unwrap())
        })
        .collect();
    let maxdepth = *list.iter().map(|v| v.0).max().unwrap();
    let mut error = 0;
    for time in 0..(maxdepth + 1) {
        if list.contains_key(&time) {
            let l = *list.get(&time).unwrap();
            let len = l * 2 - 2;
            if time % len == 0 {
                //error
                error += l * time;
                println!("{}: {}", time, l);
            }
        }
    }
    error
}

pub fn day13_2(s: String) -> usize {
    let list: HashMap<usize, usize> = s.split('\n')
        .map(|v| {
            let line: Vec<_> = v.split(": ").collect();
            (line[0].parse().unwrap(), line[1].parse().unwrap())
        })
        .collect();
    let maxdepth = *list.iter().map(|v| v.0).max().unwrap();
    let mut error = 0;
    let mut fail = true;
    let mut delay = 0;
    while fail {
        fail = false;
        delay+=1;
        for time in 0..(maxdepth + 1) {
            if list.contains_key(&time) {
                let l = *list.get(&time).unwrap();
                let len = l * 2 - 2;
                if (time + delay) % len == 0 {
                    fail = true;
                }
            }
        }
    }
    delay
}
