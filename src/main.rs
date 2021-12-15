use std::env;
use std::fs;

mod days;
use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_all(),
        2 => {
            let day = args[1]
                .parse::<usize>()
                .expect("Something went wrong parsing day number");
            run_day(day);
        }
        n => println!("Unexpected number of arguments: {}", n),
    }
}

fn run_day(day: usize) {
    let day_funcs: Vec<fn(&str) -> u64> = vec![
        day_1_1, day_1_2, day_2_1, day_2_2, day_3_1, day_3_2, day_4_1, day_4_2, day_5_1, day_5_2,
        day_6_1, day_6_2, day_7_1, day_7_2, day_8_1, day_8_2, day_9_1, day_9_2, day_10_1, day_10_2,
        day_11_1, day_11_2, day_12_1, day_12_2, day_13_1, day_13_2, day_14_1, day_14_2, day_15_1,
        day_15_2, day_16_1, day_16_2, day_17_1, day_17_2, day_18_1, day_18_2,
    ];
    let input = fs::read_to_string(&format!("inputs/day_{:02}.txt", day));
    if input.is_err() {
        return;
    }
    let input = input.unwrap();

    match day {
        1..=25 => {
            if day_funcs.len() >= day * 2 - 1 {
                let ans_1 = day_funcs[(day - 1) * 2](&input);
                println!("day {}_1:\n{}", day, ans_1);
            }
            if day_funcs.len() >= day * 2 {
                let ans_2 = day_funcs[(day - 1) * 2 + 1](&input);
                println!("day {}_2:\n{}", day, ans_2);
            }
        }
        _ => println!("Day {} out of bounds", day),
    }
}

fn run_all() {
    println!("running all");
    for day in 1..=25 {
        run_day(day);
    }
}

#[cfg(test)]
mod tests {
    use super::days::*;
    #[test]
    fn test_day_1() {
        let input = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";
        let ans_1 = day_1_1(input);
        assert_eq!(ans_1, 7);

        let ans_2 = day_1_2(input);
        assert_eq!(ans_2, 5);
    }

    #[test]
    fn test_day_2() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let ans_1 = day_2_1(input);
        assert_eq!(ans_1, 150);

        let ans_2 = day_2_2(input);
        assert_eq!(ans_2, 900);
    }

    #[test]
    fn test_day_3() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

        let ans_1 = day_3_1(input);
        assert_eq!(ans_1, 198);

        let ans_2 = day_3_2(input);
        assert_eq!(ans_2, 230);
    }

    #[test]
    fn test_day_4() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";

        let ans_1 = day_4_1(input);
        assert_eq!(ans_1, 4512);

        let ans_2 = day_4_2(input);
        assert_eq!(ans_2, 1924);
    }

    #[test]
    fn test_day_5() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let ans_1 = day_5_1(input);
        assert_eq!(ans_1, 5);

        let ans_2 = day_5_2(input);
        assert_eq!(ans_2, 12);
    }

    #[test]
    fn test_day_6() {
        let input = "3,4,3,1,2";

        let ans_1 = day_6_1(input);
        assert_eq!(ans_1, 5934);

        let ans_2 = day_6_2(input);
        assert_eq!(ans_2, 26984457539);
    }

    #[test]
    fn test_day_7() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let ans_1 = day_7_1(input);
        assert_eq!(ans_1, 37);

        let ans_2 = day_7_2(input);
        assert_eq!(ans_2, 168);
    }

    #[test]
    fn test_day_8() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let ans_1 = day_8_1(input);
        assert_eq!(ans_1, 26);

        let ans_2 = day_8_2(input);
        assert_eq!(ans_2, 61229);
    }

    #[test]
    fn test_day_9() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

        let ans_1 = day_9_1(input);
        assert_eq!(ans_1, 15);

        let ans_2 = day_9_2(input);
        assert_eq!(ans_2, 1134);
    }

    #[test]
    fn test_day_10() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let ans_1 = day_10_1(input);
        assert_eq!(ans_1, 26397);

        let ans_2 = day_10_2(input);
        assert_eq!(ans_2, 288957);
    }

    #[test]
    fn test_day_11() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";

        let ans_1 = day_11_1(input);
        assert_eq!(ans_1, 1656);

        let ans_2 = day_11_2(input);
        assert_eq!(ans_2, 195);
    }

    #[test]
    fn test_day_12() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";

        let ans_1 = day_12_1(input);
        assert_eq!(ans_1, 10);

        let ans_2 = day_12_2(input);
        assert_eq!(ans_2, 36);
    }

    #[test]
    fn test_day_13() {
        let input = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5";

        let ans_1 = day_13_1(input);
        assert_eq!(ans_1, 17);

        let ans_2 = day_13_2(input);
        assert_eq!(ans_2, 0);
    }

    #[test]
    fn test_day_14() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let ans_1 = day_14_1(input);
        assert_eq!(ans_1, 1588);

        // let ans_2 = day_14_2(input);
        // assert_eq!(ans_2, 13);
    }
}
