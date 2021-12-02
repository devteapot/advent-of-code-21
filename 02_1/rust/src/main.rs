use std::fs;

fn main() {
    let input = fs::read_to_string("/Users/lino/dev/advent-of-code-21/02_1/rust/1.txt")
        .unwrap();

    let parsed_input = input
        .split('\n')
        .map(|line| {
            let mut words = line.splitn(2, ' ');

            (words.next().unwrap(), words.next().unwrap().parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    let result = parsed_input
        .iter()
        .fold((0 ,0), | acc, (direction, offset) | match *direction {
            "forward" => (acc.0 + *offset, acc.1),
            "down" => (acc.0, acc.1 + *offset),
            "up" => (acc.0, acc.1 - *offset),
            _ => acc,
        });

    println!("{}", result.0 * result.1);
}
