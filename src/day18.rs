use std::collections::*;
use std::time::*;
use std::cell::*;
use std::rc::Rc;

pub fn day18_1(s: String) -> u32 {
    let instrs:Vec<Vec<_>> = s.split('\n').map(|v| {
        v.split(' ').collect()
    }).collect();
    let instrs_len = instrs.len() as i64;
    let mut regs = HashMap::new();
    let mut idx:i64 = 0;
    let mut sent_freq = 0;
    while idx >= 0 && idx < instrs_len{
        let instr = &instrs[idx as usize];
        //println!("{:?}", instr);
        match instr[0]{
            "snd" => sent_freq = parse(instr[1], &mut regs),
            "set" => *regs.entry(instr[1]).or_insert(0) = parse(instr[2], &mut regs),
            "add" => *regs.entry(instr[1]).or_insert(0) += parse(instr[2], &mut regs),
            "mul" => *regs.entry(instr[1]).or_insert(0) *= parse(instr[2], &mut regs),
            "mod" => *regs.entry(instr[1]).or_insert(0) %= parse(instr[2], &mut regs),
            "jgz" => 
                if *regs.entry(instr[1]).or_insert(0) > 0 
                    { idx += parse(instr[2], &mut regs) - 1; },
            "rcv" => 
                if *regs.entry(instr[1]).or_insert(0) > 0 
                    { break; },
            _ => panic!()
        }
        idx+=1;
    }
    sent_freq as u32
}
fn parse<'a>(s:&'a str, map:&mut HashMap<&'a str, i64>) -> i64{
    let int = s.parse::<i64>();
    match int{
        Result::Ok(x) => x,
        Result::Err(_) => *map.entry(s).or_insert(0)
    }
}

struct Program<'a>{
    id: u32,
    registers: Rc<RefCell<HashMap<&'a str, i64>>>,
    msg_queue: Rc<RefCell<VecDeque<i64>>>,
    instr_idx: i64,
    sent_count: u32,
    cannot_run: bool
}

impl<'a> Program<'a>{
    fn new<'b>(id: u32) -> Program<'b>{
        let registers = Rc::new(RefCell::new(HashMap::new()));
        registers.borrow_mut().insert("p", id as i64);
        Program{
            id,
            registers,
            msg_queue: Rc::new(RefCell::new(VecDeque::new())),
            instr_idx: 0,
            sent_count: 0,
            cannot_run: false,
        }
    }

    fn get_value(&mut self, s: &'a str) -> i64{
        let int = s.parse::<i64>();
        match int{
            Result::Ok(x) => x,
            Result::Err(_) => *self.registers.borrow_mut().entry(s).or_insert(0)
        }
    }

    fn message(&self) -> Option<i64>{
        let mut msg_queue = self.msg_queue.borrow_mut();
        if let Some(f) = msg_queue.pop_front() {
            Some(f)
        }
        else {
            None
        }
    }

    fn empty_queue(&self) -> bool{
        self.msg_queue.borrow().is_empty()
    }
}

struct Computer<'a>{
    programs: HashMap<u32, Rc<RefCell<Program<'a>>>>,
    current_running: u32
}

impl<'a> Computer<'a>{
    fn new<'b>() -> Computer<'b>{
        Computer{
            programs: [0,1].iter().map(|&v| (v, Rc::new(RefCell::new(Program::new(v))))).collect(),
            current_running: 0
        }
    }

    fn broadcast(&self, v:i64){
        let otheridx = if self.current_running == 0 {1} else {0};
        let otherprogram = self.programs.get(&otheridx).unwrap().borrow_mut();
        let mut msg_queue = otherprogram.msg_queue.borrow_mut();
        msg_queue.push_back(v);
    }

    fn run_all(&mut self, instrs: Vec<Instruction<'a>>){
        let instr_len = instrs.len() as i64;
        
        while !self.programs.values().all(|v| v.borrow().cannot_run) && !self.programs.values().all(|v| v.borrow().empty_queue())  {
            let mut program = self.programs.get(&self.current_running).unwrap().borrow_mut();


            if program.instr_idx < 0 || program.instr_idx >= instr_len{
                program.cannot_run = true;
            }
            else{
                match instrs[program.instr_idx as usize]{
                    Instruction::Send(x) => { self.broadcast(program.get_value(x)); program.sent_count+=1},
                    Instruction::Set(a,b) => *program.registers.borrow_mut().entry(a).or_insert(0) = program.get_value(b),
                    Instruction::Add(a,b) => *program.registers.borrow_mut().entry(a).or_insert(0) += program.get_value(b),
                    Instruction::Mul(a,b) => *program.registers.borrow_mut().entry(a).or_insert(0) *= program.get_value(b),
                    Instruction::Mod(a,b) => *program.registers.borrow_mut().entry(a).or_insert(0) %= program.get_value(b),
                    Instruction::Jgz(a,b) => if program.get_value(a) > 0 { program.instr_idx += program.get_value(b)-1 },
                    Instruction::Rcv(x) => {
                        match program.message(){
                            None        =>   { program.instr_idx -= 1; program.cannot_run = true },
                            Some(val)   =>   {*program.registers.borrow_mut().entry(x).or_insert(0) = val; program.cannot_run=false;}
                        }

                    }                        
                }
                program.instr_idx += 1;
            }
            if program.cannot_run {
                self.current_running = if self.current_running == 0 {1} else {0};
                continue;
            }
        }
    }
}

enum Instruction<'a>{
    Send(&'a str),
    Set(&'a str, &'a str),
    Add(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Mod(&'a str, &'a str),
    Jgz(&'a str, &'a str),
    Rcv(&'a str),
}

impl<'a> Instruction<'a>{
    fn parse<'b>(v: &'b str) -> Instruction<'b>{
        let instr:Vec<_> = v.split(' ').collect();
        match instr[0]{
            "snd" => Instruction::Send(instr[1]),
            "set" => Instruction::Set(instr[1], instr[2]),
            "add" => Instruction::Add(instr[1], instr[2]),
            "mul" => Instruction::Mul(instr[1], instr[2]),
            "mod" => Instruction::Mod(instr[1], instr[2]),
            "jgz" => Instruction::Jgz(instr[1], instr[2]),
            "rcv" => Instruction::Rcv(instr[1]),
            _ => panic!()
        }
    }
}

pub fn day18_2(s: String) -> u32 {
    let mut system = Computer::new();
    let instrs:Vec<_> = s.split('\n').map(|v| {
        Instruction::parse(v)
    }).collect();
    system.run_all(instrs);
    let program1 = system.programs.get(&1).unwrap().borrow_mut();
    program1.sent_count
}
