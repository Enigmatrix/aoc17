mod inputs;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

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
}