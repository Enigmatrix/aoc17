pub fn day01_1(s : String) -> u32{
    let input:Vec<char> = s.chars().collect();
    let len:usize = input.len();
    input
        .iter()
        .enumerate()
        .map(|(i, c)| 
            if c == &input[(i+1)%len]
                { c.to_digit(10).unwrap() } else { 0 })
        .sum()
}

pub fn day01_2(s: String) -> u32{
    let input:Vec<char> = s.chars().collect();
    let len:usize = input.len();
    input
        .iter()
        .enumerate()
        .map(|(i, c)| 
            if c == &input[(i+len/2)%len]
                { c.to_digit(10).unwrap() } else { 0 })
        .sum()
}