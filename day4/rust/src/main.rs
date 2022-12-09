use std::fs;

fn get_input() -> String {
    fs::read_to_string("input.txt").expect("Cannot get the input file")
}

fn main() {
    let input = get_input();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut counter = 0;
    for line in input.lines() {
        let items: Vec<i32> = line
            .split(&[',', '-'])
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        assert!(items.len() == 4);
        if (items[0] <= items[2] && items[3] <= items[1])
            || (items[2] <= items[0] && items[1] <= items[3])
        {
            counter += 1;
        }
    }
    println!("Part 1: {}", counter);
}

fn part_2(input: &str) {
    let mut counter = 0;
    for line in input.lines() {
        let items: Vec<i32> = line
            .split(&[',', '-'])
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        assert!(items.len() == 4);
        if (items[2] <= items[0] && items[0] <= items[3])
            || (items[0] <= items[2] && items[2] <= items[1])
        {
            counter += 1;
        }
    }
    println!("Part 2: {}", counter);
}
