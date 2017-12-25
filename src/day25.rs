use std::collections::*;
use std::cell::*;
use std::rc::*;

#[derive(Debug)]
struct Instruction{
    write: bool,
    move_val: i64,
    next_state_name: String
}
#[derive(Debug)]
struct State{
    instr0: Instruction,
    instr1: Instruction,
}


impl State{
    fn parse(s: &str) -> (String, State){
        let words:Vec<Vec<_>> = s.split('\n').map(|ln| ln.split(' ').collect()).collect();
        let state_name = last_but_without_extra(&words[0]);
        (state_name.to_string(), State{
            instr0: Instruction{
                write: last_but_without_extra(&words[2]).parse::<u32>().unwrap() == 1,
                move_val: to_move_val(last_but_without_extra(&words[3])),
                next_state_name: last_but_without_extra(&words[4]).to_string(),
            },
            instr1: Instruction{
                write: last_but_without_extra(&words[6]).parse::<u32>().unwrap() == 1,
                move_val: to_move_val(last_but_without_extra(&words[7])),
                next_state_name: last_but_without_extra(&words[8]).to_string(),
            }
        })
    }
}

pub fn day25_1(s: String) -> usize {
    let mut text = s.split("\n\n");
    let start_region:Vec<Vec<_>> = text.next().unwrap().split('\n').map(|v| v.split(' ').collect()).collect();
    let mut state_name = last_but_without_extra(&start_region[0]).to_string();
    let after = start_region[1][5].parse::<u32>().unwrap();
    let states:HashMap<String, State> = text.map(|t| State::parse(t)).collect();
    let mut tape:HashMap<i64, bool> = HashMap::new();
    let mut cur_pos = 0;
    for _ in 0..after{
        let mut cur_v = tape.entry(cur_pos).or_insert(false);
        let state = states.get(&state_name).unwrap();
        if *cur_v{
            *cur_v = state.instr1.write;
            cur_pos += state.instr1.move_val;
            state_name = state.instr1.next_state_name.clone();
        }
        else{
            *cur_v = state.instr0.write;
            cur_pos += state.instr0.move_val;
            state_name = state.instr0.next_state_name.clone();
        }
    }
    tape.values().filter(|&&v| v).count()
}

fn to_move_val(s:&str) -> i64{
    if s == "right"{
        1
    }
    else{-1}
}

fn last_but_without_extra<'a>(s: &'a Vec<&'a str>) -> &'a str{
    let h = s.len();
    except_last(s[h-1])
}

fn except_last(s: &str) ->&str{
    let h = s.len();
    &s[..(h-1)]
}

pub fn day25_2(_s: String) -> u32 {
    panic!("LMAO THERES NO PART 2");
}