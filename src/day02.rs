pub fn day02_1(s : String) -> u32{
    s
        .split("\n")
        .map(|v| -> u32 {
            let row:Vec<u32> = v.split("\t").map(|e| e.parse::<u32>().unwrap()).collect();
            row.iter().max().unwrap() - row.iter().min().unwrap()
            })
            .sum()
}

pub fn day02_2(s: String) -> u32{
    s
        .split("\n")
        .map(|v| -> u32 {
            let row:Vec<u32> = v.split("\t").map(|e| e.parse::<u32>().unwrap()).collect();
            let (r1, c1) = 
            row.iter()
            .map(|&r| (r, row.iter().find(|&c| r%c==0 && r!=*c)))
            .filter(|&(_,c)| c.is_some()).next().unwrap();
            r1/c1.unwrap()
            })
            .inspect(|x| println!("about to filter: {}", x))
            .sum()
    
}