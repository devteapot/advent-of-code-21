use std::{fs};

fn u8_vec_to_number(vec: &Vec<u8>) -> i32 {
    vec
        .iter()
        .rev()
        .enumerate()
        .map(|(index, value)| 2_i32.pow(index as u32) * *value as i32)
        .sum()
}

fn main() {
    let lines = fs::read_to_string("/Users/lino/dev/advent-of-code-21/03/1.txt")
        .unwrap();

    let split_lines: Vec<&str> = lines
        .split('\n')
        .collect();

    let rows = split_lines.len();
    let cols = split_lines[0].len();

    let mut result_vec = vec![0; cols];

    split_lines
        .iter()
        .for_each(|x| x
            .chars()
            .enumerate()
            .for_each(|(index, c)| {
                result_vec[index] = result_vec.get(index).unwrap() + c.to_digit(2).unwrap();
            })
        );
    
    let gamma: Vec<u8> = result_vec
        .iter()
        .map(|x| if x > &((rows / 2) as u32) {1} else {0})
        .collect();

    let epsilon: Vec<u8> = gamma
        .iter()
        .map(|x| if *x != 0 {0} else {1})
        .collect();

    println!("gamma: {:?}, epsiplon: {:?}", u8_vec_to_number(&gamma), u8_vec_to_number(&epsilon));
    println!("{}", u8_vec_to_number(&gamma) * u8_vec_to_number(&epsilon));
}
