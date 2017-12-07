use std::collections::*;
pub fn day07_1(s : String) -> String{
    let list:Vec<_> = s.split('\n')
        .map(|v| {
            let pl:Vec<_> = v.split("-> ").collect();
            let info:Vec<_> = pl[0].split(" ").collect();
            let name = info[0];
            let refs = if pl.len() > 1 { pl[1].split(", ").collect() } else {Vec::new()};
            (name, refs)
        }).collect();
    let mut names:Vec<_> = list.iter().map(|v| v.0).collect();
    let mut refs:Vec<_> = list.iter().fold(Vec::new(), |acc, v| [&acc[..], &v.1[..]].concat());
    names.sort();
    refs.sort();
    let maxlen = names.len();
    let mut ans:&str = "";
    for i in 0..maxlen{
        if names[i] != refs[i%(maxlen-1)]{
            ans = names[i];
            break;
        }
    }
    ans.to_string()
}
pub fn day07_2(s: String) -> u32{
    let list:Vec<_> = s.split('\n')
        .map(|v| {
            let pl:Vec<_> = v.split("-> ").collect();
            let info:Vec<_> = pl[0].split(" ").collect();
            let name = info[0];
            let weight:u32 = info[1][1..(info[1].len()-1)].parse().unwrap();
            let refs = if pl.len() > 1 { pl[1].split(", ").collect() } else {Vec::new()};
            (name, weight, refs)
        }).collect();
    let mut weight:HashMap<&str, u32> = HashMap::new();
    let mut data:HashMap<&str, (u32, &Vec<&str>)> = HashMap::new();
    for dat in list.iter(){
        data.entry(dat.0).or_insert((dat.1, &dat.2));
    }
    for dat in list.iter(){
        weight_find(dat.0, &&mut data, &mut weight);
    }
    let mut root = "gynfwly"; //teehee
    let mut prevd = 0u32;
    loop{
        let l = data.get(root).unwrap().1;
        if l.len() == 0 {break;}
        let w1 = *weight.get(l[0]).unwrap();
        if l.iter().all(|c| *weight.get(c).unwrap()==w1) {break};
        let mut w = 0;
        let mut c = 0;
        let mut x = l[0];
        let mut other = l[0];
        for i in l.iter(){
            if *weight.get(i).unwrap() == w1 {
                w+=1;
            }
            else{
                c+= 1;
                other = i;
            }
        }
        if w > c {
            x = other;
            prevd = data.get(other).unwrap().0 + w1-weight.get(other).unwrap();
        }else{
            prevd = data.get(x).unwrap().0 + weight.get(l[1]).unwrap()-w1;
        }

        
        root = x;
    }
    prevd
}

fn weight_find<'a>(name:&'a str, mapping: &&mut HashMap<&'a str, (u32, &'a Vec<&'a str>)>, weights: &mut HashMap<&'a str, u32>) -> u32{
    if weights.contains_key(name) {
        *weights.get(name).unwrap()
    }
    else{
        let dat = mapping.get(name).unwrap();
        let r:u32 = dat.1.iter().map(|nm| weight_find(*nm, mapping, weights)).sum();
        *weights.entry(name).or_insert(dat.0 + r)
    }
}