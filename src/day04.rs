pub fn day04_1(s : String) -> u32{
    s
        .split("\n")
        .filter(|v| -> bool {
            let mut row:Vec<&str> = v.split(" ").collect();
            row.sort();
            let x = row.len();
            row.dedup();
            let y = row.len();
            x == y
            })
        .count() as u32
}

pub fn day04_2(s: String) -> u32{
    s
        .split("\n")
        .filter(|v| -> bool {
            let mut row:Vec<String> = v.split(" ").map(|st| -> String{
                let mut g:Vec<char> = st.to_string().chars().collect();
                g.sort();
                g.iter().collect::<String>() }).collect();
            row.sort();
            let x = row.len();
            row.dedup();
            let y = row.len();
            x == y
            })
        .count() as u32
}