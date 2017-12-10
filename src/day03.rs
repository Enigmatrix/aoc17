pub fn day03_1(s : u32) -> u32{
    let lvl = ((1i32..).find(|x| x*x - s as i32 >= 0).unwrap())/2;
    let lvlb4max = (1+(lvl-1)*2)*(1+(lvl-1)*2)+1;
    let diff = s as i32 - lvlb4max;
    let y = ((diff % (lvl*2)) - (lvl-1)).abs();
    (lvl + y) as u32
}

//bruteforce :<
pub fn day03_2(s: u32) -> u32{
    /*let mut state:Vec<u32> = vec![1,1,2,4,5,10,11,23,25];
    let mut side = 4;
    let mut sidew = 0;
    let mut idx:u32 = 10;
    let mut lvl = 2;

    while *state.last().unwrap() <= s {
        let mside = side - 1;
        let v = *state.last().unwrap() + match sidew{
            0 => state[(side+1)*(side+1)],
            mside => 2,
            side => 1,
            _ => 4
        };

        state.push(v);
        if((idx as f64).sqrt().trunc() == (idx as f64).sqrt()){
            sidew = 0;
            side += 2;
            lvl+=1;
        }
        idx+=1;
    }
    *state.last().unwrap() as u32*/
    s
}