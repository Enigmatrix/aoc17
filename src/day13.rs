use std::collections::*;

pub fn day13_1(s: String) -> usize {
    let list: HashMap<usize, usize> = s.split('\n')
        .map(|v| {
            let line: Vec<_> = v.split(": ").collect();
            (line[0].parse().unwrap(), line[1].parse().unwrap())
        })
        .collect();

    let mut error = 0;
    for (time, l) in list.iter() {
        let len = l * 2 - 2;
        if time % len == 0 {
            error += l * time;
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
    let mut fail = true;
    let mut delay = 0;
    while fail {
        fail = false;
        delay += 1;

        for (time, l) in list.iter() {
            let len = l * 2 - 2;
            if (delay+time) % len == 0 {
                fail = true;
                break;
            }
        }
    }
    delay
}
