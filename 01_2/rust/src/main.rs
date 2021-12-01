use std::fs;
use itertools::Itertools;

fn main() {
    let data = fs::read_to_string("/Users/lino/dev/advent-of-code/01_2/rust/1.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = data
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold (0, |acc, (fst, snd)|  acc + if fst < snd { 1 } else { 0 });

    println!("{}", result)
}
