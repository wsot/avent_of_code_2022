use std::fs;

const INCLUSION_THRESHOLD: i32 = 100000;

fn get_input() -> String {
    fs::read_to_string("input.txt").expect("Cannot get the input file")
}

fn main() {
    let input = get_input();
    part_1(&input);
}

fn part_1(input: &String) {
    let mut cumulator = 0;
    let mut size = Vec::<i32>::with_capacity(40);
    size.push(0);

    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            let dir_size = size.pop().unwrap();
            if dir_size <= INCLUSION_THRESHOLD {
                cumulator += dir_size;
            }

            let last = size.last_mut().unwrap();
            *last += dir_size;
        } else if line.starts_with("$ cd ") {
            size.push(0);
        } else if line.starts_with("dir") {
            continue;
        } else if line.starts_with("$") {
            continue;
        } else if line == "" {
            continue;
        } else {
            let last = size.last_mut().unwrap();
            *last += line.split_once(" ").unwrap().0.parse::<i32>().unwrap();
        }
    }
    println!("{}", cumulator);
}
