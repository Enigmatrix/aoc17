use std::collections::*;
pub fn day08_1(s : String) -> i32{
    let instrs = s.split('\n').map(|line| {
        let dat:Vec<_> = line.split(" ").collect();
        let sign:i32 = if dat[1] == "dec" { -1 } else { 1 };
        (dat[0], sign * (dat[2].parse::<i32>().unwrap()), dat[4], dat[5], dat[6])
    });

    let mut regs:HashMap<&str, i32> = HashMap::new();
    for instr in instrs{
        let (reg, change, cmp1, cmpop, cmp2) = instr;
        let (a,b) = (parse(cmp1, &mut regs), parse(cmp2, &mut regs));
        let cmpresult = match cmpop {
            "==" =>  a==b,
            "!=" =>  a!=b,
            ">" =>  a>b,
            "<" =>  a<b,
            ">=" =>  a>=b,
            "<=" =>  a<=b,
            _ => panic!()
        };
        if cmpresult {
            *regs.entry(reg).or_insert(0) += change;
        }
    }
    *regs.values().max().unwrap()
}

fn parse<'a>(s:&'a str, map:&mut HashMap<&'a str, i32>) -> i32{
    let int = s.parse::<i32>();
    match int{
        Result::Ok(x) => x,
        Result::Err(_) => *map.entry(s).or_insert(0)
    }
}

pub fn day08_2(s: String) -> i32{
    let instrs = s.split('\n').map(|line| {
        let dat:Vec<_> = line.split(" ").collect();
        let sign:i32 = if dat[1] == "dec" { -1 } else { 1 };
        (dat[0], sign * (dat[2].parse::<i32>().unwrap()), dat[4], dat[5], dat[6])
    });

    let mut regs:HashMap<&str, i32> = HashMap::new();
    let mut max = <i32>::min_value();

    for instr in instrs{
        let (reg, change, cmp1, cmpop, cmp2) = instr;
        let (a,b) = (parse(cmp1, &mut regs), parse(cmp2, &mut regs));
        let cmpresult = match cmpop {
            "==" =>  a==b,
            "!=" =>  a!=b,
            ">" =>  a>b,
            "<" =>  a<b,
            ">=" =>  a>=b,
            "<=" =>  a<=b,
            _ => panic!()
        };
        if cmpresult {
            let regv = regs.entry(reg).or_insert(0);
            *regv += change;
            max = if *regv > max { *regv } else { max };
        }
    }
    max
}