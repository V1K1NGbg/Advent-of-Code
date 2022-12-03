use std::{io::{self, BufRead, BufReader}, fs::File};

fn read_lines(path: &str) -> io::Result<BufReader<File>> {
    let file =  File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    Ok(reader)
}

fn main() {
    let lines = read_lines("src/strat.txt").expect("You need a str");

    let mut score = 0;
    let mut mine = 0;

    for line in lines.lines(){
        let line = line.expect("IDK");
        let his = line.chars().nth(0).unwrap() as i32 - 64;
        let result = line.chars().nth(2).unwrap() as i32 - 87;
        match result{
            1=>if his == 1 {mine = 3} else {mine = his - 1},
            2=>mine = his,
            3=>if his == 3 {mine = 1} else {mine = his + 1},
            _=>print!(""),
        }
        score = score + mine + (result*3) - 3;
        println!("mine:{}, his:{}, result:{}, score:{}",mine,his,result,score);
    }

    println!("{}",score);
}
