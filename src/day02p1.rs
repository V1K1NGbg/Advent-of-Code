use std::{io::{self, BufRead, BufReader}, fs::File};

fn read_lines(path: &str) -> io::Result<BufReader<File>> {
    let file =  File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    Ok(reader)
}
fn main() {
    let lines = read_lines("src/strat.txt").expect("You need a str");

    let mut score = 0;

    for line in lines.lines(){
        let line = line.expect("IDK");
        let his = line.chars().nth(0).unwrap() as i32 - 64;
        let mine = line.chars().nth(2).unwrap() as i32 - 87;
        if ((mine > his) || (his == 3 && mine == 1)) && !(mine == 3 && his == 1){
            score = score + 6 + mine;
            println!("mine: {}, his: {}, score: {}, W",mine,his,score);
        } else if mine == his {
            score = score + 3 + mine;
            println!("mine: {}, his: {}, score: {}, E",mine,his,score);
        } else {
            score = score + mine;
            println!("mine: {}, his: {}, score: {}, L",mine,his,score);
        }
    }

    println!("{}",score);
}
