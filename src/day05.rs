pub fn day05_1(s : String) -> u32{
    let mut list:Vec<i32> = s.split("\n")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut idx = 0i32;
    let ln = list.len() as i32;
    let mut sum = 0;
    while idx < ln {
        let nidx = list[idx as usize]+idx;
        list[idx as usize] += 1;
        idx = nidx;
        sum+=1;
    }
    sum
}

pub fn day05_2(s: String) -> u32{
    let mut list:Vec<i32> = s.split("\n")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut idx = 0i32;
    let ln = list.len() as i32;
    let mut sum = 0;
    while idx < ln {
        let nidx = list[idx as usize]+idx;
        if list[idx as usize] >= 3 
             { list[idx as usize] -= 1; }
        else { list[idx as usize] += 1; }
        idx = nidx;
        sum+=1;
    }
    sum
}