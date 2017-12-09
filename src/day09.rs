use std::collections::*;
pub fn day09_1(s : String) -> u32{
    let mut running_total = 0;
    let mut scope = 0;

    let mut in_garbage = false;
    let mut prev_cancel = false;
    for c in s.chars(){
        if in_garbage {
            if c == '>' && !prev_cancel {
                in_garbage = false;
                prev_cancel = false;
            }
            else if c == '!' && !prev_cancel {
                prev_cancel = true;
            }
            else {
                prev_cancel = false;
            }
        }
        else{
            if c == '{' {
                scope+=1;
                running_total+=scope;
            }
            else if c == '}' {
                scope -=1;
            }
            else if c == '<' {
                in_garbage = true;
            }
        }
    }
    running_total
}

pub fn day09_2(s: String) -> u32{
    let mut running_total = 0;

    let mut in_garbage = false;
    let mut prev_cancel = false;
    for c in s.chars(){
        if in_garbage {
            if c == '>' && !prev_cancel {
                in_garbage = false;
                prev_cancel = false;
            }
            else if c == '!' && !prev_cancel {
                prev_cancel = true;
            }
            else if !prev_cancel{
                running_total+=1;
            }
            else {
                prev_cancel = false;
            }
        }
        else{
            if c == '<' {
                in_garbage = true;
                prev_cancel = false;
            }
        }
    }
    running_total
}