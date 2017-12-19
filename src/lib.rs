#![allow(dead_code)]
#![allow(unused_imports)]
extern crate disjoint_sets;
mod inputs;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

#[cfg(test)]
mod tests{
    use inputs::*;
    use day01::*;
    use day02::*;
    use day03::*;
    use day04::*;
    use day05::*;
    use day06::*;
    use day07::*;
    use day08::*;
    use day09::*;
    use day10::*;
    use day11::*;
    use day12::*;
    use day13::*;
    use day14::*;
    use day15::*;
    use day16::*;
    use day17::*;
    use day18::*;
    use day19::*;

    #[test]
    fn test_day01_1(){
        println!("Day 01 (1): {}", day01_1(DAY1.to_string()))
    }
    #[test]
    fn test_day01_2(){
        println!("Day 01 (2): {}", day01_2(DAY1.to_string()))
    }
    #[test]
    fn test_day02_1(){
        println!("Day 02 (1): {}", day02_1(DAY2.to_string()))
    }
    #[test]
    fn test_day02_2(){
        println!("Day 02 (2): {}", day02_2(DAY2.to_string()))
    }
    #[test]
    fn test_day03_1(){
        //assert_eq!(0, day03_1(1));
        assert_eq!(3, day03_1(12));
        assert_eq!(2, day03_1(23));
        assert_eq!(31, day03_1(1024));
        assert_eq!(3, day03_1(10));
        assert_eq!(2, day03_1(11));
        assert_eq!(4, day03_1(13));
        assert_eq!(3, day03_1(24));
        assert_eq!(3, day03_1(22));
        assert_eq!(2, day03_1(23));
        assert_eq!(4, day03_1(21));
        assert_eq!(3, day03_1(20));
        assert_eq!(3, day03_1(18));
        println!("Day 03 (1): {}", day03_1(DAY3.to_string().parse::<u32>().unwrap()))
    }
    #[test]
    fn test_day03_2(){
        assert_eq!(57, day03_2(12));
        assert_eq!(1, day03_2(2));
        assert_eq!(2, day03_2(3));
        assert_eq!(4, day03_2(4));
        assert_eq!(5, day03_2(5));
        println!("Day 03 (1): {}", day03_2(DAY3.to_string().parse::<u32>().unwrap()))
    }
    #[test]
    fn test_day04_1(){
        println!("Day 04 (1): {}", day04_1(DAY4.to_string()))
    }
    #[test]
    fn test_day04_2(){
        println!("Day 04 (2): {}", day04_2(DAY4.to_string()))
    }
    #[test]
    fn test_day05_1(){
        println!("Day 05 (1): {}", day05_1(DAY5.to_string()))
    }
    #[test]
    fn test_day05_2(){
        println!("Day 05 (2): {}", day05_2(DAY5.to_string()))
    }
    #[test]
    fn test_day06_1(){
        assert_eq!(5, day06_1("0\t2\t7\t0".to_string()));
        println!("Day 06 (1): {}", day06_1(DAY6.to_string()))
    }
    #[test]
    fn test_day06_2(){
        assert_eq!(4, day06_2("0\t2\t7\t0".to_string()));
        println!("Day 06 (2): {}", day06_2(DAY6.to_string()))
    }
    #[test]
    fn test_day07_1(){
        println!("Day 07 (1): {}", day07_1(DAY7.to_string()))
    }
    #[test]
    fn test_day07_2(){
        println!("Day 07 (2): {}", day07_2(DAY7.to_string()))
    }
    #[test]
    fn test_day08_1(){
        println!("Day 08 (1): {}", day08_1(DAY8.to_string()))
    }
    #[test]
    fn test_day08_2(){
        println!("Day 08 (2): {}", day08_2(DAY8.to_string()))
    }
    #[test]
    fn test_day09_1(){
        println!("Day 09 (1): {}", day09_1(DAY9.to_string()))
    }
    #[test]
    fn test_day09_2(){
        assert_eq!(0, day09_2("<!!!>>".to_string()));
        assert_eq!(3, day09_2("<<<<>".to_string()));
        assert_eq!(10, day09_2("<{o\"i!a,<{i<a>".to_string()));
        println!("Day 09 (2): {}", day09_2(DAY9.to_string()))
    }
    #[test]
    fn test_day10_1(){
        assert_eq!(12, day10_1("3,4,1,5".to_string(), 5));
        println!("Day 10 (1): {}", day10_1(DAY10.to_string(), 256))
    }
    #[test]
    fn test_day10_2(){
        println!("Day 10 (2): {}", day10_2(DAY10.to_string()))
    }
    #[test]
    fn test_day11_1(){
        assert_eq!(3, day11_1("se,sw,se,sw,sw".to_string()));
        println!("Day 11 (1): {}", day11_1(DAY11.to_string()))
    }
    #[test]
    fn test_day11_2(){
        println!("Day 11 (2): {}", day11_2(DAY11.to_string()))
    }
    #[test]
    fn test_day12_1(){
        println!("Day 12 (1): {}", day12_1(DAY12.to_string()))
    }
    #[test]
    fn test_day12_2(){
        println!("Day 12 (2): {}", day12_2(DAY12.to_string()))
    }
    #[test]
    fn test_day13_1(){
        assert_eq!(24, day13_1("0: 3
1: 2
4: 4
6: 4".to_string()));
        println!("Day 13 (1): {}", day13_1(DAY13.to_string()))
    }
    #[test]
    fn test_day13_2(){
        assert_eq!(10, day13_2("0: 3
1: 2
4: 4
6: 4".to_string()));
        println!("Day 13 (2): {}", day13_2(DAY13.to_string()))
    }
    #[test]
    fn test_day14_1(){
        println!("Day 14 (1): {}", day14_1(DAY14.to_string()))
    }
    #[test]
    fn test_day14_2(){
        assert_eq!(1242, day14_2("flqrgnkx".to_string()));
        println!("Day 14 (2): {}", day14_2(DAY14.to_string()))
    }
    #[test]
    fn test_day15_1(){
        println!("Day 15 (1): {}", day15_1(DAY15_A, DAY15_B))
    }
    #[test]
    fn test_day15_2(){
        println!("Day 15 (2): {}", day15_2(DAY15_A,DAY15_B))
    }
    #[test]
    fn test_day16_1(){
        println!("Day 16 (1): {}", day16_1(DAY16.to_string()))
    }
    #[test]
    fn test_day16_2(){
        println!("Day 16 (2): {}", day16_2(DAY16.to_string()))
    }
    #[test]
    fn test_day17_1(){
        println!("Day 17 (1): {}", day17_1(DAY17))
    }
    #[test]
    fn test_day17_2(){
        println!("Day 17 (2): {}", day17_2(DAY17))
    }
    #[test]
    fn test_day18_1(){
        assert_eq!(4, day18_1("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2".to_string()));
        println!("Day 18 (1): {}", day18_1(DAY18.to_string()))
    }
    #[test]
    fn test_day18_2(){
        println!("Day 18 (2): {}", day18_2(DAY18.to_string()))
    }
    #[test]
    fn test_day19_1(){
        println!("Day 19 (1): {}", day19_1(DAY19.to_string()))
    }
    #[test]
    fn test_day19_2(){
        println!("Day 19 (2): {}", day19_2(DAY19.to_string()))
    }
}