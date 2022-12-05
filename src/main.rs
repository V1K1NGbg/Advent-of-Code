use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lines(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    Ok(reader)
}

fn main() {
    let lines = read_lines("src/rucksack.txt").expect("You need a str");
    let mut sum = 0;
    for line in lines.lines() {
        let line = line.expect("IDK");
        sum = sum + value(line);
    }
    println!("{}",sum)
}

fn value( line: String) -> i32 {
    let len = line.len();
    for x in len / 2..len - 1 {
        for y in 0..(len / 2) - 1 {
            if line.chars().nth(x).unwrap() == line.chars().nth(y).unwrap() {
                  println!("line:{}, x:{}, y:{}, charx:{}, chary:{}", line, x, y, line.chars().nth(x).unwrap(), line.chars().nth(y).unwrap());
                if line.chars().nth(x).unwrap().is_lowercase() {
                    return line.chars().nth(x).unwrap() as i32 - 96;
                } else {
                    return line.chars().nth(x).unwrap() as i32 - 38;
                }
            }
        }
    }
    return 0;
}
// jGrGqjJfqccrfqGcGplrJpFvzggqmCtMzmsMnvMvvCgm