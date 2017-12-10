use std::collections::*;
pub fn day10_1(s: String, max: usize) -> usize {
    let mut buf = (0..max).collect::<Vec<usize>>();
    let slice = buf.as_mut_slice();
    let lens = s.split(',').map(|v| v.parse::<usize>().unwrap());
    let mut current = 0;
    for (skip, len) in lens.enumerate() {
        let end = (current + len) % max;
        for i in 0..(len / 2) {
            let tmp = slice[(current + i) % max];
            slice[(current + i) % max] = slice[(end + max * 3 - i - 1) % max];
            slice[(end + max * 3 - i - 1) % max] = tmp;
        }

        current = (current + len + skip) % max;
    }
    slice[0] * slice[1]
}

pub fn day10_2(s: String) -> String {
    let max = 256;
    let mut buf = (0..max).collect::<Vec<usize>>();
    {
        let slice = buf.as_mut_slice();
        let lens: Vec<usize> = [
            &s.chars().map(|v| v as usize).collect::<Vec<usize>>()[..],
            &vec![17, 31, 73, 47, 23][..],
        ].concat();
        let mut current = 0;
        let mut skip = 0;

        for _ in 0..64 {
            for len in lens.iter() {
                let end = (current + len) % max;
                for i in 0..(len / 2) {
                    let tmp = slice[(current + i) % max];
                    slice[(current + i) % max] = slice[(end + max * 3 - i - 1) % max];
                    slice[(end + max * 3 - i - 1) % max] = tmp;
                }
                current = (current + len + skip) % max;
                skip += 1;
            }
        }
    }
    buf.chunks(16)
        .map(|v| v.iter().fold(0, |a,c| a^c))
        .fold("".to_owned(), |a,c| a+&format!("{:x}", c))
}
