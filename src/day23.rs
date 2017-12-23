use std::collections::*;
use std::time::*;

#[derive(Debug)]
enum Instruction<'a>{
    Set(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Jnz(&'a str, &'a str),
}

impl<'a> Instruction<'a>{
    fn parse<'b>(v: &'b str) -> Instruction<'b>{
        let instr:Vec<_> = v.split(' ').collect();
        match instr[0]{
            "set" => Instruction::Set(instr[1], instr[2]),
            "sub" => Instruction::Sub(instr[1], instr[2]),
            "mul" => Instruction::Mul(instr[1], instr[2]),
            "jnz" => Instruction::Jnz(instr[1], instr[2]),
            _ => panic!()
        }
    }
}

pub fn day23_1(s: String) -> u32 {
    let v:Vec<_> = s.split('\n').map(|v| Instruction::parse(v)).collect();
    let ln = v.len() as i64;
    let mut cur = 0i64;
    let mut regs = HashMap::new();
    let mut k = 0;
    while 0 <= cur && cur < ln{
        match v[cur as usize]{
            Instruction::Set(x,y) => *regs.entry(x).or_insert(0) = parse(y, &mut regs),
            Instruction::Sub(x,y) => *regs.entry(x).or_insert(0) -= parse(y, &mut regs),
            Instruction::Mul(x,y) => {*regs.entry(x).or_insert(0) *= parse(y, &mut regs); k+=1;},
            Instruction::Jnz(x,y) => if parse(x, &mut regs) != 0 {
                cur += parse(y, &mut regs) -1;
            },
        }
        cur +=1
    }
    k
}

fn parse<'a>(s:&'a str, map:&mut HashMap<&'a str, i64>) -> i64{
    let int = s.parse::<i64>();
    match int{
        Result::Ok(x) => x,
        Result::Err(_) => *map.entry(s).or_insert(0)
    }
}
fn sqrt(n: u32) -> f32 {

  let a: f32 = n as f32;// = float::from_str(int::str(n));
  let mut x: f32 = 1.0;
  let mut i = 0;

  while i < n {
    x = 0.5 * ( x + a / x );
    i += 1;
  }

  x
}


fn is_prime(n: u32) -> bool {

  let sroot: u32 = sqrt(n) as u32;
  let mut i = 3;
  let mut rbool = true;

  if n == 1 || (n > 2 && n % 2 == 0) {
    false
  }
  else {

    while  i <= sroot  {
      if n % i == 0 {
        rbool = false;
        break;
      }
      i += 2;
    }

    rbool
  }
}
pub fn day23_2() -> usize {
    (0..1001).map(|v| 106500+17*v).filter(|&v| !is_prime(v)).count()
}
